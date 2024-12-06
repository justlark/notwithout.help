<script setup lang="ts">
import Card from "primevue/card";
import { formatDateTime } from "@/encoding";
import { type ContactMethodCode, contactMethodByCode } from "@/vars";

const props = defineProps<{
  name: string;
  contact: string;
  contactType: ContactMethodCode;
  createdAt: Date;
}>();
</script>

<template>
  <Card>
    <template #title>
      <div class="flex flex-col md:flex-row gap-x-4 justify-between items-baseline">
        <span>{{ props.name }}</span>
        <span class="text-base text-muted-color text-nowrap">{{
          formatDateTime(props.createdAt)
        }}</span>
      </div>
    </template>
    <template #content>
      <div class="flex gap-2 items-baseline">
        <span>{{ props.contact }}</span>
        <span class="text-sm text-muted-color">({{ contactMethodByCode(props.contactType) }})</span>
      </div>
    </template>
  </Card>
</template>

<style scoped></style>
