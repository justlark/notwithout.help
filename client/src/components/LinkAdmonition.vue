<script setup lang="ts">
import Button from "primevue/button";

const props = defineProps<{
  link: URL;
}>();

const emit = defineEmits(["click"]);

const copyLink = async () => {
  await navigator.clipboard.writeText(props.link.href);
  emit("click");
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <div class="flex gap-8 items-center justify-between">
      <a :href="props.link.href" target="_blank" class="break-all">{{ props.link.href }}</a>
      <Button @click="copyLink" label="Copy" icon="pi pi-clipboard" class="min-w-24" />
    </div>
    <slot />
  </div>
</template>

<style scoped></style>
