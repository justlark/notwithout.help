<script setup lang="ts">
import { watch } from "vue";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Button from "primevue/button";
import Select from "primevue/select";
import Panel from "primevue/panel";
import FormBody from "./FormBody.vue";
import FormBodyInput from "./FormBodyInput.vue";
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
    <FormBody>
      <FormBodyInput id="name" label="Your name" required :error="errors.name">
        <template #input="{ id, ariaDescribedby }">
          <InputText
            :id="id"
            v-model="name"
            v-bind="nameAttrs"
            type="text"
            size="large"
            placeholder="Alex"
            :aria-describedby="ariaDescribedby"
          />
        </template>
        <template #help>
          The name, nickname, alias, or handle you want to send to the organizers.
        </template>
      </FormBodyInput>

      <FormBodyInput id="contact" label="Contact info" required :error="errors.contact">
        <template #input="{ id, ariaDescribedby }">
          <div class="flex max-sm:flex-col gap-2">
            <InputText
              :id="id"
              v-model="contact"
              v-bind="contactAttrs"
              type="text"
              size="large"
              placeholder="alex@example.com"
              :aria-describedby="ariaDescribedby"
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
        </template>
        <template #help> How you want the organizers to contact you. </template>
      </FormBodyInput>

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

      <FormBodyInput id="comment" label="More info" help="" :error="errors.comment">
        <template #input="{ id, ariaDescribedby }">
          <Textarea
            :id="id"
            v-model="comment"
            v-bind="commentAttrs"
            size="large"
            auto-resize
            placeholder="I'm looking to contribute…"
            :aria-describedby="ariaDescribedby"
          />
        </template>
        <template #help> What kinds of help you're looking to contribute. </template>
      </FormBodyInput>

      <div class="flex justify-around">
        <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
        <Button @click="resetForm" severity="secondary" label="Reset" class="max-w-24" />
      </div>
    </FormBody>
  </form>
</template>

<style scoped></style>
