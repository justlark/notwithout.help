<script setup lang="ts">
import { computed } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import { useToast } from "primevue/usetoast";
import { TOAST_INFO_TTL } from "@/vars";
import { type ClientKeyId, type FormId, type SecretLinkKey } from "@/crypto";
import { encodeBase64Url } from "@/encoding";

const props = defineProps<{
  formId: FormId | undefined;
  clientKeyId: ClientKeyId | undefined;
  secretLinkKey: SecretLinkKey | undefined;
}>();

const origin = computed(() => window.location.origin);

const secretLink = computed(() => {
  if (!props.formId || !props.clientKeyId || !props.secretLinkKey) {
    return undefined;
  }

  return new URL(
    `${origin.value}/view/#/${props.formId}/${props.clientKeyId}/${encodeBase64Url(props.secretLinkKey)}`,
  );
});

const toast = useToast();

const copySecretLink = () => {
  toast.removeAllGroups();

  toast.add({
    severity: "warn",
    summary: "Secret link copied",
    detail: "Be careful who you share this link with.",
    life: TOAST_INFO_TTL,
  });
};
</script>

<template>
  <div class="max-w-xl mx-auto flex flex-col gap-8">
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
