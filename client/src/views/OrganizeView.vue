<script setup lang="ts">
import { computed, ref } from "vue";
import FormAdmonitions from "@/components/FormAdmonitions.vue";
import FormBuilder, { type FormValues } from "@/components/FormBuilder.vue";
import api, { ApiError } from "@/api";
import {
  deriveKeys,
  sealKeyComment,
  generatePrimaryKeypair,
  generateSecretLinkKey,
  wrapPrivatePrimaryKey,
  type ClientKeyId,
  type FormId,
  type SecretLinkKey,
} from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { getAccessToken } from "@/composables/useAccessToken";
import { useToast } from "primevue";
import { TOAST_ERROR_TTL } from "@/vars";

const INITIAL_KEY_COMMENT = "Original";

const formId = ref<FormId>();
const clientKeyId = ref<ClientKeyId>();
const secretLinkKey = ref<SecretLinkKey>();

const toast = useToast();

const formSubmitted = computed(
  () =>
    formId.value !== undefined &&
    clientKeyId.value !== undefined &&
    secretLinkKey.value !== undefined,
);

const submitForm = async (values: FormValues, resetForm: () => void) => {
  const newSecretLinkKey = generateSecretLinkKey();
  const newPrimaryKeypair = generatePrimaryKeypair();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  let response;

  try {
    response = await api.postForm({
      publicPrimaryKey: newPrimaryKeypair.public,
      publicSigningKey: derivedKeys.publicSigningKey,
      orgName: values.title,
      description: values.description,
      contactMethods: values.contactMethods,
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

    return;
  }

  formId.value = response.formId;
  clientKeyId.value = response.clientKeyId;
  secretLinkKey.value = newSecretLinkKey;

  const accessToken = await getAccessToken(
    response.formId,
    response.clientKeyId,
    derivedKeys.privateSigningKey,
  );

  const encryptedComment = sealKeyComment(
    encodeUtf8(INITIAL_KEY_COMMENT),
    newPrimaryKeypair.public,
  );
  const wrappedPrivatePrimaryKey = wrapPrivatePrimaryKey(
    newPrimaryKeypair.private,
    derivedKeys.secretWrappingKey,
  );

  try {
    await api.patchKey({
      formId: formId.value,
      clientKeyId: clientKeyId.value,
      wrappedPrivatePrimaryKey,
      encryptedComment,
      accessToken,
    });
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to create group",
      detail: "Something unexpected happened.",
      life: TOAST_ERROR_TTL,
    });
  }

  resetForm();
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
    <FormBuilder v-if="!formSubmitted" @submit="submitForm" />
    <FormAdmonitions
      v-if="formSubmitted"
      :formId="formId"
      :clientKeyId="clientKeyId"
      :secretLinkKey="secretLinkKey"
    />
  </main>
</template>

<style scoped></style>
