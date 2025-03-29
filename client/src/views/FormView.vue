<script setup lang="ts">
import api, { ApiError, CURRENT_VERSION, type SubmissionBody } from "@/api";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import ErrorCard from "@/components/ErrorCard.vue";
import useForm from "@/composables/useForm";
import useLink from "@/composables/useLink";
import {
  generatePrimaryKeyFingerprint,
  primaryKeyFingerprintsAreEqual,
  sealSubmissionBody,
} from "@/crypto";
import { encodeUtf8 } from "@/encoding";
import { isDone, returnsError } from "@/types";
import { TOAST_ERROR_TTL, TOAST_INFO_TTL } from "@/vars";
import { useToast } from "primevue";
import { computed } from "vue";

const toast = useToast();

const shareLinkParts = useLink();
const form = useForm();

const isNotFound = computed(() => returnsError(["bad-request", "not-found"], form));
const linkIsMissingFingerprint = computed(() => returnsError("invalid", shareLinkParts));

const postSubmission = async (values: FormValues, resetForm: () => void) => {
  if (!isDone(form) || !isDone(shareLinkParts)) {
    return;
  }

  const { publicPrimaryKey } = form.value.value;
  const { formId, primaryKeyFingerprint } = shareLinkParts.value.value;

  const computedPrimaryKeyFingerprint = await generatePrimaryKeyFingerprint(publicPrimaryKey);

  if (!primaryKeyFingerprintsAreEqual(primaryKeyFingerprint, computedPrimaryKeyFingerprint)) {
    toast.add({
      severity: "error",
      summary: "Failed to submit response",
      detail:
        "The security fingerprint in the link you followed is not valid. This could mean you copied the link incorrectly.",
      life: TOAST_ERROR_TTL,
    });

    return;
  }

  const submissionBody: SubmissionBody = {
    version: CURRENT_VERSION,
    name: values.name,
    contact: values.contact,
    contact_method: values.contactMethod,
    roles: values.roles,
    comment: values.comment,
  };

  const encryptedBody = await sealSubmissionBody(
    encodeUtf8(JSON.stringify(submissionBody)),
    publicPrimaryKey,
  );

  try {
    await api.postSubmission({ formId, encryptedBody });
  } catch (error) {
    if (error instanceof ApiError && error.kind === "not-found") {
      toast.add({
        severity: "error",
        summary: "Failed to submit response",
        detail: "This group is no longer accepting responses.",
        life: TOAST_ERROR_TTL,
      });
    } else if (error instanceof ApiError && error.kind === "content-too-large") {
      toast.add({
        severity: "error",
        summary: "Failed to submit response",
        detail: "Your response is too large. Cut down the number of characters and try again.",
        life: TOAST_ERROR_TTL,
      });
    } else {
      toast.add({
        severity: "error",
        summary: "Failed to submit response",
        detail: "Something unexpected happened.",
        life: TOAST_ERROR_TTL,
      });
    }

    return;
  }

  toast.add({
    severity: "success",
    summary: "Response submitted",
    detail: "Your response has been sent to the organizers.",
    life: TOAST_INFO_TTL,
  });

  resetForm();
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <ErrorCard
      v-if="isNotFound"
      title="Not found"
      message="Either this is an invalid link, or the group is no longer accepting responses."
    />
    <ErrorCard
      v-if="linkIsMissingFingerprint"
      title="Invalid link"
      message="This link is no longer valid. Please ask the organizers for a new one."
    />
    <div v-else-if="isDone(form) && isDone(shareLinkParts)">
      <ResponseForm
        @submit="postSubmission"
        :storage-key="`form:${shareLinkParts.value.formId}`"
        :contact-methods="[...form.value.contactMethods]"
        aria-labelledby="main-heading"
        :roles="form.value.roles.map((role) => ({ ...role, details: [...role.details] }))"
      >
        <template #lead>
          <h1 id="main-heading" class="text-center mb-10">
            {{ form.value.orgName }}
          </h1>
          <p class="text-jusitfy max-w-xl mx-auto">{{ form.value.description }}</p>
        </template>
      </ResponseForm>
    </div>
  </main>
</template>

<style scoped></style>
