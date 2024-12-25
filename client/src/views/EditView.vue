<script setup lang="ts">
import api, { ApiError } from "@/api";
import type { FormValues } from "@/components/FormBuilder.vue";
import FormBuilder from "@/components/FormBuilder.vue";
import ErrorCard from "@/components/ErrorCard.vue";
import useAccessToken from "@/composables/useAccessToken";
import useForm from "@/composables/useForm";
import useSecretLink from "@/composables/useSecretLink";
import { isDone, returnsError } from "@/types";
import { newSecretLink, TOAST_ERROR_TTL, TOAST_INFO_TTL } from "@/vars";
import { useToast } from "primevue";
import { computed } from "vue";
import { useRouter } from "vue-router";
import defaultRoles from "@/assets/default-roles.json";

const { formId, clientKeyId, secretLinkKey } = useSecretLink();
const accessToken = useAccessToken();
const form = useForm();

const isNotFound = computed(() => {
  return returnsError(["bad-request", "unauthorized", "forbidden", "not-found"], accessToken, form);
});

const router = useRouter();
const toast = useToast();

const initialValues = computed(() =>
  isDone(form)
    ? {
        title: form.value.value.orgName,
        description: form.value.value.description,
        contactMethods: form.value.value.contactMethods as [string, ...Array<string>],
        expirationDate: form.value.value.expirationDate,
        showRoles: form.value.value.roles.length > 0,
      }
    : undefined,
);

const secretLink = computed(() =>
  newSecretLink(formId.value, clientKeyId.value, secretLinkKey.value),
);

const submitForm = async (values: FormValues, resetForm: () => void) => {
  if (!isDone(accessToken)) {
    return;
  }

  try {
    await api.patchForm({
      formId: formId.value,
      orgName: values.title,
      description: values.description,
      contactMethods: values.contactMethods,
      expirationDate: values.expirationDate,
      accessToken: accessToken.value.value.token,
      roles: values.showRoles ? defaultRoles : [],
    });
  } catch (error) {
    if (error instanceof ApiError && error.kind === "content-too-large") {
      toast.add({
        severity: "error",
        summary: "Failed to create group",
        detail: "Your form is too large. Cut down the number of characters and try again.",
        life: TOAST_ERROR_TTL,
      });
    } else {
      toast.add({
        severity: "error",
        summary: "Failed to create group",
        detail: "Something unexpected happened.",
        life: TOAST_ERROR_TTL,
      });
    }

    resetForm();

    return;
  }

  resetForm();

  toast.add({
    severity: "success",
    summary: "Group updated",
    detail: "Your group has been updated.",
    life: TOAST_INFO_TTL,
  });

  await router.push({ path: secretLink.value.pathname, hash: secretLink.value.hash });
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <ErrorCard
      v-if="isNotFound"
      title="Not found"
      message="Either this is an invalid link, the group has been deleted, or you don't have access to it anymore."
    />
    <FormBuilder
      v-if="initialValues && !isNotFound"
      :storage-key="`edit:${formId}`"
      :initial-values="initialValues"
      @submit="submitForm"
      aria-labelledby="main-heading"
    >
      <template #lead>
        <h1 id="main-heading" class="text-center mb-10">Edit your group</h1>
      </template>
    </FormBuilder>
  </main>
</template>

<style scoped></style>
