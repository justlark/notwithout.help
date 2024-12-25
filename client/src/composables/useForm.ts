import type { ApiErrorKind, OrgRole } from "@/api";
import api, { ApiError } from "@/api";
import type { PublicPrimaryKey } from "@/crypto";
import type { Loadable } from "@/types";
import { ref, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useLink } from "./useLink";

export interface Form {
  orgName: string;
  description: string;
  contactMethods: Array<string>;
  publicPrimaryKey: PublicPrimaryKey;
  expirationDate: Date | undefined;
  roles: Array<OrgRole>;
}

export const useForm = (): DeepReadonly<Ref<Loadable<Form, ApiErrorKind>>> => {
  const form = ref<Loadable<Form, ApiErrorKind>>({ state: "loading" });

  const { formId } = useLink();

  watchEffect(async () => {
    try {
      const response = await api.getForm({ formId: formId.value });

      form.value = {
        state: "done",
        value: {
          orgName: response.orgName,
          description: response.description,
          contactMethods: response.contactMethods,
          publicPrimaryKey: response.publicPrimaryKey,
          expirationDate: response.expirationDate,
          roles: response.roles ?? [],
        },
      };
    } catch (error) {
      if (error instanceof ApiError) {
        form.value = { state: "error", error: error.kind };
      }
    }
  });

  return form;
};

export default useForm;
