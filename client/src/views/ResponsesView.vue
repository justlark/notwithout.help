<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import ConfirmDialog from "primevue/confirmdialog";
import SecretLinkList from "@/components/SecretLinkList.vue";
import Card from "primevue/card";
import ErrorCard from "@/components/ErrorCard.vue";
import Button from "primevue/button";
import { TOAST_ERROR_TTL, TOAST_INFO_TTL, newShareLink } from "@/vars";
import { computed, ref, watchEffect } from "vue";
import { decodeUtf8, formatDate } from "@/encoding";
import { unsealSubmissionBody } from "@/crypto";
import api, { ApiError, type SubmissionBody } from "@/api";
import { useConfirm, useToast } from "primevue";
import { returnsError, isDone, allDone } from "@/types";
import { useRouter } from "vue-router";
import useSecretLink from "@/composables/useSecretLink";
import useAccessToken from "@/composables/useAccessToken";
import usePrivatePrimaryKey from "@/composables/usePrivatePrimaryKey";
import useForm from "@/composables/useForm";
import { stringify as toCsv } from "csv-stringify/browser/esm/sync";

export interface Submission {
  name: string;
  contact: string;
  contactMethod: string;
  createdAt: Date;
}

const submissions = ref<Array<Submission>>([]);
const noSubmissions = ref(false);

const router = useRouter();
const confirm = useConfirm();
const toast = useToast();

const { formId, clientKeyId, secretLinkKey } = useSecretLink();
const accessToken = useAccessToken();
const privatePrimaryKey = usePrivatePrimaryKey();
const form = useForm();

const isNotFound = computed(() => {
  return returnsError(
    ["unauthorized", "forbidden", "not-found"],
    accessToken,
    privatePrimaryKey,
    form,
  );
});
const isLoaded = computed(() => allDone(accessToken, privatePrimaryKey, form));
const isReadOnly = computed(() => !isDone(accessToken) || accessToken.value.value.role === "read");

const isMenuExpanded = ref(false);

const csvFileObjectUrl = computed(() => {
  const headers = ["Name", "Contact", "Contact method", "Submitted at"];
  const data = submissions.value.map((submission) => [
    submission.name,
    submission.contact,
    submission.contactMethod,
    submission.createdAt.toISOString(),
  ]);
  const rows = [headers, ...data];
  const csv = toCsv(rows);
  const blob = new Blob([csv], { type: "text/csv" });
  return URL.createObjectURL(blob);
});

const doDelete = async () => {
  if (!isDone(accessToken)) {
    return;
  }

  const { token } = accessToken.value.value;

  try {
    await api.deleteForm({ formId: formId.value, accessToken: token });
  } catch (error) {
    if (error instanceof ApiError && error.kind === "forbidden") {
      toast.add({
        severity: "error",
        summary: "Failed to delete group",
        detail: "You do not have permission to do this.",
        life: TOAST_ERROR_TTL,
      });
    } else {
      toast.add({
        severity: "error",
        summary: "Failed to delete group",
        detail: "Something unexpected happened.",
        life: TOAST_ERROR_TTL,
      });
    }

    return;
  }

  submissions.value = [];

  toast.add({
    severity: "success",
    summary: "Group deleted",
    detail: "Your group and all responses have been permanently deleted.",
    life: TOAST_INFO_TTL,
  });

  router.push({ path: "/" });
};

const deleteForm = () => {
  confirm.require({
    header: "Delete this group?",
    message:
      "Do you want to permanently delete this group and all submissions? Make sure you export your data first; this cannot be undone.",
    icon: "pi pi-exclamation-triangle",
    acceptProps: {
      label: "Delete",
      severity: "danger",
    },
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    accept: doDelete,
  });
};

watchEffect(async () => {
  submissions.value = [];

  // Ensure all of these are tracked by Vue before the first await boundary.
  if (!isDone(accessToken) || !isDone(privatePrimaryKey) || !isDone(form)) {
    return;
  }

  const { token } = accessToken.value.value;
  const { publicPrimaryKey } = form.value.value;

  const encryptedSubmissions = await api.getSubmissions({
    formId: formId.value,
    accessToken: token,
  });

  noSubmissions.value = encryptedSubmissions.length === 0;

  for (const { encryptedBody, createdAt } of encryptedSubmissions) {
    const encodedSubmissionBody = await unsealSubmissionBody(
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
    <h1 id="main-heading" class="text-center mb-6">View responses</h1>

    <ErrorCard
      v-if="isNotFound"
      title="Not found"
      message="Either this is an invalid link, the group has been deleted, or you don't have access to it anymore."
    />

    <div class="xl:w-3/4 mx-auto" v-else>
      <div
        v-if="isDone(form) && isLoaded"
        class="text-center flex items-center sm:items-baseline flex-col sm:flex-row sm:justify-between mb-6 sm:mb-2"
      >
        <h2>{{ form.value.orgName }}</h2>
        <span v-if="form.value.expirationDate" class="text-muted-color">
          <i class="pi pi-calendar"></i>
          Expires
          <time :datetime="form.value.expirationDate.toISOString()">{{
            formatDate(form.value.expirationDate)
          }}</time>
        </span>
      </div>
      <div class="flex flex-col gap-4">
        <SecretLinkList
          v-if="!isReadOnly"
          :form-id="formId"
          :client-key-id="clientKeyId"
          :secret-link-key="secretLinkKey"
        />
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
          <Card v-if="noSubmissions" class="w-full">
            <template #content>
              <div class="flex gap-5 items-center text-muted-color">
                <i class="pi pi-info-circle !text-4xl"></i>
                <div class="flex flex-col gap-1">
                  <span class="text-2xl">No responses yet</span>
                  <span>Refresh the page to check again.</span>
                </div>
              </div>
            </template>
          </Card>
        </div>
      </div>
      <div class="xl:sticky bottom-6">
        <div
          class="flex flex-col gap-6 fixed xl:absolute xl:translate-x-full bottom-6 xl:bottom-0 right-6 xl:-right-6"
        >
          <div
            id="action-menu"
            :class="{
              flex: true,
              'flex-col': true,
              'gap-3': true,
              hidden: !isMenuExpanded,
              'xl:flex': true,
            }"
          >
            <Button
              class="!justify-start"
              as="a"
              :href="newShareLink(formId)"
              target="_blank"
              label="Share"
              severity="secondary"
              icon="pi pi-external-link"
            />
            <Button
              v-if="submissions.length > 0"
              class="!justify-start"
              as="a"
              :href="csvFileObjectUrl"
              download="responses.csv"
              label="Export"
              severity="secondary"
              icon="pi pi-download"
            />
            <Button
              v-if="isLoaded && !isReadOnly"
              class="!justify-start"
              label="Edit"
              severity="secondary"
              icon="pi pi-pen-to-square"
            />
            <Button
              v-if="isLoaded && !isReadOnly"
              @click="deleteForm"
              class="!justify-start"
              label="Delete"
              severity="danger"
              icon="pi pi-trash"
            />
          </div>
          <Button
            @click="isMenuExpanded = !isMenuExpanded"
            class="!jusitfy-start xl:!hidden"
            label="Menu"
            :severity="isMenuExpanded ? 'secondary' : 'primary'"
            :icon="isMenuExpanded ? 'pi pi-times' : 'pi pi-bars'"
            :aria-expanded="isMenuExpanded"
            aria-controls="action-menu"
          />
        </div>
      </div>
    </div>
    <ConfirmDialog class="max-w-xl mx-6" />
  </main>
</template>

<style scoped></style>
