<script setup lang="ts">
import type { ClientKeyId, FormId } from "@/types";
import { formatDateTime } from "@/encoding";
import Button from "primevue/button";

const props = defineProps<{
  index: string;
  comment: string;
  formId: FormId;
  clientKeyId: ClientKeyId;
  accessedAt: Date;
}>();
</script>

<template>
  <div class="flex gap-8 items-center justify-between">
    <div class="flex gap-4 items-center">
      <i
        class="key-icon !hidden md:!inline pi pi-key"
        style="font-size: 1.5rem"
        aria-hidden="true"
      ></i>
      <div class="flex flex-col">
        <span>{{ props.comment }}</span>
        <span class="text-muted-color">
          last used
          <time class="text-nowrap" datetime="props.accessedAt.toISOString()">
            {{ formatDateTime(props.accessedAt) }}
          </time>
        </span>
      </div>
    </div>
    <Button label="Revoke" severity="danger" size="small" />
  </div>
</template>

<style scoped>
.key-icon {
  color: var(--p-green-500);
}

@media (prefers-color-scheme: dark) {
  .key-icon {
    color: var(--p-green-200);
  }
}
</style>
