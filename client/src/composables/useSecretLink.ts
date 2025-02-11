import type { ClientKeyId, FormId, MaybeProtectedSecretLinkKey } from "@/crypto";
import { decodeBase64Url } from "@/encoding";
import type { Loadable } from "@/types";
import { ref, watchEffect, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

export interface SecretLinkParts {
  formId: FormId;
  clientKeyId: ClientKeyId;
  maybeProtectedSecretLinkKey: MaybeProtectedSecretLinkKey;
}

export const useSecretLink = (): Readonly<Ref<Loadable<SecretLinkParts, never>>> => {
  const loadable = ref<Loadable<SecretLinkParts, never>>({ state: "loading" });

  const route = useRoute();
  const router = useRouter();

  watchEffect(async () => {
    const fragment = route.hash;

    await router.isReady();

    const [, formIdPart, clientKeyIdPart, secretLinkKeyPart] = fragment.split("/");

    let secretLinkKey;

    try {
      secretLinkKey = decodeBase64Url(secretLinkKeyPart) as MaybeProtectedSecretLinkKey;
    } catch {
      // If the secret link key isn't a valid base64Url string, return an empty
      // array and the error will be handled by he downstream code that
      // attempts to derive keys from it.
      secretLinkKey = new Uint8Array() as MaybeProtectedSecretLinkKey;
    }

    loadable.value = {
      state: "done",
      value: {
        formId: formIdPart as FormId,
        clientKeyId: clientKeyIdPart as ClientKeyId,
        maybeProtectedSecretLinkKey: secretLinkKey,
      },
    };
  });

  return loadable;
};

export default useSecretLink;
