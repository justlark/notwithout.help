<script setup lang="ts">
import Card from "primevue/card";

const props = defineProps<{
  title: string;
  message: string;
}>();
</script>

<template>
  <div class="flex justify-center">
    <Card class="w-[90vw] sm:w-[28rem] mt-8">
      <template #header>
        <div class="flex justify-center">
          <i class="pi pi-exclamation-circle mt-8 !text-8xl text-red-400 dark:text-red-300"></i>
        </div>
      </template>
      <template #title>{{ props.title }}</template>
      <template #content>{{ props.message }}</template>
    </Card>
  </div>
</template>

<style scoped></style>
