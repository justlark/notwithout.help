<script setup lang="ts">
import { Form, type FormSubmitEvent } from "@primevue/forms";
import Textarea from "primevue/textarea";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import MultiSelect from "primevue/multiselect";
import Button from "primevue/button";
import DatePicker from "primevue/datepicker";
import ValidationMessage from "@/components/ValidationMessage.vue";
import { CONTACT_METHOD_TYPES, CONTACT_METHODS, type ContactMethodCode } from "@/vars";
import { computed } from "vue";
import { loadPersisted, persistingZodResolver } from "@/forms";
import { z } from "zod";

const FORM_STORAGE_KEY = "template";

type Emits = {
  (eventName: "submit", values: FormValues): void;
};

const emit = defineEmits<Emits>();

const submitForm = ({ valid, values }: FormSubmitEvent & { values: FormValues }) => {
  if (valid) {
    emit("submit", values);
  }
};

export type FormValues = {
  title: string;
  description: string;
  contactMethods: Array<ContactMethodCode>;
  expirationDate?: Date;
};

const initialValues = computed<FormValues>(() =>
  loadPersisted(FORM_STORAGE_KEY, {
    title: "",
    description: "",
    contactMethods: [],
    expirationDate: undefined,
  }),
);

const resolver = persistingZodResolver(
  FORM_STORAGE_KEY,
  z.object({
    title: z.string().min(1, { message: "Title is required." }),
    description: z.string().min(1, { message: "Description is required." }),
    contactMethods: z
      .array(z.enum(CONTACT_METHOD_TYPES))
      .nonempty({ message: "You must specify at least one contact method." }),
    expirationDate: z.date().optional(),
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
      <label for="title-input" class="flex gap-2">
        Title
        <span class="required-marker">*</span>
      </label>
      <InputText
        id="title-input"
        name="title"
        type="text"
        size="large"
        aria-describedby="title-help"
      />
      <!-- @vue-ignore -->
      <ValidationMessage name="title" :state="$form.title" />
      <Message id="title-help" size="small" severity="secondary" variant="simple">
        A title for your form. Include the name of your group or organization.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="description-input" class="flex gap-2">
        Description
        <span class="required-marker">*</span>
      </label>
      <Textarea
        id="description-input"
        name="description"
        auto-resize
        aria-describedby="description-help"
      />
      <!-- @vue-ignore -->
      <ValidationMessage name="description" :state="$form.description" />
      <Message id="description-help" size="small" severity="secondary" variant="simple">
        Provide some information about your group or organization and what you're looking for.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="contact-input" class="flex gap-2">
        <span>Contact methods</span>
        <span class="required-marker">*</span>
      </label>
      <MultiSelect
        id="contact-input"
        name="contactMethods"
        :options="[...CONTACT_METHODS]"
        option-label="name"
        option-value="code"
        placeholder="Select contact methods"
        display="chip"
        size="large"
        aria-describedby="contact-help"
      />
      <!-- @vue-ignore -->
      <ValidationMessage name="contact" :state="$form.contactMethods" />
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        Specify what contact methods you want respondents to pick from when leaving their contact
        information.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="date-input">Expiration date</label>
      <DatePicker id="date-input" name="expirationDate" size="large" aria-describedby="date-help" />
      <Message id="date-help" size="small" severity="secondary" variant="simple">
        Specify an optional expiration date for the form. All submissions will be permanently
        deleted after this date.
      </Message>
    </div>

    <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
  </Form>
</template>

<style scoped>
.required-marker {
  color: var(--p-message-error-color);
}
</style>
