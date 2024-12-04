<script setup lang="ts">
import { Form } from "@primevue/forms";
import Textarea from "primevue/textarea";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import MultiSelect from "primevue/multiselect";
import Button from "primevue/button";
import { contactMethods } from "@/vars";

const emit = defineEmits(["submit"]);

const submitForm = () => {
  emit("submit");
};
</script>

<template>
  <Form class="max-w-xl mx-auto flex flex-col gap-8" @submit="submitForm">
    <div class="flex flex-col gap-2">
      <label for="title-input">Title</label>
      <InputText
        id="title-input"
        name="title"
        type="text"
        size="large"
        aria-describedby="title-help"
      />
      <Message id="title-help" size="small" severity="secondary" variant="simple">
        A title for your form. Include the name of your group or organization.
      </Message>
    </div>
    <div class="flex flex-col gap-2">
      <label for="description-input">Description</label>
      <Textarea
        id="description-input"
        name="description"
        auto-resize
        aria-describedby="description-help"
      />
      <Message id="description-help" size="small" severity="secondary" variant="simple">
        Provide some information about your group or organization and what you're looking for.
      </Message>
    </div>
    <div class="flex flex-col gap-2">
      <label for="contact-input">Contact methods</label>
      <MultiSelect
        id="contact-input"
        name="contact_methodvs"
        :options="contactMethods"
        option-label="name"
        placeholder="Select contact methods"
        display="chip"
        size="large"
        aria-describedby="contact-help"
      />
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        Specify what contact methods you want respondents to pick from when leaving their contact
        information.
      </Message>
    </div>

    <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
  </Form>
</template>

<style scoped></style>
