import { inject, readonly, ref, watchEffect, type Ref } from "vue";
import type { ApiErrorKind, GetPasswordResponse } from "@/api";
import { exposeSecretLinkKey, type ProtectedSecretLinkKey, type SecretLinkKey } from "@/crypto";
import type { Loadable } from "@/types";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";
import { useRouter } from "vue-router";
import { passwordKey } from "@/injectKeys";

const useSecretLinkKey = (): Readonly<Ref<Loadable<SecretLinkKey, ApiErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkKey, ApiErrorKind>>({ state: "loading" });

  const password = inject<Ref<string | undefined>>(passwordKey);

  const router = useRouter();
  const { formId, clientKeyId, maybeProtectedSecretLinkKey } = useSecretLink();

  watchEffect(async () => {
    // Touch these before the first await boundary to make sure they're
    // tracked.
    password?.value;
    formId.value;
    clientKeyId.value;
    maybeProtectedSecretLinkKey.value;

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
        value: maybeProtectedSecretLinkKey.value as SecretLinkKey,
      };

      return;
    }

    if (password?.value === undefined) {
      return;
    }

    // This secret link *is* password-protected.
    loadable.value = {
      state: "done",
      value: await exposeSecretLinkKey(
        passwordParams.salt,
        passwordParams.nonce,
        maybeProtectedSecretLinkKey.value as ProtectedSecretLinkKey,
        password.value,
      ),
    };
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
