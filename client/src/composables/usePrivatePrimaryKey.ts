import type { ApiErrorKind } from "@/api";
import { deriveKeys, unwrapPrivatePrimaryKey, type PrivatePrimaryKey } from "@/crypto";
import { isDone, propagatesError, type Loadable } from "@/types";
import { ref, readonly, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useSecretLink } from "./useSecretLink";
import { useAccessToken } from "./useAccessToken";
import api, { ApiError } from "@/api";
import useSecretLinkKey from "./useSecretLinkKey";
import { injectPassword } from "@/injectKeys";

export const usePrivatePrimaryKey = (): DeepReadonly<
  Ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>
> => {
  const loadable = ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>({ state: "loading" });

  const secretLinkParts = useSecretLink();
  const accessToken = useAccessToken();
  const secretLinkKey = useSecretLinkKey(injectPassword());

  watchEffect(async () => {
    if (propagatesError(loadable, accessToken, secretLinkKey)) {
      return;
    }

    if (!isDone(accessToken) || !isDone(secretLinkKey) || !isDone(secretLinkParts)) {
      return;
    }

    const { token } = accessToken.value.value;
    const { formId, clientKeyId } = secretLinkParts.value.value;

    // Touch these before the first await boundary to make sure they're
    // tracked.
    const formIdValue = formId;
    const clientKeyIdValue = clientKeyId;
    let secretWrappingKey;

    try {
      const { secretWrappingKey: key } = await deriveKeys(secretLinkKey.value.value);
      secretWrappingKey = key;
    } catch {
      loadable.value = {
        state: "error",
        error: "unauthorized",
      };
      return;
    }

    try {
      const wrappedPrivatePrimaryKey = await api.getKey({
        formId: formIdValue,
        clientKeyId: clientKeyIdValue,
        accessToken: token,
      });

      loadable.value = {
        state: "done",
        value: await unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey),
      };
    } catch (error) {
      if (error instanceof ApiError) {
        loadable.value = {
          state: "error",
          error: error.kind,
        };
      }
    }
  });

  return readonly(loadable);
};

export default usePrivatePrimaryKey;
