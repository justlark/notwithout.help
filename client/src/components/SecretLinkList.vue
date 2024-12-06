<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import InputText from "primevue/inputtext";
import SplitButton from "primevue/splitbutton";
import type { ClientKeyId, FormId } from "@/types";
import { ref } from "vue";

// TODO: These are placeholders.
const mockFormId = "" as FormId;
const mockClientKeyId = "" as ClientKeyId;
const count = ref(4);

const newLinkComment = ref("");

const createSecretLink = () => {
  // TODO: Implement this.
};

const createSecretAdminLink = () => {
  // TODO: Implement this.
};

const secretLinkActions = [
  {
    label: "Admin",
    icon: "pi pi-shield",
    command: createSecretAdminLink,
  },
];
</script>

<template>
  <section aria-labelledby="secret-links-list-heading">
    <Panel toggleable collapsed>
      <template #header>
        <span class="flex items-center gap-3">
          <i class="pi pi-lock !text-xl"></i>
          <h1 id="secret-links-list-heading" class="text-lg mb-0">Secret links ({{ count }})</h1>
        </span>
      </template>
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <SecretLinkListItem
            index="1"
            comment="Lark"
            :formId="mockFormId"
            :clientKeyId="mockClientKeyId"
            :accessedAt="new Date()"
            :isAdmin="true"
          />
          <SecretLinkListItem
            index="1"
            comment="Avery"
            :formId="mockFormId"
            :clientKeyId="mockClientKeyId"
            :accessedAt="new Date()"
            :isAdmin="false"
          />
          <SecretLinkListItem
            index="1"
            comment="Kai"
            :formId="mockFormId"
            :clientKeyId="mockClientKeyId"
            :accessedAt="new Date()"
            :isAdmin="false"
          />
          <SecretLinkListItem
            index="1"
            comment="Ash"
            :formId="mockFormId"
            :clientKeyId="mockClientKeyId"
            :accessedAt="new Date()"
            :isAdmin="false"
          />
        </div>
        <div class="flex flex-col gap-2">
          <label for="new-link-comment" class="text-sm">Create a new secret link</label>
          <span class="flex gap-8 justify-between">
            <InputText
              id="new-link-comment"
              class="grow"
              v-model="newLinkComment"
              type="text"
              placeholder="Who are you sharing this link with?"
              size="small"
            />
            <SplitButton
              @click="createSecretLink"
              icon="pi pi-plus"
              :button-props="{ 'aria-label': 'Create' }"
              :menu-button-props="{ 'aria-label': 'More options' }"
              :model="secretLinkActions"
              size="small"
            />
          </span>
        </div>
      </div>
    </Panel>
  </section>
</template>

<style scoped></style>
