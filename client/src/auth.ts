import { jwtDecode } from "jwt-decode";
import {
  deriveKeys,
  signApiChallengeNonce,
  unwrapPrivatePrimaryKey,
  type ApiAccessToken,
  type ApiChallengeNonce,
  type ApiChallengeToken,
  type PrivatePrimaryKey,
  type PrivateSigningKey,
  type PublicPrimaryKey,
  type SecretLinkKey,
} from "./crypto";
import type { ClientKeyId, FormId, Loadable } from "./types";
import api, { ApiError, type ApiErrorKind } from "./api";
import { decodeBase64, decodeBase64Url } from "./encoding";
import { ref, watchEffect, readonly, type DeepReadonly, type Ref, type ToRef } from "vue";
import { useRoute } from "vue-router";
import { type ContactMethodCode } from "./vars";

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

export interface ShareLinkParts {
  formId: FormId;
}

export const useLink = (): DeepReadonly<Ref<ShareLinkParts>> => {
  const route = useRoute();
  const [, formId] = route.hash.split("/");

  const parts = ref<ShareLinkParts>({
    formId: formId as FormId,
  });

  watchEffect(() => {
    const [, formId] = route.hash.split("/");

    parts.value = {
      formId: formId as FormId,
    };
  });

  return readonly(parts);
};

export interface SecretLinkParts {
  formId: FormId;
  clientKeyId: ClientKeyId;
  secretLinkKey: SecretLinkKey;
}

export const useSecretLink = (): DeepReadonly<Ref<SecretLinkParts>> => {
  const route = useRoute();
  const [, formId, clientKeyId, secretLinkKey] = route.hash.split("/");

  const parts = ref<SecretLinkParts>({
    formId: formId as FormId,
    clientKeyId: clientKeyId as ClientKeyId,
    secretLinkKey: decodeBase64Url(secretLinkKey) as SecretLinkKey,
  });

  watchEffect(() => {
    const [, formId, clientKeyId, secretLinkKey] = route.hash.split("/");

    parts.value = {
      formId: formId as FormId,
      clientKeyId: clientKeyId as ClientKeyId,
      secretLinkKey: decodeBase64Url(secretLinkKey) as SecretLinkKey,
    };
  });

  return readonly(parts);
};

export const useAccessToken = () => {
  const loadable = ref<Loadable<ApiAccessToken, ApiErrorKind>>({ state: "loading" });

  const secretLinkParts = useSecretLink();

  watchEffect(async () => {
    const { formId, clientKeyId, secretLinkKey } = secretLinkParts.value;

    const { privateSigningKey } = await deriveKeys(secretLinkKey);

    try {
      loadable.value = {
        state: "done",
        value: await getAccessToken(formId, clientKeyId, privateSigningKey),
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

export const usePrivatePrimaryKey = (
  accessToken: ToRef<ApiAccessToken | undefined>,
): DeepReadonly<Ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>> => {
  const loadable = ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>({ state: "loading" });

  const secretLinkParts = useSecretLink();

  watchEffect(async () => {
    if (accessToken.value === undefined) {
      return;
    }

    const { formId, clientKeyId, secretLinkKey } = secretLinkParts.value;

    const { secretWrappingKey } = await deriveKeys(secretLinkKey);

    try {
      const wrappedPrivatePrimaryKey = await api.getKey({
        formId: formId,
        clientKeyId: clientKeyId,
        accessToken: accessToken.value,
      });

      loadable.value = {
        state: "done",
        value: unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey),
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

export interface Form {
  orgName: string;
  description: string;
  contactMethods: Array<ContactMethodCode>;
  publicPrimaryKey: PublicPrimaryKey;
}

export const useForm = (): DeepReadonly<Ref<Loadable<Form, ApiErrorKind>>> => {
  const form = ref<Loadable<Form, ApiErrorKind>>({ state: "loading" });

  const shareLinkParts = useLink();

  watchEffect(async () => {
    const { formId } = shareLinkParts.value;

    try {
      const response = await api.getForm({ formId });

      form.value = {
        state: "done",
        value: {
          orgName: response.orgName,
          description: response.description,
          contactMethods: response.contactMethods,
          publicPrimaryKey: response.publicPrimaryKey,
        },
      };
    } catch (error) {
      if (error instanceof ApiError) {
        form.value = { state: "error", error: error.kind };
      }
    }
  });

  return form;
};
