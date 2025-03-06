import {
  onMounted,
  readonly,
  ref,
  toValue,
  watchEffect,
  type MaybeRefOrGetter,
  type Ref,
} from "vue";
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
import { onUnmounted } from "vue";

const cacheKey = (formId: FormId, clientKeyId: ClientKeyId) => `key:${formId}/${clientKeyId}`;

// Window events that we listen for to reset the user idle timeout.
const IDLE_WINDOW_EVENTES = ["mousemove", "keydown", "scroll", "click", "touchstart"];

// After this many seconds of inactivity, we clear the secret link key from the
// session storage, requiring the user to re-enter their password.
const INVACTIVE_PASSWORD_SESSION_TIMEOUT_SECONDS = 15;

const idleTimeoutId = ref<ReturnType<typeof setTimeout>>();
const passwordTimedOut = ref(false);
const isPasswordProtected = ref(true);

const resetIdleTimeout = () => {
  passwordTimedOut.value = false;
  clearTimeout(toValue(idleTimeoutId));
  idleTimeoutId.value = undefined;
};

const restartIdleTimeout = ({
  timeoutSeconds,
  formId,
  clientKeyId,
}: {
  timeoutSeconds: number;
  formId: FormId;
  clientKeyId: ClientKeyId;
}) => {
  resetIdleTimeout();

  idleTimeoutId.value = setTimeout(() => {
    if (!isPasswordProtected.value) {
      return;
    }

    sessionStorage.removeItem(cacheKey(formId, clientKeyId));
    passwordTimedOut.value = true;
  }, timeoutSeconds * 1000);
};

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

type PasswordErrorKind = "no-password" | "invalid-password" | "idle-timeout";

const useSecretLinkKey = (
  passwordRef: MaybeRefOrGetter<string | undefined>,
): Readonly<Ref<Loadable<SecretLinkKey, ApiErrorKind | PasswordErrorKind>>> => {
  const loadable = ref<Loadable<SecretLinkKey, ApiErrorKind | PasswordErrorKind>>({
    state: "loading",
  });

  const secretLinkParts = useSecretLink();

  const restartIdleTimeoutAction = () => {
    if (!isDone(secretLinkParts)) {
      return;
    }

    restartIdleTimeout({
      timeoutSeconds: INVACTIVE_PASSWORD_SESSION_TIMEOUT_SECONDS,
      formId: secretLinkParts.value.value.formId,
      clientKeyId: secretLinkParts.value.value.clientKeyId,
    });
  };

  watchEffect(async () => {
    if (!isDone(secretLinkParts)) {
      return;
    }

    if (passwordTimedOut.value) {
      loadable.value = {
        state: "error",
        error: "idle-timeout",
      };

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
      isPasswordProtected.value = false;

      secretLinkKey = maybeProtectedSecretLinkKey as SecretLinkKey;
    } else {
      isPasswordProtected.value = true;

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
    restartIdleTimeoutAction();

    loadable.value = {
      state: "done",
      value: secretLinkKey,
    };
  });

  onMounted(() => {
    IDLE_WINDOW_EVENTES.forEach((event) => {
      window.addEventListener(event, restartIdleTimeoutAction);
    });
  });

  onUnmounted(() => {
    IDLE_WINDOW_EVENTES.forEach((event) => {
      window.removeEventListener(event, restartIdleTimeoutAction);
    });
  });

  return readonly(loadable);
};

export default useSecretLinkKey;
