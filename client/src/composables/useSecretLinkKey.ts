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
import type { Loadable } from "@/types";
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

type SecretLinkKeyState = "loading" | "done";
const secretLinkKeyState = ref(new Map<string, SecretLinkKeyState>());

const setSecretLinkKeyState = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  value: SecretLinkKeyState,
) => {
  secretLinkKeyState.value.set(cacheKey(formId, clientKeyId), value);
};

const getSecretLinkKeyState = (formId: FormId, clientKeyId: ClientKeyId) =>
  secretLinkKeyState.value.get(cacheKey(formId, clientKeyId));

const useSecretLinkKey = (
  passwordRef: MaybeRefOrGetter<string | undefined>,
): Readonly<Ref<Loadable<SecretLinkKey, ApiErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkKey, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId, maybeProtectedSecretLinkKey } = useSecretLink();

  watchEffect(async () => {
    const password = toValue(passwordRef);

    // Touch these before the first await boundary to make sure they're
    // tracked.
    maybeProtectedSecretLinkKey.value;

    const storedSecretLinkKey = loadStoredSecretLinkKey(formId.value, clientKeyId.value);

    if (storedSecretLinkKey !== undefined) {
      loadable.value = {
        state: "done",
        value: storedSecretLinkKey,
      };

      // Make sure the `watchEffect` tracks that we're done loading the secret
      // link key, because it won't react to changes in the session storage.
      setSecretLinkKeyState(formId.value, clientKeyId.value, "done");

      return;
    }

    if (getSecretLinkKeyState(formId.value, clientKeyId.value) === "loading") {
      return;
    }

    // Keep track of whether we've started loading a given secret link key to
    // ensure this hook doesn't run more than once concurrently. We want to
    // make sure we don't call the API endpoint and decrypt the protected
    // secret link key more than once per page load; it's inefficient.
    setSecretLinkKeyState(formId.value, clientKeyId.value, "loading");

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

    let secretLinkKey: SecretLinkKey;

    if (passwordParams === undefined) {
      // This secret link *is not* password-protected.
      secretLinkKey = maybeProtectedSecretLinkKey.value as SecretLinkKey;
    } else {
      // This secret link *is* password-protected.
      if (password === undefined) {
        return;
      }

      try {
        secretLinkKey = await exposeSecretLinkKey(
          passwordParams.salt,
          passwordParams.nonce,
          maybeProtectedSecretLinkKey.value as ProtectedSecretLinkKey,
          password,
        );
      } catch {
        loadable.value = {
          state: "error",
          error: "unauthorized",
        };

        return;
      }
    }

    storeSecretLinkKey(formId.value, clientKeyId.value, secretLinkKey);
    setSecretLinkKeyState(formId.value, clientKeyId.value, "done");

    loadable.value = {
      state: "done",
      value: secretLinkKey,
    };
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
