<script setup lang="ts">
import { computed, watch } from "vue";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";
import Select from "primevue/select";
import { CONTACT_METHOD_CODES, contactMethodByCode, type ContactMethodCode } from "@/vars";
import { z } from "zod";
import { loadState, persistState } from "@/state";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";

const FORM_STORAGE_KEY = "form";

const props = defineProps<{
  contactMethods: Array<ContactMethodCode>;
}>();

type Emits = {
  (eventName: "submit", values: FormValues): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  name: z.string().min(1, { message: "You must provide a name." }),
  contact: z.string().min(1, { message: "You must provide a way to contact you." }),
  contactMethod: z.enum(CONTACT_METHOD_CODES, {
    message: "You must provide a preferred contact method.",
  }),
});

export type FormValues = z.infer<typeof schema>;

const initialValues = computed(() =>
  loadState<FormValues>(FORM_STORAGE_KEY, (values) => ({
    name: values.name ?? "",
    contact: values.contact ?? "",
    contactMethod: values.contactMethod,
  })),
);

const {
  values,
  errors,
  defineField,
  resetForm: resetFormInner,
  handleSubmit,
} = useForm<FormValues>({
  validationSchema: toTypedSchema(schema),
  initialValues: initialValues.value,
});

const [name, nameAttrs] = defineField("name");
const [contact, contactAttrs] = defineField("contact");
const [contactMethod, contactMethodAttrs] = defineField("contactMethod");

watch(values, () => {
  persistState(FORM_STORAGE_KEY, values);
});

const submitForm = handleSubmit((values) => {
  emit("submit", values);
});

const resetForm = () => {
  localStorage.removeItem(FORM_STORAGE_KEY);
  resetFormInner({ values: initialValues.value });
};
</script>

<template>
  <form @submit="submitForm" class="max-w-xl mx-auto flex flex-col gap-8">
    <div class="flex flex-col gap-2">
      <label for="name-input">Your name</label>
      <InputText
        id="name-input"
        v-model="name"
        v-bind="nameAttrs"
        type="text"
        size="large"
        placeholder="Jane"
        aria-describedby="name-help"
      />
      <Message v-if="errors.name" severity="error" size="small" variant="simple">
        {{ errors.name }}
      </Message>
      <Message id="name-help" size="small" severity="secondary" variant="simple">
        The name, nickname, alias, or handle you want to send to the organizers.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="contact-input">Contact info</label>
      <div class="flex max-sm:flex-col gap-2">
        <InputText
          id="contact-input"
          v-model="contact"
          v-bind="contactAttrs"
          type="text"
          size="large"
          placeholder="jane@example.com"
          aria-describedby="contact-help"
          class="grow"
        />
        <Select
          v-model="contactMethod"
          v-bind="contactMethodAttrs"
          :options="props.contactMethods.map(contactMethodByCode)"
          option-label="name"
          option-value="code"
          size="large"
          placeholder="Method"
          class="basis-1/3 max-sm:grow"
        />
      </div>
      <Message v-if="errors.contact" severity="error" size="small" variant="simple">
        {{ errors.contact }}
      </Message>
      <Message v-if="errors.contactMethod" severity="error" size="small" variant="simple">
        {{ errors.contactMethod }}
      </Message>
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        How you want the organizers to contact you.
      </Message>
    </div>

    <div class="flex justify-around">
      <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
      <Button @click="resetForm" severity="secondary" label="Reset" class="max-w-24" />
    </div>
  </form>
</template>

<style scoped></style>
