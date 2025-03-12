use std::{fmt, time::Duration};

use anyhow::{anyhow, bail};
use axum::{
    body::Body,
    http::{header::AUTHORIZATION, Request, Response, StatusCode},
    response::IntoResponse,
};
use futures::future::{BoxFuture, FutureExt};
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use worker::Date;

use crate::{
    config,
    keys::{ApiChallengeNonce, ClientNonceSignature},
    models::{ChallengeId, ClientKeyId, FormId, ServerKeyId},
    store::{Store, UnauthenticatedStore},
};

//
// See the security architecture document for an overview of how the auth flow works. The names of
// identifiers in this file generally match the terms defined in that document.
//
// https://github.com/justlark/notwithout.help/blob/main/docs/security-whitepaper.md
//

const BEARER_PREFIX: &str = "Bearer ";

const JWT_ALGORITHM: jwt::Algorithm = jwt::Algorithm::HS256;

fn unix_timestamp() -> u64 {
    let datetime = Date::now();
    Duration::from_millis(datetime.as_millis()).as_secs()
}

fn new_jwt_validation() -> jwt::Validation {
    let mut validation = jwt::Validation::new(JWT_ALGORITHM);

    validation.required_spec_claims = ["exp", "sub", "aud", "iss"]
        .iter()
        .map(|claim| claim.to_string())
        .collect();
    validation.aud = Some(config::api_allowed_origins().into_iter().collect());
    validation.iss = Some(config::api_allowed_origins().into_iter().collect());
    validation.algorithms = vec![JWT_ALGORITHM];

    validation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthErrorType {
    Unauthorized,
    Forbidden,
}

#[derive(Debug, Clone)]
pub struct AuthError {
    kind: AuthErrorType,
    message: String,
}

// Not to be confused with the roles within in an organization that form respondents can choose
// from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
}

impl AccessRole {
    // Whether the permissions granted by this role include the permissions granted by `other`.
    pub fn includes(self, other: Self) -> bool {
        match self {
            Self::Admin => true,
            Self::Read => other == Self::Read,
        }
    }
}

impl std::error::Error for AuthError {}

