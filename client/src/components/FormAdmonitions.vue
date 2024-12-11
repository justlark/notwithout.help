<script setup lang="ts">
import { type ClientKeyId, type FormId, type SecretLinkKey } from "@/crypto";
import ShareLinkAdmonition from "@/components/ShareLinkAdmonition.vue";
import SecretLinkAdmonition from "@/components/SecretLinkAdmonition.vue";

const props = defineProps<{
  formId: FormId;
  clientKeyId: ClientKeyId;
  secretLinkKey: SecretLinkKey;
}>();
</script>

<template>
  <div class="max-w-xl mx-auto flex flex-col gap-8">
    <Card>
      <template #title>
        <span class="flex gap-3 items-center">
          <i class="pi pi-share-alt"></i>
          <strong>Share this link</strong>
        </span>
      </template>
      <template #content>
        <ShareLinkAdmonition :formId="props.formId" />
      </template>
    </Card>
    <Card>
      <template #title>
        <span class="flex gap-3 items-center">
          <i class="pi pi-lock"></i>
          <strong>Keep this link secret</strong>
        </span>
      </template>
      <template #content>
        <SecretLinkAdmonition
          :formId="props.formId"
          :clientKeyId="props.clientKeyId"
          :secretLinkKey="props.secretLinkKey"
        />
      </template>
    </Card>
  </div>
</template>

<style scoped></style>
