import { readonly, ref, toValue, watchEffect, type MaybeRefOrGetter, type Ref } from "vue";
import type { ApiErrorKind, GetPasswordResponse } from "@/api";
import {
  exposeSecretLinkKey,
  type ClientKeyId,
  type FormId,
  type ProtectedSecretLinkKey,
  type SecretLinkKey,
} from "@/crypto";
import { decodeBase64, encodeBase64 } from "@/encoding";
import { isDone, type Loadable } from "@/types";
import useSecretLink from "./useSecretLink";
import api, { ApiError } from "@/api";

const cacheKey = (formId: FormId, clientKeyId: ClientKeyId) => `key:${formId}/${clientKeyId}`;

const storeSecretLinkKey = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  secretLinkKey: SecretLinkKey,
) => {
  // We use the session storage instead of the local storage because the secret
  // link key is highly sensitive; we cache it to avoid making the user
  // re-enter their password, but it can always be derived from the URL
  // fragment.
  sessionStorage.setItem(cacheKey(formId, clientKeyId), encodeBase64(secretLinkKey));
};

const loadStoredSecretLinkKey = (
  formId: FormId,
  clientKeyId: ClientKeyId,
): SecretLinkKey | undefined => {
  const encodedSecretLinkKey = sessionStorage.getItem(cacheKey(formId, clientKeyId));
  const secretLinkKey =
    encodedSecretLinkKey === null
      ? undefined
      : (decodeBase64(encodedSecretLinkKey) as SecretLinkKey);

  if (secretLinkKey === undefined) {
    return undefined;
  }

  return secretLinkKey;
};

type PasswordErrorKind = "no-password" | "invalid-password";

const useSecretLinkKey = (
  passwordRef: MaybeRefOrGetter<string | undefined>,
): Readonly<Ref<Loadable<SecretLinkKey, ApiErrorKind | PasswordErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkKey, ApiErrorKind | PasswordErrorKind>>({
    state: "loading",
  });

  const secretLinkParts = useSecretLink();

  watchEffect(async () => {
    if (!isDone(secretLinkParts)) {
      return;
    }

    const { formId, clientKeyId, maybeProtectedSecretLinkKey } = secretLinkParts.value.value;

    const password = toValue(passwordRef);

    const storedSecretLinkKey = loadStoredSecretLinkKey(formId, clientKeyId);

    if (storedSecretLinkKey !== undefined) {
      loadable.value = {
        state: "done",
        value: storedSecretLinkKey,
      };

      return;
    }

    let passwordParams: GetPasswordResponse | undefined;

    try {
      passwordParams = await api.getPassword({ formId, clientKeyId });
    } catch (error) {
      if (error instanceof ApiError) {
        loadable.value = {
          state: "error",
          error: error.kind,
        };
      }

      return;
    }

    let secretLinkKey: SecretLinkKey;

    if (passwordParams === undefined) {
      // This secret link *is not* password-protected.
      secretLinkKey = maybeProtectedSecretLinkKey as SecretLinkKey;
    } else {
      // This secret link *is* password-protected.
      if (password === undefined) {
        loadable.value = {
          state: "error",
          error: "no-password",
        };

        return;
      }

      try {
        secretLinkKey = await exposeSecretLinkKey(
          passwordParams.salt,
          passwordParams.nonce,
          maybeProtectedSecretLinkKey as ProtectedSecretLinkKey,
          password,
        );
      } catch {
        loadable.value = {
          state: "error",
          error: "invalid-password",
        };

        return;
      }
    }

    storeSecretLinkKey(formId, clientKeyId, secretLinkKey);

    loadable.value = {
      state: "done",
      value: secretLinkKey,
    };
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
