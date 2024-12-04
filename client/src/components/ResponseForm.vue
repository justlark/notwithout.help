<script setup lang="ts">
import { Form } from "@primevue/forms";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";
import Select from "primevue/select";
import { contactMethods } from "@/vars";
</script>

<template>
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
        The name, nickname, alias, or handle you want to send to the organizers.
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
          :options="contactMethods"
          option-label="name"
          size="large"
          placeholder="Method"
          class="basis-1/3 max-sm:grow"
        />
      </div>
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        How you want the organizers to contact you.
      </Message>
    </div>

    <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
  </Form>
</template>

<style scoped></style>
