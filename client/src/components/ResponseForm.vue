<script setup lang="ts">
import { computed } from "vue";
import { Form, type FormSubmitEvent } from "@primevue/forms";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";
import Select from "primevue/select";
import ValidationMessage from "@/components/ValidationMessage.vue";
import { CONTACT_METHOD_TYPES, CONTACT_METHODS, type ContactMethodCode } from "@/vars";
import { z } from "zod";
import { loadPersisted, persistingZodResolver } from "@/forms";

const FORM_STORAGE_KEY = "form";

const emit = defineEmits(["submit"]);

const submitForm = ({ valid, values }: FormSubmitEvent) => {
  if (valid) {
    emit("submit", values);
  }
};

type FormValues = {
  name: string;
  contact: string;
  contactType?: ContactMethodCode;
};

const initialValues = computed<FormValues>(() =>
  loadPersisted(FORM_STORAGE_KEY, {
    name: "",
    contact: "",
    contactType: undefined,
  }),
);

const resolver = persistingZodResolver(
  FORM_STORAGE_KEY,
  z.object({
    name: z.string().min(1, { message: "You must provide a name." }),
    contact: z.string().min(1, { message: "You must provide a way to contact you." }),
    contactType: z.enum(CONTACT_METHOD_TYPES, {
      message: "You must provide a preferred contact method.",
    }),
  }),
);
</script>

<template>
  <!--
    All these @vue-ignore directives are necessary to work around an apparent
    bug in the typing for the PrimeVue forms library. See this issue for
    details:

    https://github.com/primefaces/primevue/issues/6723
  -->

  <Form
    v-slot="$form"
    class="max-w-xl mx-auto flex flex-col gap-8"
    :initial-values="initialValues"
    :resolver="resolver"
    @submit="submitForm"
  >
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
      <!-- @vue-ignore -->
      <ValidationMessage name="name" :state="$form.name" />
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
          name="contactType"
          :options="[...CONTACT_METHODS]"
          option-label="name"
          option-value="code"
          size="large"
          placeholder="Method"
          class="basis-1/3 max-sm:grow"
        />
      </div>
      <!-- @vue-ignore -->
      <ValidationMessage name="contact" :state="$form.contact" />
      <!-- @vue-ignore -->
      <ValidationMessage name="contact-type" :state="$form.contactType" />
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        How you want the organizers to contact you.
      </Message>
    </div>

    <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
  </Form>
</template>

<style scoped></style>
