<script setup lang="ts">
import { ref } from "vue";
import SelectButton from "primevue/selectbutton";
import InputText from "primevue/inputtext";
import Button from "primevue/button";

const linkTypeOptions = ref([
  { label: "Admin", icon: "pi pi-shield", value: "admin" },
  { label: "Read-only", icon: "pi pi-eye", value: "read" },
]);

const selectedLinkType = ref<"admin" | "read">();
const linkName = ref<string>();
const linkPassword = ref<string>();
</script>

<template>
  <div class="flex flex-col max-w-2xl gap-6">
    <div class="flex flex-col gap-2">
      <label for="secret-link-name-input" class="flex gap-2">
        <span>Who are you sharing this link with?</span>
        <span class="text-red-600 dark:text-red-500">*</span>
      </label>
      <InputText
        id="secret-link-name-input"
        v-model="linkName"
        type="text"
        aria-describedby="secret-link-name-help"
      />
      <span id="secret-link-name-help" class="text-muted-color text-sm font-medium">
        Enter the name of the person you're sharing this link with, so you can remember who has
        which link and revoke it later if you need to.
      </span>
    </div>
    <div class="flex flex-col gap-2">
      <label for="secret-link-type-input" class="flex gap-2">
        <span>What kind of link is this?</span>
        <span class="text-red-600 dark:text-red-500">*</span>
      </label>
      <SelectButton
        id="secret-link-type-input"
        v-model="selectedLinkType"
        :options="linkTypeOptions"
        optionLabel="label"
        dataKey="value"
        aria-describedby="secret-link-type-help"
      >
        <template #option="slotProps">
          <span class="flex gap-3 items-center">
            <i :class="slotProps.option.icon"></i>
            <span>{{ slotProps.option.label }}</span>
          </span>
        </template>
      </SelectButton>
      <span id="secret-link-type-help" class="text-muted-color text-sm font-medium">
        Read-only links can't be used to make edits or generate new secret links. Admin links can.
      </span>
    </div>
    <div class="flex flex-col gap-2">
      <label for="secret-link-password-input">Set a password (optional)</label>
      <InputText
        id="secret-link-password-input"
        v-model="linkPassword"
        type="text"
        aria-describedby="secret-link-password-help"
      />
      <span id="secret-link-password-help" class="text-muted-color text-sm font-medium">
        For added security, you can set a password that the person you send this link to will need
        to know to access the page.
      </span>
    </div>
    <Button type="submit" severity="primary" label="Create" class="max-w-24" />
  </div>
</template>

<style scoped></style>
