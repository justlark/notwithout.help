<script setup lang="ts">
import api, { type SubmissionBody } from "@/api";
import { useForm, useLink } from "@/auth";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import { sealSubmissionBody } from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { TOAST_TTL } from "@/vars";
import { useToast } from "primevue";
import Toast from "primevue/toast";

const toast = useToast();

const { formId } = useLink();
const { orgName, description, contactMethods, publicPrimaryKey } = useForm();

const postSubmission = async (values: FormValues) => {
  if (!formId.value || !publicPrimaryKey.value) {
    return;
  }

  const submissionBody: SubmissionBody = {
    name: values.name,
    contact: values.contact,
    contact_method: values.contactMethod,
  };

  const encryptedBody = sealSubmissionBody(
    encodeUtf8(JSON.stringify(submissionBody)),
    publicPrimaryKey.value,
  );

  api.postSubmission({ formId: formId.value, encryptedBody });

  toast.add({
    severity: "success",
    summary: "Response submitted",
    detail: "Your response has been sent to the organizers.",
    life: TOAST_TTL,
  });
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">{{ orgName }}</h1>
    <p class="text-jusitfy max-w-xl mx-auto">{{ description }}</p>
    <ResponseForm
      @submit="postSubmission"
      v-if="contactMethods"
      :contact-methods="contactMethods"
    />
    <Toast position="bottom-center" />
  </main>
</template>

<style scoped></style>
