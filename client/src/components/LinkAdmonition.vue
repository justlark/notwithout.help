<script setup lang="ts">
import Card from "primevue/card";
import Button from "primevue/button";

const props = defineProps<{
  link: URL;
  summary: string;
  icon: string;
}>();
</script>

<template>
  <Card>
    <template #title>
      <span class="flex gap-3 items-center">
        <i :class="icon"></i>
        <strong>{{ props.summary }}</strong>
      </span>
    </template>
    <template #content>
      <div class="flex flex-col gap-4">
        <div class="flex gap-8 items-center justify-between">
          <a :href="props.link.href" target="_blank" class="break-all">{{ props.link.href }}</a>
          <Button label="Copy" icon="pi pi-clipboard" class="min-w-24" />
        </div>
        <slot />
      </div>
    </template>
  </Card>
</template>

<style scoped></style>
