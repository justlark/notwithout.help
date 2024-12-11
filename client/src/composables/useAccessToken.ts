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
import { ref, watchEffect, readonly, computed } from "vue";
import { decodeBase64 } from "@/encoding";
import { jwtDecode } from "jwt-decode";
import { useSecretLink } from "./useSecretLink";

const accessTokenCache = ref(new Map<string, ApiAccessToken>());
const isLoadingAccessToken = ref(false);

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

  const cacheKey = computed(() => `${formId.value}/${clientKeyId.value}`);

  watchEffect(async () => {
    if (isLoadingAccessToken.value || loadable.value.state === "done") {
      return;
    }

    const cachedAccessToken = accessTokenCache.value.get(cacheKey.value);

    if (cachedAccessToken !== undefined) {
      console.log("using cached access token");
      loadable.value = {
        state: "done",
        value: cachedAccessToken,
      };

      return;
    }

    isLoadingAccessToken.value = true;

    const { privateSigningKey } = await deriveKeys(secretLinkKey.value);

    try {
      console.log("getting new access token");
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

    isLoadingAccessToken.value = false;
  });

  return readonly(loadable);
};

export default useAccessToken;
