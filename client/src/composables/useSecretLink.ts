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

export const useSecretLink = (): Readonly<Ref<Loadable<SecretLinkParts, "invalid">>> => {
  const loadable = ref<Loadable<SecretLinkParts, "invalid">>({ state: "loading" });

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
      loadable.value = {
        state: "error",
        error: "invalid",
      };

      return;
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
