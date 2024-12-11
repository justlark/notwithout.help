<script setup lang="ts">
import { computed } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import { useToast } from "primevue/usetoast";
import { newShareLink, TOAST_INFO_TTL } from "@/vars";
import type { FormId } from "@/crypto";

const props = defineProps<{
  formId: FormId;
}>();

const shareLink = computed(() => newShareLink(props.formId));

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
    <LinkAdmonition :link="shareLink" @click="copyShareLink">
      <ul>
        <li>Send this link to anyone you want to fill out your survey.</li>
        <li>People with this link can respond to your survey, but cannot view other responses.</li>
        <li>
          Copy this link down in a safe place, because it will disappear when you leave this page.
        </li>
      </ul>
    </LinkAdmonition>
  </div>
</template>

<style scoped></style>
