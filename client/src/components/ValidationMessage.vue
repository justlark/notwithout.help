<script setup lang="ts">
import Message from "primevue/message";

const props = defineProps<{
  name: string;
  state:
    | {
        invalid: boolean;
        error: { message: string };
      }
    | undefined;
}>();
</script>

<template>
  <Message
    :id="`${props.name}-error`"
    v-if="props.state?.invalid"
    severity="error"
    size="small"
    variant="simple"
  >
    {{ props.state?.error.message }}
  </Message>
</template>

<style scoped></style>
