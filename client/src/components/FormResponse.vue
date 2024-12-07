<script setup lang="ts">
import Card from "primevue/card";
import { formatDateTime } from "@/encoding";
import { type ContactMethodCode, contactMethodByCode, TOAST_TTL } from "@/vars";
import { useToast } from "primevue";
import { computed } from "vue";

const props = defineProps<{
  index: string;
  name: string;
  contact: string;
  contactType: ContactMethodCode;
  createdAt: Date;
}>();

const toast = useToast();

const copyContact = async () => {
  await navigator.clipboard.writeText(props.contact);

  toast.add({
    severity: "info",
    summary: "Contact info copied",
    detail: `Copied contact info for ${props.name}.`,
    life: TOAST_TTL,
  });
};

const headingId = computed(() => `contact-card-heading-${props.index}`);
</script>

<template>
  <section :aria-labelledby="headingId">
    <Card>
      <template #title>
        <div class="flex flex-col md:flex-row gap-x-4 justify-between items-baseline">
          <h1 :id="headingId" class="text-lg mb-0">{{ props.name }}</h1>
          <time
            :datetime="props.createdAt.toISOString()"
            class="text-base text-muted-color text-nowrap"
          >
            {{ formatDateTime(props.createdAt) }}
          </time>
        </div>
      </template>
      <template #content>
        <div class="flex gap-2 items-baseline">
          <button @click="copyContact" aria-label="Copy">
            <i class="pi pi-clipboard cursor-pointer" aria-hidden="true" @click="copyContact" />
          </button>
          <span>{{ props.contact }}</span>
          <span class="text-sm text-muted-color"
            >({{ contactMethodByCode(props.contactType)?.name }})</span
          >
        </div>
      </template>
    </Card>
  </section>
</template>

<style scoped></style>
