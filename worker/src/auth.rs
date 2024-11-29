use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Current system time is before the unix epoch!")
        .as_secs()
}

fn new_jwt_validation() -> jwt::Validation {
    let mut validation = jwt::Validation::new(JWT_ALGORITHM);

    validation.required_spec_claims = ["exp", "sub", "aud", "iss"]
        .iter()
        .map(|claim| claim.to_string())
        .collect();
    validation.aud = Some(config::origins().into_iter().map(String::from).collect());
    validation.iss = Some(config::origins().into_iter().map(String::from).collect());
    validation.algorithms = vec![JWT_ALGORITHM];

    validation
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
    sub: ApiTokenJwtSub,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Clone)]
pub struct SignedApiAccessToken(String);

impl SignedApiAccessToken {
    pub async fn validate(
        self,
        store: &UnauthenticatedStore,
        form_id: FormId,
    ) -> anyhow::Result<&Store> {
        let store = store.without_authenticating();

        let header = jwt::decode_header(&self.0)?;

        let server_key_id = header
            .kid
            .ok_or_else(|| anyhow::anyhow!("Access token is missing the `kid` claim."))?
            .into();

        let ephemeral_server_key = store
            .get_ephemeral_server_key(server_key_id)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("Ephemeral server key for access token `kid` does not exist.")
            })?;

        let token_claims = jwt::decode::<ApiAccessTokenClaims>(
            &self.0,
            &ephemeral_server_key.decoding_key(),
            &new_jwt_validation(),
        )?
        .claims;

        // If we don't do this check, there would be nothing stopping a user from authenticating
        // with a challenge token, since they're also signed by the ephemeral server key.
        if token_claims.token_type != ApiTokenType::Access {
            bail!("Attempted to use a challenge token as an access token.");
        }

        if token_claims.sub.form_id != form_id {
            bail!("Form ID in access token `sub` does not match the form being accessed.");
        }

        let client_keys = store
            .get_client_keys(token_claims.sub.form_id, token_claims.sub.client_key_id)
            .await?;

        if client_keys.is_none() {
            bail!("Client key in access token `sub` does not exist or has been revoked.");
        }

        Ok(store)
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
            .into();

        let ephemeral_server_key = store
            .get_ephemeral_server_key(server_key_id.clone())
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

        if !store.has_challenge_id(claims.jti.clone()).await? {
            bail!("This challenge token has already been used.");
        }

        store.delete_challenge_id(claims.jti.clone()).await?;

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

#[derive(Debug)]
pub struct ApiChallengeResponse {
    pub signature: ClientNonceSignature,
    pub challenge: SignedApiChallenge,
}

impl ApiChallengeResponse {
    pub async fn validate(&self, store: &Store) -> anyhow::Result<ValidatedApiChallenge> {
        let challenge = self.challenge.validate(store).await?.0;

        let client_keys = store
            .get_client_keys(challenge.form_id.clone(), challenge.client_key_id)
            .await?;

        let public_signing_key = client_keys
            .as_ref()
            .map(|keys| &keys.public_signing_key)
            .ok_or_else(|| {
                anyhow!("Public signing key for this challenge does not exist or has been revoked.")
            })?;

        public_signing_key.verify(&challenge.nonce, &self.signature)?;

        Ok(ValidatedApiChallenge(challenge))
    }
}

// This wrapper is a defensive measure to ensure we don't accidentally generate an access token
// from anything but a valid API challenge response.
#[derive(Debug, Clone)]
pub struct ValidatedApiChallenge(ApiChallenge);

impl ValidatedApiChallenge {
    pub fn into_access_token(
        self,
        key: &jwt::EncodingKey,
        exp: Duration,
    ) -> anyhow::Result<SignedApiAccessToken> {
        let challenge = self.0;

        let mut header = jwt::Header::new(JWT_ALGORITHM);
        header.kid = Some(challenge.server_key_id.to_string());

        let secs_since_epoch = unix_timestamp();

        let claims = ApiAccessTokenClaims {
            token_type: ApiTokenType::Access,
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

// Extract the bearer token from the Authorization header and insert it into the request
// extensions.
pub fn auth_layer<'a>() -> AsyncRequireAuthorizationLayer<
    impl Fn(Request<Body>) -> BoxFuture<'a, Result<Request<Body>, Response<Body>>> + Clone,
> {
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
