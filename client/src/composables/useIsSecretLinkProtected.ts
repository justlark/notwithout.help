import type { ApiErrorKind } from "@/api";
import type { Loadable } from "@/types";
import { readonly, ref, watchEffect, type Ref } from "vue";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";

const useIsSecretLinkProtected = (): Readonly<Ref<Loadable<boolean, ApiErrorKind>>> => {
  const loadable = ref<Loadable<boolean, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId } = useSecretLink();

  watchEffect(async () => {
    try {
      const passwordParams = await api.getPassword({
        formId: formId.value,
        clientKeyId: clientKeyId.value,
      });

      loadable.value = {
        state: "done",
        value: passwordParams !== undefined,
      };

      return;
    } catch (error) {
      if (error instanceof ApiError) {
        loadable.value = {
          state: "error",
          error: error.kind,
        };
      }

      return;
    }
  });

  return readonly(loadable);
};

export default useIsSecretLinkProtected;
