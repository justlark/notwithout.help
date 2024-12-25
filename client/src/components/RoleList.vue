<script setup lang="ts">
import type { OrgRole } from "@/api";
import Checkbox from "primevue/checkbox";
import type { BaseFieldProps } from "vee-validate";

const props = defineProps<{
  roles: Array<OrgRole>;
  inputAttrs?: BaseFieldProps;
  preview?: boolean;
}>();

const selectedRoleIds = defineModel<Array<string>>();
</script>

<template>
  <div>
    <div v-for="role of props.roles" :key="role.id" class="flex flex-col gap-2">
      <div class="flex gap-4">
        <Checkbox
          v-model="selectedRoleIds"
          :input-id="`role-checkbox-${role.id}`"
          :value="role.id"
          size="large"
          v-bind="props.inputAttrs"
          :disabled="props.preview"
        />
        <div class="flex flex-col gap-2">
          <label :for="`role-checkbox-${role.id}`" class="font-bold">{{ role.name }}</label>
          <ul class="hidden sm:block">
            <li v-for="(item, index) of role.details" :key="index">
              {{ item }}
            </li>
          </ul>
        </div>
      </div>
      <ul class="block sm:hidden">
        <li v-for="(item, index) of role.details" :key="index">
          {{ item }}
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped></style>
