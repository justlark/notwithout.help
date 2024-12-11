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
  formId: DeepReadonly<Ref<FormId>>;
}

export const useLink = (): ShareLinkParts => {
  const route = useRoute();
  const [, formIdPart] = route.hash.split("/");

  const formId = ref(formIdPart as FormId);

  watchEffect(() => {
    const [, formIdPart] = route.hash.split("/");

    formId.value = formIdPart as FormId;
  });

  return {
    formId: readonly(formId),
  };
};

export interface SecretLinkParts {
  formId: DeepReadonly<Ref<FormId>>;
  clientKeyId: DeepReadonly<Ref<ClientKeyId>>;
  secretLinkKey: DeepReadonly<Ref<SecretLinkKey>>;
}

export const useSecretLink = (): SecretLinkParts => {
  const route = useRoute();
  const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = route.hash.split("/");

  const formId = ref(formIdPart as FormId);
  const clientKeyId = ref(clientKeyIdPart as ClientKeyId);
  const secretLinkKey = ref(decodeBase64Url(secretLinkKeyPart) as SecretLinkKey);

  watchEffect(() => {
    const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = route.hash.split("/");

    formId.value = formIdPart as FormId;
    clientKeyId.value = clientKeyIdPart as ClientKeyId;
    secretLinkKey.value = decodeBase64Url(secretLinkKeyPart) as SecretLinkKey;
  });

  return {
    formId: readonly(formId),
    clientKeyId: readonly(clientKeyId),
    secretLinkKey: readonly(secretLinkKey),
  };
};

export const useAccessToken = () => {
  const loadable = ref<Loadable<ApiAccessToken, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();

  watchEffect(async () => {
    // Touch these before the first await boundary to make sure they're
    // tracked.
    const formIdValue = formId.value;
    const clientKeyIdValue = clientKeyId.value;
    const secretLinkKeyValue = secretLinkKey.value;

    const { privateSigningKey } = await deriveKeys(secretLinkKeyValue);

    try {
      loadable.value = {
        state: "done",
        value: await getAccessToken(formIdValue, clientKeyIdValue, privateSigningKey),
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

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();

  watchEffect(async () => {
    if (accessToken.value === undefined) {
      return;
    }

    // Touch these before the first await boundary to make sure they're
    // tracked.
    const formIdValue = formId.value;
    const clientKeyIdValue = clientKeyId.value;
    const secretLinkKeyValue = secretLinkKey.value;

    const { secretWrappingKey } = await deriveKeys(secretLinkKeyValue);

    try {
      const wrappedPrivatePrimaryKey = await api.getKey({
        formId: formIdValue,
        clientKeyId: clientKeyIdValue,
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

  const { formId } = useLink();

  watchEffect(async () => {
    try {
      const response = await api.getForm({ formId: formId.value });

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
