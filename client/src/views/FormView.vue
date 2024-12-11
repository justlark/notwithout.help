<script setup lang="ts">
import api, { type SubmissionBody } from "@/api";
import { useForm, useLink } from "@/auth";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import { sealSubmissionBody } from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { isDone } from "@/types";
import { TOAST_INFO_TTL } from "@/vars";
import { useToast } from "primevue";

const toast = useToast();

const shareLinkParts = useLink();
const form = useForm();

const postSubmission = async (values: FormValues) => {
  if (!isDone(form)) {
    return;
  }

  const { formId } = shareLinkParts.value;
  const { publicPrimaryKey } = form.value.value;

  const submissionBody: SubmissionBody = {
    name: values.name,
    contact: values.contact,
    contact_method: values.contactMethod,
  };

  const encryptedBody = sealSubmissionBody(
    encodeUtf8(JSON.stringify(submissionBody)),
    publicPrimaryKey,
  );

  api.postSubmission({ formId, encryptedBody });

  toast.add({
    severity: "success",
    summary: "Response submitted",
    detail: "Your response has been sent to the organizers.",
    life: TOAST_INFO_TTL,
  });
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <div v-if="isDone(form)">
      <h1 id="main-heading" v-if="isDone(form)" class="text-center mb-10">
        {{ form.value.orgName }}
      </h1>
      <p class="text-jusitfy max-w-xl mx-auto">{{ form.value.description }}</p>
      <ResponseForm @submit="postSubmission" :contact-methods="form.value.contactMethods" />
    </div>
  </main>
</template>

<style scoped></style>
