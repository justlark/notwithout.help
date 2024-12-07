<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import Toast from "primevue/toast";
import SecretLinkList from "@/components/SecretLinkList.vue";
import Button from "primevue/button";
import type { ContactMethodCode } from "@/vars";
import { computed, ref, watchEffect } from "vue";
import { useRoute } from "vue-router";
import { decodeUtf8, parseSecretLinkFragment } from "@/encoding";
import { deriveKeys, unsealSubmissionBody, unwrapPrivatePrimaryKey } from "@/crypto";
import { getAccessToken } from "@/auth";
import api, { type SubmissionBody } from "@/api";

export interface Submission {
  name: string;
  contact: string;
  contactMethod: ContactMethodCode;
  createdAt: Date;
}

const submissions = ref<Array<Submission>>([]);

const route = useRoute();

const secretLinkParts = computed(() => parseSecretLinkFragment(route.hash));

const formData = computed(async () => await api.getForm({ formId: secretLinkParts.value.formId }));

const derivedKeys = computed(async () => await deriveKeys(secretLinkParts.value.secretLinkKey));

const privatePrimaryKey = computed(async () => {
  const wrappedPrivatePrimaryKey = await api.getKey({
    formId: secretLinkParts.value.formId,
    clientKeyId: secretLinkParts.value.clientKeyId,
    accessToken: await accessToken.value,
  });

  const { secretWrappingKey } = await derivedKeys.value;

  return unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey);
});

const accessToken = computed(async () => {
  const { privateSigningKey } = await derivedKeys.value;

  return await getAccessToken(
    secretLinkParts.value.formId,
    secretLinkParts.value.clientKeyId,
    privateSigningKey,
  );
});

const computedSubmissions = computed(async () => {
  const submissions = await api.getSubmissions({
    formId: secretLinkParts.value.formId,
    accessToken: await accessToken.value,
  });

  return await Promise.all(
    submissions.map(async ({ encryptedBody, createdAt }) => {
      const { publicPrimaryKey } = await formData.value;

      const encodedSubmissionBody = unsealSubmissionBody(
        encryptedBody,
        publicPrimaryKey,
        await privatePrimaryKey.value,
      );

      const submissionBody: SubmissionBody = JSON.parse(decodeUtf8(encodedSubmissionBody));

      return {
        name: submissionBody.name,
        contact: submissionBody.contact,
        contactMethod: submissionBody.contact_method,
        createdAt: new Date(createdAt),
      };
    }),
  );
});

const deleteForm = async () => {
  api.deleteForm({ formId: secretLinkParts.value.formId, accessToken: await accessToken.value });
};

watchEffect(async () => {
  submissions.value = await computedSubmissions.value;
});
</script>
<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">View responses</h1>
    <div class="xl:w-3/4 mx-auto">
      <div class="flex flex-col gap-8">
        <SecretLinkList class="self-center w-full" />
        <div class="flex flex-col gap-4 items-center">
          <FormResponse
            v-for="(submission, index) in submissions"
            :key="index"
            :index="index.toString()"
            class="w-full"
            :name="submission.name"
            :contact="submission.contact"
            :contactMethod="submission.contactMethod"
            :createdAt="submission.createdAt"
          />
        </div>
      </div>
      <div class="xl:sticky bottom-6">
        <div
          class="flex flex-col gap-3 fixed xl:absolute xl:translate-x-full bottom-6 xl:bottom-0 right-6 xl:-right-6"
        >
          <Button
            class="!justify-start"
            label="Export"
            severity="secondary"
            icon="pi pi-download"
          />
          <Button
            class="!justify-start"
            label="Edit"
            severity="secondary"
            icon="pi pi-pen-to-square"
          />
          <Button
            @click="deleteForm"
            class="!justify-start"
            label="Delete"
            severity="danger"
            icon="pi pi-trash"
          />
        </div>
      </div>
    </div>
    <Toast position="bottom-center" />
  </main>
</template>

<style scoped></style>
