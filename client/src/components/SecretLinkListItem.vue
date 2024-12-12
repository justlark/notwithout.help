<script setup lang="ts">
import api, { type AccessRole } from "@/api";
import useAccessToken from "@/composables/useAccessToken";
import type { ClientKeyId, FormId } from "@/crypto";
import { formatDateTime } from "@/encoding";
import { isDone } from "@/types";
import { TOAST_ERROR_TTL, TOAST_INFO_TTL } from "@/vars";
import { useConfirm, useToast } from "primevue";
import Button from "primevue/button";
import Tag from "primevue/tag";
import { computed } from "vue";

const props = defineProps<{
  index: number;
  comment: string;
  formId: FormId;
  clientKeyId: ClientKeyId;
  activeClientKeyId: ClientKeyId;
  accessedAt: Date | undefined;
  role: AccessRole;
  count: number;
}>();

type Emits = {
  (eventName: "revoke", index: number): void;
};

const emit = defineEmits<Emits>();

const toast = useToast();
const confirm = useConfirm();
const accessToken = useAccessToken();

const isReadOnly = computed(() => props.role === "read");
const isCurrentUser = computed(() => props.clientKeyId === props.activeClientKeyId);
const isOnlyRemainingLink = computed(() => props.count === 1);

const doRevoke = async () => {
  if (!isDone(accessToken)) {
    return;
  }

  const { token } = accessToken.value.value;

  try {
    await api.deleteKey({
      formId: props.formId,
      clientKeyId: props.clientKeyId,
      accessToken: token,
    });
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to revoke secret link",
      detail: "Something unexpected happened.",
      life: TOAST_ERROR_TTL,
    });

    return;
  }

  toast.add({
    severity: "success",
    summary: "Secret link revoked",
    detail: "That link can no longer be used to access this page.",
    life: TOAST_INFO_TTL,
  });

  emit("revoke", props.index);
};

const revokeSecretLink = async () => {
  if (isOnlyRemainingLink.value) {
    toast.add({
      severity: "error",
      summary: "Cannot revoke only remaining secret link",
      detail: "Without any secret links, you would be locked out of this page!",
      life: TOAST_ERROR_TTL,
    });

    return;
  }

  confirm.require({
    header: "Revoke this secret link?",
    message: isCurrentUser.value
      ? "You are about to revoke the secret link you are currently using to access this page! Are you sure you want to do this? You will be locked out of this page unless someone with access generates a new secret link for you."
      : "Are you sure you want to permanently revoke this secret link? Once revoked, nobody will be able to use it to access this page.",
    icon: isCurrentUser.value ? "pi pi-exclamation-triangle" : "pi pi-info-circle",
    acceptProps: {
      label: "Revoke",
      severity: "danger",
    },
    rejectProps: {
      label: "Cancel",
      severity: "secondary",
      outlined: true,
    },
    accept: doRevoke,
  });
};
</script>

<template>
  <div class="flex gap-4 sm:gap-8 items-center justify-between">
    <div class="flex gap-4 items-center">
      <i
        class="!hidden md:!inline pi pi-key text-green-500 dark:text-green-200"
        style="font-size: 1.5rem"
        aria-hidden="true"
      ></i>
      <div class="flex flex-col">
        <span class="flex flex-wrap gap-2 justify-start items-center">
          <i class="pi pi-key md:!hidden text-green-500 dark:text-green-200" aria-hidden="true"></i>
          <span>{{ props.comment }}</span>
          <Tag
            class="text-xs text-nowrap"
            v-if="isCurrentUser"
            value="me"
            severity="info"
            rounded
          />
          <Tag
            class="text-xs text-nowrap"
            v-if="isReadOnly"
            value="read-only"
            severity="warn"
            rounded
          />
        </span>
        <span class="text-muted-color">
          <span v-if="props.accessedAt">
            last used
            <time class="text-nowrap" datetime="props.accessedAt.toISOString()">
              {{ formatDateTime(props.accessedAt) }}
            </time>
          </span>
          <span v-else>never used</span>
        </span>
      </div>
    </div>
    <Button
      class="shrink-0"
      label="Revoke"
      icon="pi pi-times"
      severity="danger"
      size="small"
      variant="outlined"
      @click="revokeSecretLink"
    />
  </div>
</template>

<style scoped></style>
