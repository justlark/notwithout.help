<script setup lang="ts">
import { ref } from "vue";
import FormLinks from "@/components/FormLinks.vue";
import type { ClientKeyId, FormId, SecretLinkKey } from "@/types";

// TODO: Example of a Form ID.
const formId = ref("VmrfdKsn" as FormId);

// TODO: Example of a Client Key ID.
const clientKeyId = ref("0" as ClientKeyId);

// TODO: Example of a base64url-encoded Secret Link Key.
const secretLinkKey = ref("6IKYnsjFRUkvZfpcLjlaMEAvo61IUO44y8EIcCbV2rM=" as SecretLinkKey);
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
    <FormLinks :formId="formId" :clientKeyId="clientKeyId" :secretLinkKey="secretLinkKey" />
  </main>
</template>

<style scoped></style>
