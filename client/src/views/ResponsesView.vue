<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import Toast from "primevue/toast";
import SecretLinkList from "@/components/SecretLinkList.vue";
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">View responses</h1>
    <div class="flex flex-col gap-8">
      <SecretLinkList class="self-center w-full xl:w-3/4" />
      <div class="flex flex-col gap-4 items-center">
        <FormResponse
          index="1"
          class="w-full xl:w-3/4"
          name="Kit has a really really really really long name"
          contact="kit@example.com"
          contactType="email"
          :createdAt="new Date()"
        />
        <FormResponse
          index="2"
          class="w-full xl:w-3/4"
          name="Aster"
          contact="aster@example.com"
          contactType="email"
          :createdAt="new Date()"
        />
      </div>
    </div>
    <Toast position="bottom-center" />
  </main>
</template>

<style scoped></style>
