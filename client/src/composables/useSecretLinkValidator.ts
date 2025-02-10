import type { ApiErrorKind, GetPasswordResponse } from "@/api";
import type { Loadable } from "@/types";
import { readonly, ref, watchEffect, type Ref } from "vue";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";
import { exposeSecretLinkKey, type ProtectedSecretLinkKey } from "@/crypto";

export type SecretLinkValidator =
  | {
      protected: true;
      validator: (password: string) => Promise<boolean>;
    }
  | {
      protected: false;
    };

const useSecretLinkValidator = (): Readonly<Ref<Loadable<SecretLinkValidator, ApiErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkValidator, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId, maybeProtectedSecretLinkKey } = useSecretLink();

  watchEffect(async () => {
    let passwordParams: GetPasswordResponse | undefined;

    try {
      passwordParams = await api.getPassword({
        formId: formId.value,
        clientKeyId: clientKeyId.value,
      });
    } catch (error) {
      if (error instanceof ApiError) {
        loadable.value = {
          state: "error",
          error: error.kind,
        };
      }

      return;
    }

    if (passwordParams === undefined) {
      loadable.value = {
        state: "done",
        value: { protected: false },
      };

      return;
    }

    loadable.value = {
      state: "done",
      value: {
        protected: true,
        validator: async (password) => {
          try {
            await exposeSecretLinkKey(
              passwordParams.salt,
              passwordParams.nonce,
              maybeProtectedSecretLinkKey.value as ProtectedSecretLinkKey,
              password,
            );

            return true;
          } catch {
            return false;
          }
        },
      },
    };
  });

  return readonly(loadable);
};

export default useSecretLinkValidator;
