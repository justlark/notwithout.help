import type { ClientKeyId, FormId, SecretLinkKey } from "@/crypto";
import { decodeBase64Url } from "@/encoding";
import { readonly, ref, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useRoute } from "vue-router";

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

  let secretLinkKey;

  try {
    secretLinkKey = ref(decodeBase64Url(secretLinkKeyPart) as SecretLinkKey);
  } catch {
    // If the secret link key isn't a valid base64Url string, return an empty
    // array and the error will be handled by he downstream code that attempts
    // to derive keys from it.
    secretLinkKey = ref(new Uint8Array() as SecretLinkKey);
  }

  watchEffect(() => {
    const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = route.hash.split("/");

    formId.value = formIdPart as FormId;
    clientKeyId.value = clientKeyIdPart as ClientKeyId;

    try {
      secretLinkKey.value = decodeBase64Url(secretLinkKeyPart) as SecretLinkKey;
    } catch {
      secretLinkKey.value = new Uint8Array() as SecretLinkKey;
    }
  });

  return {
    formId: readonly(formId),
    clientKeyId: readonly(clientKeyId),
    secretLinkKey: readonly(secretLinkKey),
  };
};

export default useSecretLink;
