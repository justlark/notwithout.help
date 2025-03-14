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

const secretLinkParts = useSecretLink();
const accessToken = useAccessToken();
const form = useForm();

const isNotFound = computed(() => {
  return returnsError(["bad-request", "unauthorized", "forbidden", "not-found"], accessToken, form);
});

const router = useRouter();
const toast = useToast();

const initialValues = computed(() => {
  if (!isDone(form)) {
    return undefined;
  }

  // Convert the read-only object into a mutable object.
  const roles = form.value.value.roles.map((role) => ({
    ...role,
    details: [...role.details],
  }));

  let rolesPreset: FormValues["rolesPreset"] = "none";

  // Match only on the *shape* of the objects.
  if (JSON.stringify(roles) === JSON.stringify(defaultRoles)) {
    rolesPreset = "default";
  } else if (roles.length > 0) {
    rolesPreset = "custom";
  }

  return {
    title: form.value.value.orgName,
    description: form.value.value.description,
    contactMethods: form.value.value.contactMethods as [string, ...Array<string>],
    expirationDate: form.value.value.expirationDate,
    rolesPreset,
    roles,
  };
});

const secretLink = computed(() =>
  isDone(secretLinkParts)
    ? newSecretLink(
        secretLinkParts.value.value.formId,
        secretLinkParts.value.value.clientKeyId,
        secretLinkParts.value.value.maybeProtectedSecretLinkKey,
      )
    : undefined,
);

const submitForm = async (values: FormValues, resetForm: () => void) => {
  if (!isDone(accessToken) || !isDone(secretLinkParts) || secretLink.value === undefined) {
    return;
  }

  const { formId } = secretLinkParts.value.value;

  try {
    await api.patchForm({
      formId: formId,
      orgName: values.title,
      description: values.description,
      contactMethods: values.contactMethods,
      expirationDate: values.expirationDate,
      accessToken: accessToken.value.value.token,
      roles: values.roles,
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
      v-if="isDone(secretLinkParts) && initialValues && !isNotFound"
      :storage-key="`edit:${secretLinkParts.value.formId}`"
      :initial-values="initialValues"
      cancelable
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
