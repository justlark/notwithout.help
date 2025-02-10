import {
  deriveKeys,
  signApiChallengeNonce,
  type ApiAccessToken,
  type ApiChallengeNonce,
  type ApiChallengeToken,
  type ClientKeyId,
  type FormId,
  type PrivateSigningKey,
} from "@/crypto";
import { isDone, propagatesError, type Loadable } from "@/types";
import api, { ApiError, type AccessRole, type ApiErrorKind } from "@/api";
import { ref, readonly, watchEffect } from "vue";
import { decodeBase64 } from "@/encoding";
import { jwtDecode } from "jwt-decode";
import { useSecretLink } from "./useSecretLink";
import useSecretLinkKey from "./useSecretLinkKey";

const cacheKey = (formId: FormId, clientKeyId: ClientKeyId) => `token:${formId}/${clientKeyId}`;

const storeAccessToken = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  accessToken: ApiAccessToken,
) => {
  // We use the session storage instead of the local storage because there's no
  // need to persist the access token across sessions; the URL fragment
  // contains all the information necessary to request a new one, which only
  // takes a few seconds longer.
  sessionStorage.setItem(cacheKey(formId, clientKeyId), accessToken);
};

const loadStoredAccessToken = (
  formId: FormId,
  clientKeyId: ClientKeyId,
): ApiAccessToken | undefined => {
  if (getTokenState(formId, clientKeyId) === "expired") {
    sessionStorage.removeItem(cacheKey(formId, clientKeyId));
    return undefined;
  }

  const accessToken = sessionStorage.getItem(
    cacheKey(formId, clientKeyId),
  ) as ApiAccessToken | null;

  if (accessToken === null) {
    return undefined;
  }

  const exp = extractExp(accessToken);

  if (exp < new Date()) {
    console.warn("Access token has expired. Requesting a new one.");
    return undefined;
  }

  return accessToken;
};

type TokenState = "loading" | "done" | "expired";
const accessTokenState = ref(new Map<string, TokenState>());

const setTokenState = (formId: FormId, clientKeyId: ClientKeyId, value: TokenState) => {
  accessTokenState.value.set(cacheKey(formId, clientKeyId), value);
};

const getTokenState = (formId: FormId, clientKeyId: ClientKeyId) =>
  accessTokenState.value.get(cacheKey(formId, clientKeyId));

const extractNonce = (challengeToken: ApiChallengeToken): ApiChallengeNonce => {
  const { nonce } = jwtDecode<{ nonce: string }>(challengeToken);
  return decodeBase64(nonce) as ApiChallengeNonce;
};

const extractRole = (accessToken: ApiAccessToken): AccessRole => {
  const { role } = jwtDecode<{ role: AccessRole }>(accessToken);
  return role;
};

const extractExp = (accessToken: ApiAccessToken): Date => {
  const { exp } = jwtDecode<{ exp: number }>(accessToken);
  return new Date(exp * 1000);
};

export interface AccessTokenParts {
  token: ApiAccessToken;
  // The access role is enforced server-side; this just allows us to
  // conditionally render components.
  role: AccessRole;
}

export const getAccessToken = async (
  formId: FormId,
  clientKeyId: ClientKeyId,
  privateSigningKey: PrivateSigningKey,
): Promise<ApiAccessToken> => {
  const challenge = await api.getChallengeToken({ formId, clientKeyId });
  const nonce = extractNonce(challenge);
  const signature = await signApiChallengeNonce(nonce, privateSigningKey);
  return await api.postAccessToken({
    challenge,
    signature,
  });
};

export const useAccessToken = () => {
  const loadable = ref<Loadable<AccessTokenParts, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId } = useSecretLink();
  const secretLinkKey = useSecretLinkKey();

  watchEffect(async () => {
    const storedAccessToken = loadStoredAccessToken(formId.value, clientKeyId.value);

    if (storedAccessToken !== undefined) {
      loadable.value = {
        state: "done",
        value: {
          token: storedAccessToken,
          role: extractRole(storedAccessToken),
        },
      };

      // Make sure the `watchEffect` tracks that we're done loading the access
      // token, because it won't react to changes in the session storage.
      setTokenState(formId.value, clientKeyId.value, "done");

      return;
    }

    if (getTokenState(formId.value, clientKeyId.value) === "loading") {
      return;
    }

    if (propagatesError(secretLinkKey, loadable)) {
      return;
    }

    if (!isDone(secretLinkKey.value)) {
      return;
    }

    // Keep track of whether we've started loading a given access token to
    // ensure this hook doesn't run more than once concurrently. We want to
    // make sure we don't go through the auth flow more than once per page
    // load.
    setTokenState(formId.value, clientKeyId.value, "loading");

    let privateSigningKey;

    try {
      const { privateSigningKey: key } = await deriveKeys(secretLinkKey.value.value);
      privateSigningKey = key;
    } catch {
      loadable.value = {
        state: "error",
        error: "unauthorized",
      };

      return;
    }

    try {
      const accessToken = await getAccessToken(formId.value, clientKeyId.value, privateSigningKey);

      storeAccessToken(formId.value, clientKeyId.value, accessToken);
      setTokenState(formId.value, clientKeyId.value, "done");

      // Once the token expires, trigger the `watchEffect` to request a new one.
      const tokenTtl = extractExp(accessToken).getTime() - Date.now();
      setTimeout(() => {
        console.warn("Access token has expired. Requesting a new one.");
        setTokenState(formId.value, clientKeyId.value, "expired");
      }, tokenTtl);

      loadable.value = {
        state: "done",
        value: {
          token: accessToken,
          role: extractRole(accessToken),
        },
      };
    } catch (error) {
      if (error instanceof ApiError) {
        loadable.value = {
          state: "error",
          error: error.kind,
        };
      }
    }
  });

  return readonly(loadable);
};

export default useAccessToken;
