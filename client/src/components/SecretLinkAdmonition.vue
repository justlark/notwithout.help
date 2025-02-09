<script setup lang="ts">
import { computed } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import { useToast } from "primevue/usetoast";
import { newSecretLink, TOAST_INFO_TTL } from "@/vars";
import {
  type ClientKeyId,
  type FormId,
  type ProtectedSecretLinkKey,
  type SecretLinkKey,
} from "@/crypto";

const props = defineProps<{
  formId: FormId;
  clientKeyId: ClientKeyId;
  secretLinkKey: SecretLinkKey | ProtectedSecretLinkKey;
}>();

const secretLink = computed(() =>
  newSecretLink(props.formId, props.clientKeyId, props.secretLinkKey),
);

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
    <LinkAdmonition :link="secretLink" @click="copySecretLink">
      <ul>
        <li>Use this link to view responses to your survey.</li>
        <li>
          This link is like a password. Anyone with this link can access your survey responses.
        </li>
        <li>
          Copy this link down in a safe place, because it will disappear when you leave this page.
        </li>
      </ul>
    </LinkAdmonition>
  </div>
</template>

<style scoped></style>
