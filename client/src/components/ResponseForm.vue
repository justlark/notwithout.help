<script setup lang="ts">
import { watch } from "vue";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Message from "primevue/message";
import Button from "primevue/button";
import Select from "primevue/select";
import Panel from "primevue/panel";
import { z } from "zod";
import { deleteState, loadState, persistState } from "@/state";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import RoleList from "./RoleList.vue";
import type { OrgRole } from "@/api";

const props = defineProps<{
  storageKey: string;
  contactMethods: Array<string>;
  roles: Array<OrgRole>;
}>();

type Emits = {
  (eventName: "submit", values: FormValues, resetForm: () => void): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  name: z.string().min(1, { message: "You must provide a name." }),
  contact: z.string().min(1, { message: "You must provide a way to contact you." }),
  contactMethod: z.string({
    message: "You must provide a preferred contact method.",
  }),
  comment: z.string().optional(),
  roles: z.array(z.string()),
});

export type FormValues = z.infer<typeof schema>;

const loadInitialValues = () =>
  loadState<FormValues>(props.storageKey, (values) => ({
    name: values.name ?? "",
    contact: values.contact ?? "",
    contactMethod: values.contactMethod,
    comment: values.comment ?? "",
    roles: values.roles ?? [],
  }));

const {
  values,
  errors,
  defineField,
  resetForm: resetFormInner,
  handleSubmit,
} = useForm<FormValues>({
  validationSchema: toTypedSchema(schema),
  initialValues: loadInitialValues(),
});

const [name, nameAttrs] = defineField("name");
const [contact, contactAttrs] = defineField("contact");
const [contactMethod, contactMethodAttrs] = defineField("contactMethod");
const [comment, commentAttrs] = defineField("comment");
const [selectedRoleIds, selectedRoleIdsAttrs] = defineField("roles");

watch(values, () => {
  persistState(props.storageKey, values);
});

const submitForm = handleSubmit((values) => {
  emit("submit", values, resetForm);
});

const resetForm = () => {
  deleteState(props.storageKey);
  resetFormInner({ values: loadInitialValues() });
};
</script>

<template>
  <form @submit="submitForm">
    <div>
      <slot name="lead" />
    </div>
    <div class="max-w-xl mx-auto flex flex-col gap-8">
      <div class="flex flex-col gap-2">
        <label for="name-input" class="flex gap-2">
          <span>Your name</span>
          <span class="text-red-600 dark:text-red-500">*</span>
        </label>
        <InputText
          id="name-input"
          v-model="name"
          v-bind="nameAttrs"
          type="text"
          size="large"
          placeholder="Alex"
          aria-describedby="name-help"
        />
        <Message v-if="errors.name" severity="error" size="small" variant="simple">
          {{ errors.name }}
        </Message>
        <span id="name-help" class="text-muted-color text-sm font-medium">
          The name, nickname, alias, or handle you want to send to the organizers.
        </span>
      </div>

      <div class="flex flex-col gap-2">
        <label for="contact-input" class="flex gap-2">
          <span>Contact info</span>
          <span class="text-red-600 dark:text-red-500">*</span>
        </label>
        <div class="flex max-sm:flex-col gap-2">
          <InputText
            id="contact-input"
            v-model="contact"
            v-bind="contactAttrs"
            type="text"
            size="large"
            placeholder="alex@example.com"
            aria-describedby="contact-help"
            class="grow"
          />
          <Select
            v-model="contactMethod"
            v-bind="contactMethodAttrs"
            :options="props.contactMethods"
            size="large"
            class="basis-1/3 max-sm:grow"
          />
        </div>
        <Message v-if="errors.contact" severity="error" size="small" variant="simple">
          {{ errors.contact }}
        </Message>
        <Message v-if="errors.contactMethod" severity="error" size="small" variant="simple">
          {{ errors.contactMethod }}
        </Message>
        <span id="contact-help" class="text-muted-color text-sm font-medium">
          How you want the organizers to contact you.
        </span>
      </div>

      <div v-if="props.roles.length > 0" class="flex flex-col gap-2">
        <Panel header="What roles are you interested in?">
          <RoleList
            v-model="selectedRoleIds"
            id="role-input"
            :roles="props.roles"
            :inputAttrs="selectedRoleIdsAttrs"
          />
        </Panel>
      </div>

      <div class="flex flex-col gap-2">
        <label for="contact-input">More info</label>
        <Textarea
          id="comment-input"
          v-model="comment"
          v-bind="commentAttrs"
          size="large"
          auto-resize
          placeholder="I'm looking to contribute…"
          aria-describedby="comment-help"
        />
        <Message v-if="errors.comment" severity="error" size="small" variant="simple">
          {{ errors.comment }}
        </Message>
        <span id="contact-help" class="text-muted-color text-sm font-medium">
          What kinds of help you're looking to contribute.
        </span>
      </div>

      <div class="flex justify-around">
        <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
        <Button @click="resetForm" severity="secondary" label="Reset" class="max-w-24" />
      </div>
    </div>
  </form>
</template>

<style scoped></style>
