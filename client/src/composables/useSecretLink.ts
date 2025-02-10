import type { ClientKeyId, FormId, MaybeProtectedSecretLinkKey, SecretLinkKey } from "@/crypto";
import { decodeBase64Url } from "@/encoding";
import { readonly, ref, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useRoute } from "vue-router";

export interface SecretLinkParts {
  formId: DeepReadonly<Ref<FormId>>;
  clientKeyId: DeepReadonly<Ref<ClientKeyId>>;
  secretLinkKey: DeepReadonly<Ref<MaybeProtectedSecretLinkKey>>;
}

export const useSecretLink = (): SecretLinkParts => {
  const route = useRoute();
  const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = route.hash.split("/");

  const formId = ref(formIdPart as FormId);
  const clientKeyId = ref(clientKeyIdPart as ClientKeyId);

  let secretLinkKey;

  try {
    secretLinkKey = ref(decodeBase64Url(secretLinkKeyPart) as MaybeProtectedSecretLinkKey);
  } catch {
    // If the secret link key isn't a valid base64Url string, return an empty
    // array and the error will be handled by he downstream code that attempts
    // to derive keys from it.
    secretLinkKey = ref(new Uint8Array() as MaybeProtectedSecretLinkKey);
  }

  watchEffect(() => {
    const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = route.hash.split("/");

    formId.value = formIdPart as FormId;
    clientKeyId.value = clientKeyIdPart as ClientKeyId;

    try {
      secretLinkKey.value = decodeBase64Url(secretLinkKeyPart) as MaybeProtectedSecretLinkKey;
    } catch {
      secretLinkKey.value = new Uint8Array() as MaybeProtectedSecretLinkKey;
    }
  });

  return {
    formId: readonly(formId),
    clientKeyId: readonly(clientKeyId),
    secretLinkKey: readonly(secretLinkKey),
  };
};

export default useSecretLink;
