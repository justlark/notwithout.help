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

export default useSecretLink;
