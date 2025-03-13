<script setup lang="ts">
import Message from "primevue/message";

const props = defineProps<{
  id: string;
  label?: string;
  required?: boolean;
  error?: string;
}>();
</script>

<template>
  <div class="flex flex-col gap-2">
    <label v-if="props.label" :for="`${props.id}-input`" class="flex gap-2">
      <span>{{ props.label }}</span>
      <span v-if="props.required" class="text-red-600 dark:text-red-500">*</span>
    </label>
    <slot name="input" :id="`${props.id}-input`" :aria-describedby="`${props.id}-help`" />
    <Message v-if="props.error" severity="error" size="small" variant="simple">
      {{ props.error }}
    </Message>
    <span id="`${props.id}-help`" class="text-muted-color text-sm font-medium">
      <slot name="help"></slot>
    </span>
  </div>
</template>

<style scoped></style>
