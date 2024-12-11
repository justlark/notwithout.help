<script setup lang="ts">
import { RouterView } from "vue-router";
import Toast from "primevue/toast";
import { randomTitleLead } from "./vars";
import { computed } from "vue";

const titleLead = computed(() => randomTitleLead());
</script>

<template>
  <header class="flex flex-col gap-2 mb-10 justify-baseline items-center text-center">
    <span class="text-muted-color">{{ titleLead }}</span>
    <RouterLink to="/" class="text-lg font-medium">Not Without Help</RouterLink>
  </header>

  <RouterView />

  <Toast class="max-w-[90vw]" position="bottom-center" />
</template>

<style scoped></style>
