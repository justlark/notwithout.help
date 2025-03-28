import type { ApiErrorKind, OrgRole } from "@/api";
import api, { ApiError } from "@/api";
import type { FormId, PublicPrimaryKey } from "@/crypto";
import { type Loadable } from "@/types";
import { ref, watchEffect, type DeepReadonly, type Ref } from "vue";
import { useRoute, useRouter } from "vue-router";

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

  const route = useRoute();
  const router = useRouter();

  watchEffect(async () => {
    const fragment = route.hash;

    await router.isReady();

    const [, formIdPart] = fragment.split("/");

    const formId = formIdPart as FormId;

    try {
      const response = await api.getForm({ formId });

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
