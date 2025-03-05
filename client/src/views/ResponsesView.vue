<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import ConfirmDialog from "primevue/confirmdialog";
import SecretLinkList from "@/components/SecretLinkList.vue";
import Card from "primevue/card";
import Dialog from "primevue/dialog";
import ShareLinkAdmonition from "@/components/ShareLinkAdmonition.vue";
import Skeleton from "primevue/skeleton";
import ErrorCard from "@/components/ErrorCard.vue";
import Button from "primevue/button";
import { TOAST_ERROR_TTL, TOAST_INFO_TTL, newEditLink } from "@/vars";
import { computed, ref, watch, watchEffect } from "vue";
import { datetimeToCsvFormat, decodeUtf8, formatDate } from "@/encoding";
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
  comment: string;
  roles: Array<string>;
  createdAt: Date;
}

const submissions = ref<Array<Submission>>([]);
const submissionsCount = computed(() => submissions.value.length);
const submissionsState = ref<"loading" | "done" | "none" | "reloading">("loading");

const router = useRouter();
const confirm = useConfirm();
const toast = useToast();

const secretLinkParts = useSecretLink();
const accessToken = useAccessToken();
const privatePrimaryKey = usePrivatePrimaryKey();
const form = useForm();
const editLink = computed(() =>
  isDone(secretLinkParts)
    ? newEditLink(
        secretLinkParts.value.value.formId,
        secretLinkParts.value.value.clientKeyId,
        secretLinkParts.value.value.maybeProtectedSecretLinkKey,
      )
    : undefined,
);

const isNotFound = computed(() => {
  return returnsError(
    ["bad-request", "unauthorized", "forbidden", "not-found"],
    accessToken,
    privatePrimaryKey,
    form,
  );
});
const isLoaded = computed(() => allDone(accessToken, privatePrimaryKey, form));
const isReadOnly = computed(() => !isDone(accessToken) || accessToken.value.value.role === "read");

const isMenuExpanded = ref(false);
const isShareLinkModalVisible = ref(false);
const isReloadButtonSpinning = ref(false);

watch([submissionsState], () => {
  if (submissionsState.value === "reloading") {
    isReloadButtonSpinning.value = true;
    setTimeout(() => {
      isReloadButtonSpinning.value = false;
    }, 1000);
  }
});

const csvFileObjectUrl = computed(() => {
  const headers = ["Submitted at (UTC)", "Name", "Contact", "Contact method", "Comment", "Roles"];
  const data = submissions.value.map((submission) => [
    datetimeToCsvFormat(submission.createdAt),
    submission.name,
    submission.contact,
    submission.contactMethod,
    submission.comment,
    submission.roles.join(", "),
  ]);
  const rows = [headers, ...data];
  const csv = toCsv(rows);
  const blob = new Blob([csv], { type: "text/csv" });
  return URL.createObjectURL(blob);
});

