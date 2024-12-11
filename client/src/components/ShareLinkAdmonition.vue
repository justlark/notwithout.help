<script setup lang="ts">
import { computed } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import type { FormId } from "@/types";
import { useToast } from "primevue/usetoast";
import { TOAST_INFO_TTL } from "@/vars";

const props = defineProps<{
  formId: FormId | undefined;
}>();

const origin = computed(() => window.location.origin);

const shareLink = computed(() => {
  if (!props.formId) {
    return undefined;
  }

  return new URL(`${origin.value}/share/#/${props.formId}`);
});

const toast = useToast();

const copyShareLink = () => {
  toast.removeAllGroups();

  toast.add({
    severity: "info",
    summary: "Link copied",
    detail: "Share this link to collect responses.",
    life: TOAST_INFO_TTL,
  });
};
</script>

<template>
  <div class="max-w-xl mx-auto flex flex-col gap-8">
    <LinkAdmonition v-if="shareLink" :link="shareLink" @click="copyShareLink">
      <template #title>
        <span class="flex gap-3 items-center">
          <i class="pi pi-share-alt"></i>
          <strong>Share this link</strong>
        </span>
      </template>

      <template #details>
        <ul>
          <li>Send this link to anyone you want to fill out your survey.</li>
          <li>
            People with this link can respond to your survey, but cannot view other responses.
          </li>
          <li>
            Copy this link down in a safe place, because it will disappear when you leave this page.
          </li>
        </ul>
      </template>
    </LinkAdmonition>
  </div>
</template>

<style scoped></style>
