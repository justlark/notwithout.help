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
  protectSecretLinkKey,
  type MaybeProtectedSecretLinkKey,
  type PrimaryKeyFingerprint,
  generatePrimaryKeyFingerprint,
} from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { getAccessToken } from "@/composables/useAccessToken";
import { useToast } from "primevue";
import { TOAST_ERROR_TTL } from "@/vars";

const INITIAL_KEY_COMMENT = "Original";

const formId = ref<FormId>();
const clientKeyId = ref<ClientKeyId>();
const secretLinkKey = ref<MaybeProtectedSecretLinkKey>();
const primaryKeyFingerprint = ref<PrimaryKeyFingerprint>();

const toast = useToast();

const formSubmitted = computed(
  () =>
    formId.value !== undefined &&
    clientKeyId.value !== undefined &&
    secretLinkKey.value !== undefined,
);

const submitForm = async (values: FormValues, resetForm: () => void) => {
  const newSecretLinkKey = await generateSecretLinkKey();
  const newPrimaryKeypair = await generatePrimaryKeypair();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  let response;

  try {
    response = await api.postForm({
      publicPrimaryKey: newPrimaryKeypair.public,
      publicSigningKey: derivedKeys.publicSigningKey,
      orgName: values.title,
      description: values.description,
      contactMethods: values.contactMethods,
      expirationDate: values.expirationDate,
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

    return;
  }

  const accessToken = await getAccessToken(
    response.formId,
    response.clientKeyId,
    derivedKeys.privateSigningKey,
  );

  const encryptedComment = await sealKeyComment(
    encodeUtf8(INITIAL_KEY_COMMENT),
    newPrimaryKeypair.public,
  );

  const wrappedPrivatePrimaryKey = await wrapPrivatePrimaryKey(
    newPrimaryKeypair.private,
    derivedKeys.secretWrappingKey,
  );

  let newMaybeProtectedSecretLinkKey: MaybeProtectedSecretLinkKey;

  try {
    await api.patchKey({
      formId: response.formId,
      clientKeyId: response.clientKeyId,
      wrappedPrivatePrimaryKey,
      encryptedComment,
      accessToken,
    });

    if (values.password) {
      const { key, params } = await protectSecretLinkKey(newSecretLinkKey, values.password);

      await api.postPassword({
        formId: response.formId,
        clientKeyId: response.clientKeyId,
        salt: params.salt,
        nonce: params.nonce,
        accessToken: accessToken,
      });

      newMaybeProtectedSecretLinkKey = key;
    } else {
      newMaybeProtectedSecretLinkKey = newSecretLinkKey;
    }
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to create group",
      detail: "Something unexpected happened.",
      life: TOAST_ERROR_TTL,
    });

    return;
  }

  formId.value = response.formId;
  clientKeyId.value = response.clientKeyId;
  secretLinkKey.value = newMaybeProtectedSecretLinkKey;
  primaryKeyFingerprint.value = await generatePrimaryKeyFingerprint(newPrimaryKeypair.public);

  resetForm();
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <FormBuilder
      v-if="!formSubmitted"
      storage-key="template"
      @submit="submitForm"
      aria-labelledby="main-heading"
      show-password-input
    >
      <template #lead>
        <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
      </template>
    </FormBuilder>
    <FormAdmonitions
      v-if="
        formId !== undefined &&
        clientKeyId !== undefined &&
        secretLinkKey !== undefined &&
        primaryKeyFingerprint !== undefined
      "
      :form-id="formId"
      :client-key-id="clientKeyId"
      :secret-link-key="secretLinkKey"
      :primary-key-fingerprint="primaryKeyFingerprint"
    />
  </main>
</template>

<style scoped></style>
