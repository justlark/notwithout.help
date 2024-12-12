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
import { type Loadable } from "@/types";
import api, { ApiError, type ApiErrorKind } from "@/api";
import { ref, readonly, computed, watchEffect } from "vue";
import { decodeBase64 } from "@/encoding";
import { jwtDecode } from "jwt-decode";
import { useSecretLink } from "./useSecretLink";

// We cache access tokens to avoid going through the auth flow more than once
// per page load.
//
// Remember: The URL fragment (and therefore the form ID, client key ID, and
// secret link key) can change without reloading the page.
//
// TODO: What happens if the access token expires before the user reloads the
// page?
const accessTokenCache = ref(new Map<string, ApiAccessToken>());

// Keep track of whether we've started loading a given access token to ensure
// this hook doesn't run more than once concurrently. We want to make sure we
// don't go through the auth flow more than once per page load.
//
// Once this value is set to `true` for a given (form ID, client key ID) pair,
// it will stay `true` for the lifetime of the page load, because and invalid
// secret link is unlikely to ever become valid in the future; revoked secret
// links cannot be re-enabled.
const accessTokenHasStartedLoading = ref(new Map<string, boolean>());

const extractNonce = (challengeToken: ApiChallengeToken): ApiChallengeNonce => {
  const { nonce } = jwtDecode<{ nonce: string }>(challengeToken);
  return decodeBase64(nonce) as ApiChallengeNonce;
};

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
  const loadable = ref<Loadable<ApiAccessToken, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();

  const cacheKey = computed(() => `${formId.value}/${clientKeyId.value}/${secretLinkKey.value}`);

  watchEffect(async () => {
    const cachedAccessToken = accessTokenCache.value.get(cacheKey.value);

    if (cachedAccessToken !== undefined) {
      loadable.value = {
        state: "done",
        value: cachedAccessToken,
      };

      return;
    }

    if (accessTokenHasStartedLoading.value.get(cacheKey.value)) {
      return;
    }

    accessTokenHasStartedLoading.value.set(cacheKey.value, true);

    let privateSigningKey;

    try {
      const { privateSigningKey: key } = await deriveKeys(secretLinkKey.value);
      privateSigningKey = key;
    } catch {
      loadable.value = {
        state: "error",
        error: "unauthorized",
      };
    }

    try {
      const accessToken = await getAccessToken(formId.value, clientKeyId.value, privateSigningKey);

      accessTokenCache.value.set(cacheKey.value, accessToken);

      loadable.value = {
        state: "done",
        value: accessToken,
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
