import type { FormId, PrimaryKeyFingerprint } from "@/crypto";
import { decodeBase64Url } from "@/encoding";
import type { Loadable } from "@/types";
import { ref, watchEffect, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

export interface ShareLinkParts {
  formId: FormId;
  primaryKeyFingerprint: PrimaryKeyFingerprint;
}

export const useLink = (): Readonly<Ref<Loadable<ShareLinkParts, "invalid">>> => {
  const loadable = ref<Loadable<ShareLinkParts, "invalid">>({ state: "loading" });

  const route = useRoute();
  const router = useRouter();

  watchEffect(async () => {
    const fragment = route.hash;

    await router.isReady();

    const [, formIdPart, primaryKeyFingerprintPart] = fragment.split("/");

    if (!primaryKeyFingerprintPart) {
      loadable.value = {
        state: "error",
        error: "invalid",
      };

      return;
    }

    let primaryKeyFingerprint: PrimaryKeyFingerprint;

    try {
      primaryKeyFingerprint = decodeBase64Url(primaryKeyFingerprintPart) as PrimaryKeyFingerprint;
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
        primaryKeyFingerprint,
      },
    };
  });

  return loadable;
};

export default useLink;
