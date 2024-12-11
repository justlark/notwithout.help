import type { FormId } from "@/types";
import { readonly, ref, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useRoute } from "vue-router";

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

export default useLink;
