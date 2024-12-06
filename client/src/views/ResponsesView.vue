<script setup lang="ts">
import FormResponse from "@/components/FormResponse.vue";
import Toast from "primevue/toast";
import SecretLinkList from "@/components/SecretLinkList.vue";
import Button from "primevue/button";
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">View responses</h1>
    <div class="xl:w-3/4 mx-auto">
      <div class="flex flex-col gap-8">
        <SecretLinkList class="self-center w-full" />
        <div class="flex flex-col gap-4 items-center">
          <FormResponse
            index="1"
            class="w-full"
            name="Kit"
            contact="kit@example.com"
            contactType="email"
            :createdAt="new Date()"
          />
          <FormResponse
            index="2"
            class="w-full"
            name="Aster"
            contact="@aster"
            contactType="telegram"
            :createdAt="new Date()"
          />
          <FormResponse
            index="3"
            class="w-full"
            name="Rowan"
            contact="555-555-1234"
            contactType="sms"
            :createdAt="new Date()"
          />
        </div>
      </div>
      <div class="xl:sticky bottom-6">
        <div
          class="flex flex-col gap-3 fixed xl:absolute xl:translate-x-full bottom-6 xl:bottom-0 right-6 xl:-right-6"
        >
          <Button label="Export" severity="secondary" icon="pi pi-download" />
          <Button label="Delete" severity="danger" icon="pi pi-trash" />
        </div>
      </div>
    </div>
    <Toast position="bottom-center" />
  </main>
</template>

<style scoped></style>
