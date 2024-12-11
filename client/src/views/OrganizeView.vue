<script setup lang="ts">
import { computed, ref } from "vue";
import FormAdmonitions from "@/components/FormAdmonitions.vue";
import FormBuilder, { type FormValues } from "@/components/FormBuilder.vue";
import api from "@/api";
import {
  deriveKeys,
  encryptKeyComment,
  generatePrimaryKeypair,
  generateSecretLinkKey,
  wrapPrivatePrimaryKey,
  type ClientKeyId,
  type FormId,
  type SecretLinkKey,
} from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { getAccessToken } from "@/composables/useAccessToken";

const INITIAL_KEY_COMMENT = "Original";

const formId = ref<FormId>();
const clientKeyId = ref<ClientKeyId>();
const secretLinkKey = ref<SecretLinkKey>();

const formSubmitted = computed(
  () =>
    formId.value !== undefined &&
    clientKeyId.value !== undefined &&
    secretLinkKey.value !== undefined,
);

const submitForm = async (values: FormValues) => {
  const newSecretLinkKey = generateSecretLinkKey();
  const newPrimaryKeypair = generatePrimaryKeypair();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  const response = await api.postForm({
    publicPrimaryKey: newPrimaryKeypair.public,
    publicSigningKey: derivedKeys.publicSigningKey,
    orgName: values.title,
    description: values.description,
    contactMethods: values.contactMethods,
  });

  formId.value = response.formId;
  clientKeyId.value = response.clientKeyId;
  secretLinkKey.value = newSecretLinkKey;

  const accessToken = await getAccessToken(
    response.formId,
    response.clientKeyId,
    derivedKeys.privateSigningKey,
  );

  const encryptedComment = encryptKeyComment(
    encodeUtf8(INITIAL_KEY_COMMENT),
    derivedKeys.secretWrappingKey,
  );
  const wrappedPrivatePrimaryKey = wrapPrivatePrimaryKey(
    newPrimaryKeypair.private,
    derivedKeys.secretWrappingKey,
  );

  await api.patchKey({
    formId: formId.value,
    clientKeyId: clientKeyId.value,
    wrappedPrivatePrimaryKey,
    encryptedComment,
    accessToken,
  });
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