impl AuthError {
    pub fn kind(&self) -> AuthErrorType {
        self.kind
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            kind: AuthErrorType::Unauthorized,
            message: message.into(),
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            kind: AuthErrorType::Forbidden,
            message: message.into(),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ApiTokenType {
    Access,
    Challenge,
}

// The JWT `sub` claim has the format `<form_id>/<client_key_id>`.
#[derive(Debug, Clone)]
struct ApiTokenJwtSub {
    form_id: FormId,
    client_key_id: ClientKeyId,
}

impl ApiTokenJwtSub {
    const SEPARATOR: char = '/';
}

impl Serialize for ApiTokenJwtSub {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}{}{}", self.form_id, Self::SEPARATOR, self.client_key_id).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ApiTokenJwtSub {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts = s.splitn(2, Self::SEPARATOR).collect::<Vec<_>>();

        Ok(match parts.as_slice() {
            [form_id, client_key_id] => Self {
                form_id: form_id.to_string().into(),
                client_key_id: client_key_id.parse().map_err(serde::de::Error::custom)?,
            },
            _ => {
                return Err(serde::de::Error::custom(
                    "JWT `sub` claim is not in the expected format.",
                ))
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiAccessTokenClaims {
    #[serde(rename = "type")]
    token_type: ApiTokenType,
    role: AccessRole,
    sub: ApiTokenJwtSub,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct SignedApiAccessToken(String);

impl SignedApiAccessToken {
    pub async fn validate<'a>(
        self,
        store: &'a UnauthenticatedStore,
        form_id: &'a FormId,
        needs_role: AccessRole,
    ) -> Result<&'a Store, AuthError> {
        self.validate_with(store, form_id, |_, role| {
            if role.includes(needs_role) {
                Ok(())
            } else {
                Err(AuthError::forbidden(
                    "This access token does not have the required permissions.",
                ))
            }
        })
        .await
    }

    pub async fn validate_with<'a>(
        self,
        store: &'a UnauthenticatedStore,
        form_id: &'a FormId,
        role_validator: impl Fn(ClientKeyId, AccessRole) -> Result<(), AuthError>,
    ) -> Result<&'a Store, AuthError> {
        let store = store.without_authenticating();

        let header =
            jwt::decode_header(&self.0).map_err(|err| AuthError::unauthorized(err.to_string()))?;

        let server_key_id = header
            .kid
            .ok_or_else(|| AuthError::unauthorized("Access token is missing the `kid` claim."))?
            .parse()
            .map_err(|_| AuthError::unauthorized("Could not parse header key ID."))?;

        let ephemeral_server_key = store
            .get_ephemeral_server_key(&server_key_id)
            .await
            .map_err(|err| AuthError::unauthorized(err.to_string()))?
            .ok_or_else(|| {
                AuthError::unauthorized(
                    "Ephemeral server key for access token `kid` does not exist.",
                )
            })?;

        let token_claims = jwt::decode::<ApiAccessTokenClaims>(
            &self.0,
            &ephemeral_server_key.decoding_key(),
            &new_jwt_validation(),
        )
        .map_err(|err| AuthError::unauthorized(err.to_string()))?
        .claims;

        // If we don't do this check, there would be nothing stopping a user from authenticating
        // with a challenge token, since they're also signed by the ephemeral server key.
        if token_claims.token_type != ApiTokenType::Access {
            return Err(AuthError::unauthorized(
                "Attempted to use a challenge token as an access token.",
            ));
        }

        if &token_claims.sub.form_id != form_id {
            return Err(AuthError::forbidden(
                "Form ID in access token `sub` does not match the form being accessed.",
            ));
        }

        let client_keys = store
            .get_client_keys(&token_claims.sub.form_id, &token_claims.sub.client_key_id)
            .await
            .map_err(|err| AuthError::unauthorized(err.to_string()))?;

        match client_keys {
            Some(keys) => role_validator(keys.id, keys.role)?,
            None => {
                return Err(AuthError::unauthorized(
                    "Client key in access token `sub` does not exist or has been revoked.",
                ));
            }
        }

        store
            .log_access(form_id, &token_claims.sub.client_key_id)
            .await
            .map_err(|err| AuthError::unauthorized(err.to_string()))?;

        Ok(store)
    }
}

impl fmt::Display for SignedApiAccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ApiChallenge {
    pub server_key_id: ServerKeyId,
    pub form_id: FormId,
    pub client_key_id: ClientKeyId,
    pub challenge_id: ChallengeId,
    pub nonce: ApiChallengeNonce,
    pub origin: String,
    pub exp: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiChallengeClaims {
    #[serde(rename = "type")]
    token_type: ApiTokenType,
    sub: ApiTokenJwtSub,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
    jti: ChallengeId,
    nonce: ApiChallengeNonce,
}

impl ApiChallenge {
    pub fn encode(&self, key: &jwt::EncodingKey) -> anyhow::Result<SignedApiChallenge> {
        let mut header = jwt::Header::new(JWT_ALGORITHM);
        header.kid = Some(self.server_key_id.to_string());

        let secs_since_epoch = unix_timestamp();

        let claims = ApiChallengeClaims {
            token_type: ApiTokenType::Challenge,
            sub: ApiTokenJwtSub {
                form_id: self.form_id.clone(),
                client_key_id: self.client_key_id,
            },
            aud: self.origin.clone(),
            iss: self.origin.clone(),
            iat: secs_since_epoch,
            exp: secs_since_epoch + self.exp.as_secs(),
            jti: self.challenge_id.clone(),
            nonce: self.nonce.clone(),
        };

        Ok(SignedApiChallenge(jwt::encode(&header, &claims, key)?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignedApiChallenge(String);

impl SignedApiChallenge {
    async fn validate(&self, store: &Store) -> anyhow::Result<ValidatedApiChallenge> {
        let header = jwt::decode_header(&self.0)?;

        let server_key_id: ServerKeyId = header
            .kid
            .ok_or_else(|| anyhow!("Challenge token is missing the `kid` claim."))?
            .parse()?;

        let ephemeral_server_key = store
            .get_ephemeral_server_key(&server_key_id)
            .await?
            .ok_or_else(|| {
                anyhow!("Ephemeral server key for challenge token `kid` does not exist.")
            })?;

        let claims = jwt::decode::<ApiChallengeClaims>(
            &self.0,
            &ephemeral_server_key.decoding_key(),
            &new_jwt_validation(),
        )?
        .claims;

        if claims.token_type != ApiTokenType::Challenge {
            bail!("Attempted to use an access token as a challenge token.");
        }

        if !store.has_challenge_id(&claims.jti).await? {
            bail!("This challenge token has already been used.");
        }

        store.delete_challenge_id(&claims.jti).await?;

        Ok(ValidatedApiChallenge(ApiChallenge {
            server_key_id,
            form_id: claims.sub.form_id,
            client_key_id: claims.sub.client_key_id,
            challenge_id: claims.jti,
            nonce: claims.nonce,
            origin: claims.iss,
            exp: Duration::from_secs(claims.exp - claims.iat),
        }))
    }
}

impl fmt::Display for SignedApiChallenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ApiChallengeResponse {
    pub signature: ClientNonceSignature,
    pub challenge: SignedApiChallenge,
}

impl ApiChallengeResponse {
    pub async fn validate(&self, store: &Store) -> anyhow::Result<ValidatedApiChallengeResponse> {
        let challenge = self.challenge.validate(store).await?.0;

        let client_keys = store
            .get_client_keys(&challenge.form_id, &challenge.client_key_id)
            .await?;

        let client_keys = client_keys.ok_or_else(|| {
            anyhow!("Client key for this challenge does not exist or has been revoked.")
        })?;

        client_keys
            .public_signing_key
            .verify(&challenge.nonce, &self.signature)?;

        Ok(ValidatedApiChallengeResponse {
            challenge,
            role: client_keys.role,
        })
    }
}

//
// These wrappers are a defensive measure to ensure we don't accidentally generate an access token
// from anything but a valid API challenge response.
//

#[derive(Debug, Clone)]
pub struct ValidatedApiChallenge(ApiChallenge);

#[derive(Debug, Clone)]
pub struct ValidatedApiChallengeResponse {
    challenge: ApiChallenge,
    role: AccessRole,
}

impl ValidatedApiChallengeResponse {
    pub fn server_key_id(&self) -> ServerKeyId {
        self.challenge.server_key_id.clone()
    }

    pub fn into_access_token(
        self,
        key: &jwt::EncodingKey,
        exp: Duration,
    ) -> anyhow::Result<SignedApiAccessToken> {
        let challenge = self.challenge;

        let mut header = jwt::Header::new(JWT_ALGORITHM);
        header.kid = Some(challenge.server_key_id.to_string());

        let secs_since_epoch = unix_timestamp();

        let claims = ApiAccessTokenClaims {
            token_type: ApiTokenType::Access,
            role: self.role,
            sub: ApiTokenJwtSub {
                form_id: challenge.form_id.clone(),
                client_key_id: challenge.client_key_id,
            },
            aud: challenge.origin.clone(),
            iss: challenge.origin.clone(),
            iat: secs_since_epoch,
            exp: secs_since_epoch + exp.as_secs(),
        };

        Ok(SignedApiAccessToken(jwt::encode(&header, &claims, key)?))
    }
}

type BoxFutureResponseResult<'a> = BoxFuture<'a, Result<Request<Body>, Response<Body>>>;

// Extract the bearer token from the Authorization header and insert it into the request
// extensions.
pub fn auth_layer<'a>(
) -> AsyncRequireAuthorizationLayer<impl Fn(Request<Body>) -> BoxFutureResponseResult<'a> + Clone> {
    AsyncRequireAuthorizationLayer::new(|mut req: Request<Body>| {
        async move {
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            let auth_header_value = auth_header
                .to_str()
                .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

            let token = auth_header_value
                .strip_prefix(BEARER_PREFIX)
                .map(|token| SignedApiAccessToken(token.to_string()))
                .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

            req.extensions_mut().insert(token);

            Ok(req)
        }
        .boxed()
    })
}
