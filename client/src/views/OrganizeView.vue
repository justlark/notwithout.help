<script setup lang="ts">
import { ref } from "vue";
import FormLinks from "@/components/FormLinks.vue";
import FormBuilder, { type FormValues } from "@/components/FormBuilder.vue";
import type { ClientKeyId, FormId } from "@/types";
import api from "@/api";
import {
  deriveKeys,
  generatePrimaryKeypair,
  generateSecretLinkKey,
  type SecretLinkKey,
} from "@/crypto";

const formId = ref<FormId>();
const clientKeyId = ref<ClientKeyId>();
const secretLinkKey = ref<SecretLinkKey>();

const formSubmitted = ref(false);

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

  formSubmitted.value = true;
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
    <FormBuilder v-if="!formSubmitted" @submit="submitForm" />
    <FormLinks
      v-if="formSubmitted"
      :formId="formId"
      :clientKeyId="clientKeyId"
      :secretLinkKey="secretLinkKey"
    />
  </main>
</template>

<style scoped></style>
