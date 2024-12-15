<script setup lang="ts">
import { useForm } from "vee-validate";
import Textarea from "primevue/textarea";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import MultiSelect from "primevue/multiselect";
import Button from "primevue/button";
import DatePicker from "primevue/datepicker";
import { CONTACT_METHODS } from "@/vars";
import { loadState, persistState } from "@/state";
import { z } from "zod";
import { serializeDate, deserializeDate } from "@/encoding";
import { toTypedSchema } from "@vee-validate/zod";
import { computed, ref, watch } from "vue";

const FORM_STORAGE_KEY = "template";

type Emits = {
  (eventName: "submit", values: FormValues, resetForm: () => void): void;
};

const emit = defineEmits<Emits>();

const props = defineProps<{
  initialValues?: FormValues;
}>();

const schema = z.object({
  title: z.string().min(1, { message: "You must provide a name for your group." }),
  description: z.string().min(1, { message: "You must provide a description." }),
  contactMethods: z
    .array(z.string())
    .nonempty({ message: "You must specify at least one contact method." }),
  expirationDate: z.date().nullish().optional(),
});

export type FormValues = z.infer<typeof schema>;

const initialValues = computed(
  () =>
    props.initialValues ??
    loadState<FormValues>(FORM_STORAGE_KEY, (values) => ({
      title: values.title ?? "",
      description: values.description ?? "",
      contactMethods: values.contactMethods ?? [],
      expirationDate: values.expirationDate ? deserializeDate(values.expirationDate) : undefined,
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

const [title, titleAttrs] = defineField("title");
const [description, descriptionAttrs] = defineField("description");
const [contactMethods, contactMethodsAttrs] = defineField("contactMethods");
const [expirationDate, expirationDateAttrs] = defineField("expirationDate");

watch(values, () => {
  persistState(FORM_STORAGE_KEY, values, (values) => ({
    ...values,
    expirationDate: values.expirationDate ? serializeDate(values.expirationDate) : undefined,
  }));
});

const newCustomContactMethod = ref("");

// If the user refreshes the page, loading the form state from local storage, we want to make sure
// we include any custom contact methods to the multiselect options, while preserving the ordering.
const contactMethodOptions = computed(() => {
  const defaultMethods = new Set<string>(CONTACT_METHODS);
  return [
    ...CONTACT_METHODS,
    ...contactMethods.value.filter((method) => !defaultMethods.has(method)),
  ];
});

const submitForm = handleSubmit((values) => {
  emit("submit", values, resetForm);
});

const resetForm = () => {
  localStorage.removeItem(FORM_STORAGE_KEY);
  resetFormInner({ values: initialValues.value });
};

const addCustomContactMethod = () => {
  if (!newCustomContactMethod.value) {
    return;
  }

  contactMethodOptions.value.push(newCustomContactMethod.value);
  contactMethods.value.push(newCustomContactMethod.value);
  newCustomContactMethod.value = "";
};
</script>

<template>
  <form @submit="submitForm" class="max-w-xl mx-auto flex flex-col gap-8">
    <div class="flex flex-col gap-2">
      <label for="title-input" class="flex gap-2">
        Name
        <span class="required-marker">*</span>
      </label>
      <InputText
        id="title-input"
        v-model="title"
        v-bind="titleAttrs"
        type="text"
        size="large"
        placeholder="My Organization"
        aria-describedby="title-help"
      />
      <Message v-if="errors.title" severity="error" size="small" variant="simple">
        {{ errors.title }}
      </Message>
      <Message id="title-help" size="small" severity="secondary" variant="simple">
        The name of the group or organization you're recruiting for.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="description-input" class="flex gap-2">
        Description
        <span class="required-marker">*</span>
      </label>
      <Textarea
        id="description-input"
        v-model="description"
        v-bind="descriptionAttrs"
        auto-resize
        placeholder="We're a group that…"
        aria-describedby="description-help"
      />
      <Message v-if="errors.description" severity="error" size="small" variant="simple">
        {{ errors.description }}
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
        v-model="contactMethods"
        v-bind="contactMethodsAttrs"
        :options="contactMethodOptions"
        placeholder="Email, Signal, Telegram, etc."
        display="chip"
        size="large"
        aria-describedby="contact-help"
      >
        <template #footer>
          <div class="flex gap-4 m-4">
            <InputText
              v-model="newCustomContactMethod"
              placeholder="Custom"
              size="small"
              class="grow"
            />
            <Button
              @click="addCustomContactMethod"
              :disabled="!newCustomContactMethod"
              label="Add"
              severity="primary"
              class="min-w-16"
              size="small"
            />
          </div>
        </template>
      </MultiSelect>
      <Message v-if="errors.contactMethods" severity="error" size="small" variant="simple">
        {{ errors.contactMethods }}
      </Message>
      <Message id="contact-help" size="small" severity="secondary" variant="simple">
        Specify what contact methods you want respondents to pick from when leaving their contact
        information.
      </Message>
    </div>

    <div class="flex flex-col gap-2">
      <label for="date-input">Expiration date</label>
      <DatePicker
        id="date-input"
        v-model="expirationDate"
        v-bind="expirationDateAttrs"
        size="large"
        aria-describedby="date-help"
        :minDate="new Date()"
        show-button-bar
        icon="pi pi-calendar"
        show-icon
      />
      <Message v-if="errors.expirationDate" severity="error" size="small" variant="simple">
        {{ errors.expirationDate }}
      </Message>
      <Message id="date-help" size="small" severity="secondary" variant="simple">
        All submissions will automatically be permanently deleted after this date.
      </Message>
    </div>

    <div class="flex justify-around">
      <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
      <Button @click="resetForm" severity="secondary" label="Reset" class="max-w-24" />
    </div>
  </form>
</template>

<style>
/*
 * Prevent the date picker panel from filling the width of the input box, making it difficult to
 * navigate with a pointer.
 */
.p-datepicker-panel {
  min-width: 0 !important;
}
</style>

<style scoped>
.required-marker {
  color: var(--p-message-error-color);
}
</style>
