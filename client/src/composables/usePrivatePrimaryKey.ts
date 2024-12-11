import type { ApiErrorKind } from "@/api";
import { deriveKeys, unwrapPrivatePrimaryKey, type PrivatePrimaryKey } from "@/crypto";
import type { Loadable } from "@/types";
import { ref, readonly, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useSecretLink } from "./useSecretLink";
import { useAccessToken } from "./useAccessToken";
import api, { ApiError } from "@/api";

export const usePrivatePrimaryKey = (): DeepReadonly<
  Ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>
> => {
  const loadable = ref<Loadable<PrivatePrimaryKey, ApiErrorKind>>({ state: "loading" });

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();
  const accessToken = useAccessToken();

  watchEffect(async () => {
    if (accessToken.value.state !== "done") {
      return;
    }

    // Touch these before the first await boundary to make sure they're
    // tracked.
    const formIdValue = formId.value;
    const clientKeyIdValue = clientKeyId.value;

    const { secretWrappingKey } = await deriveKeys(secretLinkKey.value);

    try {
      const wrappedPrivatePrimaryKey = await api.getKey({
        formId: formIdValue,
        clientKeyId: clientKeyIdValue,
        accessToken: accessToken.value.value,
      });

      loadable.value = {
        state: "done",
        value: unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey),
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
