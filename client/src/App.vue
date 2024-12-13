<script setup lang="ts">
import { RouterView } from "vue-router";
import Toast from "primevue/toast";
import { randomTitleLead } from "./vars";
import { computed } from "vue";
import icon from "./assets/icon.svg";

const titleLead = computed(() => randomTitleLead());
</script>

<template>
  <header class="flex gap-3 items-center mb-8">
    <img :src="icon" alt="A lifebuoy" class="w-14 h-14" />
    <div class="flex flex-col gap-1">
      <span class="text-muted-color">{{ titleLead }}</span>
      <RouterLink to="/" class="text-lg font-medium">Not Without Help</RouterLink>
    </div>
  </header>

  <RouterView />

  <Toast class="max-w-[90vw]" position="bottom-center" />
</template>

<style scoped></style>
