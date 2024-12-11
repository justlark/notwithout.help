<script setup lang="ts">
import type { FormId } from "@/crypto";
import { formatDateTime } from "@/encoding";
import Button from "primevue/button";
import Tag from "primevue/tag";

const props = defineProps<{
  comment: string;
  formId: FormId;
  accessedAt: Date;
  isAdmin: boolean;
}>();
</script>

<template>
  <div class="flex gap-8 items-center justify-between">
    <div class="flex gap-4 items-center">
      <i
        class="!hidden md:!inline pi pi-key text-green-500 dark:text-green-200"
        style="font-size: 1.5rem"
        aria-hidden="true"
      ></i>
      <div class="flex flex-col">
        <span class="flex gap-2 items-baseline">
          <i class="pi pi-key md:!hidden text-green-500 dark:text-green-200" aria-hidden="true"></i>
          <span>{{ props.comment }}</span>
          <Tag class="text-xs" v-if="props.isAdmin" value="admin" severity="warn" rounded />
        </span>
        <span class="text-muted-color">
          last used
          <time class="text-nowrap" datetime="props.accessedAt.toISOString()">
            {{ formatDateTime(props.accessedAt) }}
          </time>
        </span>
      </div>
    </div>
    <Button label="Revoke" icon="pi pi-times" severity="danger" size="small" variant="outlined" />
  </div>
</template>

<style scoped></style>
