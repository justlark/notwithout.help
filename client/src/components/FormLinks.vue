<script setup lang="ts">
import { computed } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import type { ClientKeyId, FormId } from "@/types";
import { useToast } from "primevue/usetoast";
import { TOAST_TTL } from "@/vars";
import Toast from "primevue/toast";
import { type SecretLinkKey } from "@/crypto";
import { encodeBase64Url } from "@/encoding";

const TOAST_GROUP = "form-link-copy";

const props = defineProps<{
  formId: FormId | undefined;
  clientKeyId: ClientKeyId | undefined;
  secretLinkKey: SecretLinkKey | undefined;
}>();

const origin = computed(() => window.location.origin);

const shareLink = computed(() => {
  if (!props.formId) {
    return undefined;
  }

  return new URL(`${origin.value}/share/#/${props.formId}`);
});

const secretLink = computed(() => {
  if (!props.formId || !props.clientKeyId || !props.secretLinkKey) {
    return undefined;
  }

  return new URL(
    `${origin.value}/view/#/${props.formId}/${props.clientKeyId}/${encodeBase64Url(props.secretLinkKey)}`,
  );
});

const toast = useToast();

const copyShareLink = () => {
  toast.removeGroup(TOAST_GROUP);

  toast.add({
    severity: "info",
    summary: "Link copied",
    detail: "Share this link to collect responses.",
    life: TOAST_TTL,
    group: TOAST_GROUP,
  });
};

const copySecretLink = () => {
  toast.removeGroup(TOAST_GROUP);

  toast.add({
    severity: "warn",
    summary: "Secret link copied",
    detail: "Be careful who you share this link with.",
    life: TOAST_TTL,
    group: TOAST_GROUP,
  });
};
</script>

<template>
  <div class="max-w-xl mx-auto flex flex-col gap-8">
    <Toast position="bottom-center" :group="TOAST_GROUP" />
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

    <LinkAdmonition v-if="secretLink" :link="secretLink" @click="copySecretLink">
      <template #title>
        <span class="flex gap-3 items-center">
          <i class="pi pi-lock"></i>
          <strong>Keep this link secret</strong>
        </span>
      </template>

      <template #details>
        <ul>
          <li>Use this link to view responses to your survey.</li>
          <li>
            This link is like a password. <strong>Anyone</strong> with this link can access your
            survey responses.
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
