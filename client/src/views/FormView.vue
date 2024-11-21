<script setup lang="ts">
import { ref } from "vue";
import { Form } from "@primevue/forms";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";
import Select from "primevue/select";

const contactTypes = ref([
  { name: "Email", code: "email" },
  { name: "SMS", code: "sms" },
  { name: "Signal", code: "signal" },
  { name: "Matrix", code: "matrix" },
  { name: "Telegram", code: "telegram" },
  { name: "Discord", code: "discord" },
  { name: "Mastodon", code: "mastodon" },
  { name: "Bluesky", code: "bluesky" },
  { name: "Threads", code: "threads" },
  { name: "Twitter", code: "twitter" },
  { name: "Instagram", code: "instagram" },
  { name: "Facebook", code: "facebook" },
  { name: "Other", code: "other" },
]);
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Get Involved</h1>
    <Form class="max-w-xl mx-auto flex flex-col gap-8">
      <div class="flex flex-col gap-2">
        <label for="name-input">Your name</label>
        <InputText
          id="name-input"
          name="name"
          type="text"
          size="large"
          placeholder="Jane"
          aria-describedby="name-help"
        />
        <Message id="name-help" size="small" severity="secondary" variant="simple">
          The name, nickname, alias, or handle you want to send to the organizers
        </Message>
      </div>

      <div class="flex flex-col gap-2">
        <label for="contact-input">Contact info</label>
        <div class="flex max-sm:flex-col gap-2">
          <InputText
            id="contact-input"
            name="contact"
            type="text"
            size="large"
            placeholder="jane@example.com"
            aria-describedby="contact-help"
            class="grow"
          />
          <Select
            name="contact_type"
            :options="contactTypes"
            optionLabel="name"
            size="large"
            placeholder="Method"
            class="basis-1/3 max-sm:grow"
          />
        </div>
        <Message id="contact-help" size="small" severity="secondary" variant="simple">
          How you want the organizers to contact you
        </Message>
      </div>

      <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
    </Form>
  </main>
</template>

<style scoped></style>
