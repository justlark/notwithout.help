import type { FormId } from "@/crypto";
import type { Loadable } from "@/types";
import { ref, watchEffect, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

export interface ShareLinkParts {
  formId: FormId;
}

export const useLink = (): Readonly<Ref<Loadable<ShareLinkParts, never>>> => {
  const loadable = ref<Loadable<ShareLinkParts, never>>({ state: "loading" });

  const route = useRoute();
  const router = useRouter();

  watchEffect(async () => {
    const fragment = route.hash;

    await router.isReady();

    const [, formIdPart] = fragment.split("/");

    loadable.value = {
      state: "done",
      value: {
        formId: formIdPart as FormId,
      },
    };
  });

  return loadable;
};

export default useLink;