const doDelete = async () => {
  if (!isDone(accessToken) || !isDone(secretLinkParts)) {
    return;
  }

  const { token } = accessToken.value.value;
  const { formId } = secretLinkParts.value.value;

  try {
    await api.deleteForm({ formId: formId, accessToken: token });
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

const refreshSubmissions = () => {
  submissionsState.value = "reloading";
};

const roleNamesById = computed(() => {
  if (!isDone(form)) {
    return new Map<string, string>();
  }

  return new Map(form.value.value.roles.map((role) => [role.id, role.name]));
});

watchEffect(async () => {
  // Make sure this is tracked by the `watchEffect` before the first await boundary.
  //
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  submissionsState.value;

  // Ensure all of these are tracked by Vue before the first await boundary.
  if (
    !isDone(accessToken) ||
    !isDone(privatePrimaryKey) ||
    !isDone(form) ||
    !isDone(secretLinkParts)
  ) {
    return;
  }

  const { token } = accessToken.value.value;
  const { publicPrimaryKey } = form.value.value;
  const { formId } = secretLinkParts.value.value;

  const encryptedSubmissions = await api.getSubmissions({
    formId: formId,
    accessToken: token,
  });

  if (encryptedSubmissions.length === 0) {
    submissionsState.value = "none";
    return;
  }

  const newSubmissions = [];

  for (const { encryptedBody, createdAt } of encryptedSubmissions) {
    const encodedSubmissionBody = await unsealSubmissionBody(
      encryptedBody,
      publicPrimaryKey,
      privatePrimaryKey.value.value,
    );

    const submissionBody: SubmissionBody = JSON.parse(decodeUtf8(encodedSubmissionBody));

    newSubmissions.push({
      name: submissionBody.name,
      contact: submissionBody.contact,
      contactMethod: submissionBody.contact_method,
      comment: submissionBody.comment ?? "",
      roles:
        submissionBody.roles
          ?.map((roleId) => roleNamesById.value.get(roleId))
          .filter((roleName): roleName is string => roleName !== undefined) ?? [],
      createdAt: new Date(createdAt),
    });
  }

  submissions.value = newSubmissions;
  submissionsState.value = "done";
});
</script>
<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center flex gap-2 justify-center items-baseline mb-6">
      <span>View responses</span>
      <span v-if="submissionsCount" class="text-2xl text-muted-color">
        ({{ submissionsCount }})
      </span>
    </h1>

    <ErrorCard
      v-if="isNotFound"
      title="Not found"
      message="Either this is an invalid link, the group has been deleted, or you don't have access to it anymore."
    />

    <!--
      Include a bottom margin so the user can scroll far enough that the
      floating action buttons don't overlap the content.
    -->
    <div class="xl:w-3/4 mx-auto mb-20 xl:mb-0" v-else>
      <div
        class="text-center flex items-center sm:items-baseline flex-col sm:flex-row sm:justify-between mb-6 sm:mb-2"
      >
        <h2 v-if="isDone(form)">{{ form.value.orgName }}</h2>
        <Skeleton v-else width="10rem" height="2.5rem" />
        <span v-if="isDone(form) && form.value.expirationDate" class="text-muted-color">
          <i class="pi pi-calendar"></i>
          Expires
          <time :datetime="form.value.expirationDate.toISOString()">{{
            formatDate(form.value.expirationDate)
          }}</time>
        </span>
      </div>
      <div class="flex flex-col gap-4">
        <SecretLinkList
          v-if="!isReadOnly && isDone(secretLinkParts)"
          :form-id="secretLinkParts.value.formId"
          :client-key-id="secretLinkParts.value.clientKeyId"
          :secret-link-key="secretLinkParts.value.maybeProtectedSecretLinkKey"
        />
        <div class="xl:sticky top-6">
          <div
            v-if="isLoaded && submissionsState !== 'loading'"
            class="fixed xl:absolute xl:-translate-x-full xl:translate-y-1/2 bottom-6 xl:bottom-auto xl:top-full xl:-left-6"
          >
            <Button
              @click="refreshSubmissions"
              :loading="isReloadButtonSpinning"
              icon="pi pi-refresh"
              size="large"
              severity="secondary"
              aria-label="Refresh"
              raised
              rounded
            />
          </div>
        </div>
        <div class="flex flex-col gap-4 items-center">
          <FormResponse
            v-for="(submission, index) in submissions"
            :key="index"
            :index="index.toString()"
            class="w-full"
            :name="submission.name"
            :contact="submission.contact"
            :contactMethod="submission.contactMethod"
            :roles="submission.roles"
            :comment="submission.comment"
            :createdAt="submission.createdAt"
          />
          <Card
            v-if="
              submissionsState === 'none' ||
              (submissionsCount === 0 && submissionsState === 'reloading')
            "
            class="w-full"
          >
            <template #content>
              <div class="flex gap-5 items-center text-muted-color">
                <i class="pi pi-info-circle !text-4xl"></i>
                <div class="flex flex-col gap-1">
                  <span class="text-2xl">No responses yet</span>
                  <span>Click the refresh button to check again.</span>
                </div>
              </div>
            </template>
          </Card>
          <Skeleton v-if="submissionsState === 'loading'" height="6rem" />
          <Skeleton v-if="submissionsState === 'loading'" height="6rem" />
        </div>
      </div>
      <div class="xl:sticky bottom-6">
        <div
          v-if="isLoaded && submissionsState !== 'loading'"
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
              @click="isShareLinkModalVisible = true"
              label="Share"
              severity="secondary"
              icon="pi pi-share-alt"
              aria-controls="share-link-modal"
              :aria-expanded="isShareLinkModalVisible"
              raised
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
              raised
            />
            <Button
              v-if="!isReadOnly && editLink !== undefined"
              class="!justify-start"
              as="router-link"
              :to="{ path: editLink.pathname, hash: editLink.hash }"
              label="Edit"
              severity="secondary"
              icon="pi pi-pen-to-square"
              raised
            />
            <Button
              v-if="!isReadOnly"
              @click="deleteForm"
              class="!justify-start"
              label="Delete"
              severity="danger"
              icon="pi pi-trash"
              raised
            />
          </div>
          <Button
            @click="isMenuExpanded = !isMenuExpanded"
            class="!jusitfy-start xl:!hidden"
            label="Menu"
            :severity="isMenuExpanded ? 'secondary' : 'primary'"
            :icon="isMenuExpanded ? 'pi pi-times' : 'pi pi-bars'"
            raised
            :aria-expanded="isMenuExpanded"
            aria-controls="action-menu"
          />
        </div>
      </div>
    </div>
    <Dialog
      id="share-link-modal"
      class="p-2 mx-4"
      v-model:visible="isShareLinkModalVisible"
      modal
      aria-labelledby="share-link-modal-name"
    >
      <template #header>
        <span class="flex gap-3 text-xl items-center">
          <i class="pi pi-share-alt"></i>
          <strong id="share-link-modal-name">Share this link</strong>
        </span>
      </template>
      <ShareLinkAdmonition v-if="isDone(secretLinkParts)" :form-id="secretLinkParts.value.formId" />
    </Dialog>
    <ConfirmDialog class="max-w-xl mx-6" />
  </main>
</template>

<style scoped></style>
