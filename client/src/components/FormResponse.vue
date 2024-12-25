<script setup lang="ts">
import Card from "primevue/card";
import Tag from "primevue/tag";
import { formatDateTime } from "@/encoding";
import { TOAST_INFO_TTL } from "@/vars";
import { useToast } from "primevue";
import { computed } from "vue";

const props = defineProps<{
  index: string;
  name: string;
  contact: string;
  contactMethod: string;
  roles: Array<string>;
  comment: string;
  createdAt: Date;
}>();

const toast = useToast();

const copyContact = async () => {
  await navigator.clipboard.writeText(props.contact);

  toast.add({
    severity: "info",
    summary: "Contact info copied",
    detail: `Copied contact info for ${props.name}.`,
    life: TOAST_INFO_TTL,
  });
};

const headingId = computed(() => `contact-card-heading-${props.index}`);

// Convert newline-separated blocks of text in the input into paragraphs, just
// like Markdown. Collapse adjacent lines and remove empty paragraphs.
const commentParagraphs = computed(() => props.comment.split("\n\n").filter((p) => p.length > 0));
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
        <div class="flex flex-col gap-2">
          <div class="flex gap-2 items-baseline">
            <button @click="copyContact" aria-label="Copy">
              <i class="pi pi-clipboard cursor-pointer" aria-hidden="true" @click="copyContact" />
            </button>
            <span>{{ props.contact }}</span>
            <span class="text-sm text-muted-color">({{ props.contactMethod }})</span>
          </div>
          <div v-if="props.roles.length > 0" class="flex flex-wrap gap-2 text-xs text-nowrap">
            <Tag
              v-for="(role, index) of props.roles"
              :key="index"
              :value="role"
              severity="secondary"
            />
          </div>
          <div v-if="commentParagraphs.length > 0">
            <blockquote>
              <p v-for="(paragraph, index) of commentParagraphs" :key="index" class="last:mb-0">
                {{ paragraph }}
              </p>
            </blockquote>
          </div>
        </div>
      </template>
    </Card>
  </section>
</template>

<style scoped></style>
