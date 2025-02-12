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
  protected: boolean;
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
  confirm.require({
    header: "Revoke this secret link?",
    message: `Are you sure you want to permanently revoke the secret link "${props.comment}"? Once revoked, nobody will be able to use it to access this page.`,
    icon: "pi pi-info-circle",
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
      <i class="!hidden md:!inline pi pi-user" style="font-size: 1.5rem" aria-hidden="true"></i>
      <div class="flex flex-col">
        <span>
          <i class="pi pi-user md:!hidden me-2" aria-hidden="true"></i>
          <span class="me-2">{{ props.comment }}</span>
          <Tag
            class="text-xs text-nowrap me-2"
            v-if="isCurrentUser"
            value="me"
            severity="primary"
            rounded
          />
          <Tag
            class="text-xs text-nowrap me-2"
            v-if="isReadOnly"
            value="read-only"
            severity="secondary"
            rounded
          />
          <Tag
            class="text-xs text-nowrap me-2"
            v-if="props.protected"
            value="password"
            severity="secondary"
            rounded
          />
        </span>
        <span class="text-muted-color">
          <span v-if="props.accessedAt">
            last used
            <time class="text-nowrap" :datetime="props.accessedAt.toISOString()">
              {{ formatDateTime(props.accessedAt) }}
            </time>
          </span>
          <span v-else>never used</span>
        </span>
      </div>
    </div>
    <Button
      v-if="!isCurrentUser && !isOnlyRemainingLink"
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
