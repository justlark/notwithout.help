<script setup lang="ts">
import { Form } from "@primevue/forms";
import { zodResolver } from "@primevue/forms/resolvers/zod";
import Textarea from "primevue/textarea";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import MultiSelect from "primevue/multiselect";
import Button from "primevue/button";
import { contactMethods } from "@/vars";
import { reactive, ref } from "vue";
import { z } from "zod";

const emit = defineEmits(["submit"]);

const submitForm = ({ valid }: { valid: boolean }) => {
  if (valid) {
    emit("submit");
  }
};

const initialValues = reactive({
  title: "",
  description: "",
  contactMethods: [],
});

const resolver = ref(
  zodResolver(
    z.object({
      title: z.string().min(1, { message: "Title is required." }),
      description: z.string().min(1, { message: "Description is required." }),
      contactMethods: z
        .array(z.object({}))
        .nonempty({ message: "You must specify at least one contact method." }),
    }),
  ),
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
      <Message
        id="title-error"
        v-if="$form.title?.invalid"
        severity="error"
        size="small"
        variant="simple"
      >
        <!-- @vue-ignore -->
        {{ $form.title?.error.message }}
      </Message>
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
      <Message
        id="description-error"
        v-if="$form.description?.invalid"
        severity="error"
        size="small"
        variant="simple"
      >
        <!-- @vue-ignore -->
        {{ $form.description?.error.message }}
      </Message>
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
        :options="contactMethods"
        option-label="name"
        placeholder="Select contact methods"
        display="chip"
        size="large"
        aria-describedby="contact-help"
      />
      <!-- @vue-ignore -->
      <Message
        id="contact-error"
        v-if="$form.contactMethods?.invalid"
        severity="error"
        size="small"
        variant="simple"
      >
        <!-- @vue-ignore -->
        {{ $form.contactMethods?.error.message }}
      </Message>
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        Specify what contact methods you want respondents to pick from when leaving their contact
        information.
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
