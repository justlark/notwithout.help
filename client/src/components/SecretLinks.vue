<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLink from "@/components/SecretLink.vue";
import type { ClientKeyId, FormId } from "@/types";
import { ref } from "vue";

// TODO: These are placeholders.
const mockFormId = "" as FormId;
const mockClientKeyId = "" as ClientKeyId;
const count = ref(4);
</script>

<template>
  <section aria-labelledby="secret-links-list-heading">
    <Panel toggleable>
      <template #header>
        <span class="flex items-center gap-3">
          <i class="pi pi-lock !text-xl"></i>
          <h1 id="secret-links-list-heading" class="text-lg mb-0">Secret links ({{ count }})</h1>
        </span>
      </template>
      <div class="flex flex-col gap-2">
        <SecretLink
          index="1"
          comment="Lark"
          :formId="mockFormId"
          :clientKeyId="mockClientKeyId"
          :accessedAt="new Date()"
        />
        <SecretLink
          index="1"
          comment="Avery"
          :formId="mockFormId"
          :clientKeyId="mockClientKeyId"
          :accessedAt="new Date()"
        />
        <SecretLink
          index="1"
          comment="Kai"
          :formId="mockFormId"
          :clientKeyId="mockClientKeyId"
          :accessedAt="new Date()"
        />
        <SecretLink
          index="1"
          comment="Ash has a really long name"
          :formId="mockFormId"
          :clientKeyId="mockClientKeyId"
          :accessedAt="new Date()"
        />
      </div>
    </Panel>
  </section>
</template>

<style scoped></style>
