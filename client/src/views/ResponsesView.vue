<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import ConfirmDialog from "primevue/confirmdialog";
import SecretLinkList from "@/components/SecretLinkList.vue";
import ErrorCard from "@/components/ErrorCard.vue";
import Button from "primevue/button";
import { TOAST_INFO_TTL, type ContactMethodCode } from "@/vars";
import { computed, ref, watchEffect } from "vue";
import { decodeUtf8 } from "@/encoding";
import { unsealSubmissionBody } from "@/crypto";
import { useAccessToken, useForm, usePrivatePrimaryKey, useSecretLink } from "@/auth";
import api, { type SubmissionBody } from "@/api";
import { useConfirm, useToast } from "primevue";
import { loadableRef, returnsError, isDone } from "@/types";
import { useRouter } from "vue-router";

export interface Submission {
  name: string;
  contact: string;
  contactMethod: ContactMethodCode;
  createdAt: Date;
}

const submissions = ref<Array<Submission>>([]);

const router = useRouter();
const confirm = useConfirm();
const toast = useToast();

const secretLinkParts = useSecretLink();
const accessToken = useAccessToken();
const privatePrimaryKey = usePrivatePrimaryKey(loadableRef(accessToken));
const form = useForm();

const isNotFound = computed(() => {
  return returnsError(["unauthorized", "not-found"], accessToken, privatePrimaryKey, form);
});

const deleteForm = () => {
  confirm.require({
    header: "Delete this form?",
    message:
      "Do you want to permanently delete this form and all submissions? Make sure you export your data first; this cannot be undone.",
    icon: "pi pi-info-circle",
    acceptProps: {
      label: "Delete",
      severity: "danger",
    },
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    accept: async () => {
      const { formId } = secretLinkParts.value;

      if (!isDone(accessToken)) {
        return;
      }

      await api.deleteForm({ formId: formId, accessToken: accessToken.value.value });

      submissions.value = [];

      toast.add({
        severity: "success",
        summary: "Form deleted",
        detail: "Your form and all responses have been permanently deleted.",
        life: TOAST_INFO_TTL,
      });

      router.push({ path: "/" });
    },
  });
};

watchEffect(async () => {
  submissions.value = [];

  // Ensure all of these are tracked by Vue before the first await boundary.
  if (!isDone(accessToken) || !isDone(privatePrimaryKey) || !isDone(form)) {
    return;
  }

  const { formId } = secretLinkParts.value;
  const { publicPrimaryKey } = form.value.value;

  const encryptedSubmissions = await api.getSubmissions({
    formId: formId,
    accessToken: accessToken.value.value,
  });

  for (const { encryptedBody, createdAt } of encryptedSubmissions) {
    const encodedSubmissionBody = unsealSubmissionBody(
      encryptedBody,
      publicPrimaryKey,
      privatePrimaryKey.value.value,
    );

    const submissionBody: SubmissionBody = JSON.parse(decodeUtf8(encodedSubmissionBody));

    submissions.value.push({
      name: submissionBody.name,
      contact: submissionBody.contact,
      contactMethod: submissionBody.contact_method,
      createdAt: new Date(createdAt),
    });
  }
});
</script>
<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">View responses</h1>
    <ErrorCard
      v-if="isNotFound"
      title="Not found"
      message="Either this is an invalid link, the form has been deleted, or you don't have access to it anymore."
    />
    <div v-else class="xl:w-3/4 mx-auto">
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
    <ConfirmDialog class="max-w-xl mx-6" />
  </main>
</template>

<style scoped></style>
