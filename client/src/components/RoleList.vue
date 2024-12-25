<script setup lang="ts">
import Checkbox from "primevue/checkbox";
import type { BaseFieldProps } from "vee-validate";

export interface Role {
  id: string;
  name: string;
  responsibilities: Array<string>;
}

const props = defineProps<{
  roles: Array<Role>;
  inputAttrs: BaseFieldProps;
}>();

const selectedRoleIds = defineModel<Array<string>>();
</script>

<template>
  <div>
    <div v-for="role of props.roles" :key="role.id" class="flex gap-4">
      <Checkbox
        v-model="selectedRoleIds"
        :input-id="`role-checkbox-${role.id}`"
        :value="role.id"
        size="large"
        v-bind="props.inputAttrs"
      />
      <div class="flex flex-col gap-2">
        <label :for="`role-checkbox-${role.id}`" class="font-bold">{{ role.name }}</label>
        <ul>
          <li v-for="(responsibility, index) of role.responsibilities" :key="index">
            {{ responsibility }}
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
