import { inject, readonly, ref, watchEffect, type Ref } from "vue";
import type { ApiErrorKind, GetPasswordResponse } from "@/api";
import { exposeSecretLinkKey, type ProtectedSecretLinkKey, type SecretLinkKey } from "@/crypto";
import type { Loadable } from "@/types";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";
import { passwordKey } from "@/injectKeys";

const useSecretLinkKey = (): Readonly<Ref<Loadable<SecretLinkKey, ApiErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkKey, ApiErrorKind>>({ state: "loading" });

  const password = inject<Ref<string | undefined>>(passwordKey);

  const { formId, clientKeyId, maybeProtectedSecretLinkKey } = useSecretLink();

  watchEffect(async () => {
    // Touch these before the first await boundary to make sure they're
    // tracked.
    password?.value;
    maybeProtectedSecretLinkKey.value;

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
        value: maybeProtectedSecretLinkKey.value as SecretLinkKey,
      };

      return;
    }

    if (password?.value === undefined) {
      return;
    }

    let secretLinkKey: SecretLinkKey;

    try {
      secretLinkKey = await exposeSecretLinkKey(
        passwordParams.salt,
        passwordParams.nonce,
        maybeProtectedSecretLinkKey.value as ProtectedSecretLinkKey,
        password.value,
      );
    } catch {
      loadable.value = {
        state: "error",
        error: "unauthorized",
      };

      return;
    }

    // This secret link *is* password-protected.
    loadable.value = {
      state: "done",
      value: secretLinkKey,
    };
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
