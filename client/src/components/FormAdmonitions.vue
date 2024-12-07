<script setup lang="ts">
import type { ClientKeyId, FormId } from "@/types";
import { type SecretLinkKey } from "@/crypto";
import ShareLinkAdmonition from "@/components/ShareLinkAdmonition.vue";
import SecretLinkAdmonition from "@/components/SecretLinkAdmonition.vue";

const TOAST_GROUP = "form-link-copy";

const props = defineProps<{
  formId: FormId | undefined;
  clientKeyId: ClientKeyId | undefined;
  secretLinkKey: SecretLinkKey | undefined;
}>();
</script>

<template>
  <div class="max-w-xl mx-auto flex flex-col gap-8">
    <ShareLinkAdmonition :formId="props.formId" :toastGroup="TOAST_GROUP" />
    <SecretLinkAdmonition
      :formId="props.formId"
      :clientKeyId="props.clientKeyId"
      :secretLinkKey="props.secretLinkKey"
      :toastGroup="TOAST_GROUP"
    />
  </div>
</template>

<style scoped></style>
