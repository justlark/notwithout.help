import { readonly, ref, toValue, watchEffect, type MaybeRefOrGetter, type Ref } from "vue";
import type { ApiErrorKind, GetPasswordResponse } from "@/api";
import { exposeSecretLinkKey, type ProtectedSecretLinkKey, type SecretLinkKey } from "@/crypto";
import type { Loadable } from "@/types";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";
import { useRouter } from "vue-router";

export type SecretLinkSource =
  | {
      protected: false;
      secretLinkKey: SecretLinkKey;
    }
  | {
      protected: true;
      secretLinkKey: (password: string) => Promise<SecretLinkKey>;
    };

const useSecretLinkKey = (): Readonly<Ref<Loadable<SecretLinkSource, ApiErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkSource, ApiErrorKind>>({ state: "loading" });

  const router = useRouter();
  const { formId, clientKeyId, secretLinkKey: maybeProtectedSecretLinkKey } = useSecretLink();

  watchEffect(async () => {
    // If we don't wait for the router to be ready, the app will try and
    // request the password params before the URL fragment (and therefore the
    // Form ID and Client Key ID) is available.
    await router.isReady();

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
      // This secret link *is not* password-protected.
      loadable.value = {
        state: "done",
        value: {
          protected: false,
          secretLinkKey: maybeProtectedSecretLinkKey.value,
        },
      };

      return;
    }

    // This secret link *is* password-protected.
    loadable.value = {
      state: "done",
      value: {
        protected: true,
        secretLinkKey: (password: string) =>
          exposeSecretLinkKey(
            passwordParams.salt,
            passwordParams.nonce,
            maybeProtectedSecretLinkKey.value as unknown as ProtectedSecretLinkKey,
            password,
          ),
      },
    };
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
