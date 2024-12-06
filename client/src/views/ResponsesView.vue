<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">View responses</h1>
    <div class="flex flex-col gap-4 items-center">
      <FormResponse
        class="w-full xl:w-3/4"
        name="Lark has a really really really really long name"
        contact="lark@example.com"
        contactType="email"
        :createdAt="new Date()"
      />
      <FormResponse
        class="w-full xl:w-3/4"
        name="Lark"
        contact="lark@example.com"
        contactType="email"
        :createdAt="new Date()"
      />
    </div>
  </main>
</template>

<style scoped></style>
