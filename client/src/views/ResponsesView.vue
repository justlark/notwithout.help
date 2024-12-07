<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import Toast from "primevue/toast";
import SecretLinkList from "@/components/SecretLinkList.vue";
import Button from "primevue/button";
import type { ContactMethodCode } from "@/vars";
import { onBeforeMount, ref } from "vue";
import { useRoute } from "vue-router";
import { decodeUtf8, parseSecretLinkFragment } from "@/encoding";
import { deriveKeys, unsealSubmissionBody, unwrapPrivatePrimaryKey } from "@/crypto";
import { getAccessToken } from "@/auth";
import api, { type SubmissionBody } from "@/api";

export interface Response {
  name: string;
  contact: string;
  contactMethod: ContactMethodCode;
  createdAt: Date;
}

const route = useRoute();
const responses = ref<Array<Response>>([]);

onBeforeMount(async () => {
  const { formId, clientKeyId, secretLinkKey } = parseSecretLinkFragment(route.hash);

  const { publicPrimaryKey } = await api.getForm({ formId });
  const { privateSigningKey, secretWrappingKey } = await deriveKeys(secretLinkKey);

  const accessToken = await getAccessToken(formId, clientKeyId, privateSigningKey);

  const wrappedPrivatePrimaryKey = await api.getKey({ formId, clientKeyId, accessToken });
  const privatePrimaryKey = unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey);

  const submissions = await api.getSubmissions({ formId, accessToken });

  responses.value = submissions.map(({ encryptedBody, createdAt }) => {
    const encodedSubmissionBody = unsealSubmissionBody(
      encryptedBody,
      publicPrimaryKey,
      privatePrimaryKey,
    );

    const submissionBody: SubmissionBody = JSON.parse(decodeUtf8(encodedSubmissionBody));

    return {
      name: submissionBody.name,
      contact: submissionBody.contact,
      contactMethod: submissionBody.contact_method,
      createdAt: new Date(createdAt),
    };
  });
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
            v-for="(response, index) in responses"
            :key="index"
            :index="index.toString()"
            class="w-full"
            :name="response.name"
            :contact="response.contact"
            :contactMethod="response.contactMethod"
            :createdAt="response.createdAt"
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
          <Button class="!justify-start" label="Delete" severity="danger" icon="pi pi-trash" />
        </div>
      </div>
    </div>
    <Toast position="bottom-center" />
  </main>
</template>

<style scoped></style>
