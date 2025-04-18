<script setup lang="ts">
import { useForm } from "vee-validate";
import Textarea from "primevue/textarea";
import InputText from "primevue/inputtext";
import MultiSelect from "primevue/multiselect";
import RadioButton from "primevue/radiobutton";
import Button from "primevue/button";
import FileUpload, { type FileUploadUploadEvent } from "primevue/fileupload";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import Card from "primevue/card";
import DatePicker from "primevue/datepicker";
import RoleList from "./RoleList.vue";
import Panel from "primevue/panel";
import FormBody from "./FormBody.vue";
import FormBodyInput from "./FormBodyInput.vue";
import SecretLinkPasswordHelpDialog from "./SecretLinkPasswordHelpDialog.vue";
import { CONTACT_METHODS, TOAST_ERROR_TTL, TOAST_INFO_TTL } from "@/vars";
import { deleteState, loadState, persistState } from "@/state";
import { z } from "zod";
import { serializeDate, deserializeDate } from "@/encoding";
import { toTypedSchema } from "@vee-validate/zod";
import { computed, ref, watch, watchEffect } from "vue";
import defaultRoles from "@/assets/default-roles.json";
import { useRouter } from "vue-router";
import { parseRolesFile } from "@/orgRoles";
import type { OrgRole } from "@/api";
import { useToast } from "primevue";
import { isDone, returnsError } from "@/types";
import { generateDicewarePassphrase, SECRET_LINK_PASSPHRASE_WORDS } from "@/crypto";

const toast = useToast();

type Emits = {
  (eventName: "submit", values: FormValues, resetForm: () => void): void;
};

const emit = defineEmits<Emits>();

const props = defineProps<{
  storageKey: string;
  initialValues?: FormValues;
  cancelable?: boolean;
  showPasswordInput?: boolean;
}>();

const schema = z.object({
  title: z.string().min(1, { message: "You must provide a name for your group." }),
  description: z.string().min(1, { message: "You must provide a description." }),
  contactMethods: z
    .array(z.string())
    .nonempty({ message: "You must specify at least one contact method." }),
  expirationDate: z
    .date()
    .nullish()
    .optional()
    .transform((value) => value ?? undefined),
  rolesPreset: z.enum(["none", "default", "custom"]),
  roles: z.array(z.object({ id: z.string(), name: z.string(), details: z.array(z.string()) })),
  password: z.string().optional(),
});

export type FormValues = z.infer<typeof schema>;

const loadInitialValues = () =>
  loadState<FormValues>(props.storageKey, (values) => ({
    title: values.title ?? props.initialValues?.title ?? "",
    description: values.description ?? props.initialValues?.description ?? "",
    contactMethods: values.contactMethods ?? props.initialValues?.contactMethods ?? [],
    expirationDate: values.expirationDate
      ? deserializeDate(values.expirationDate)
      : props.initialValues?.expirationDate,
    rolesPreset: values.rolesPreset ?? props.initialValues?.rolesPreset ?? "none",
    roles: values.roles ?? props.initialValues?.roles ?? [],
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

const [title, titleAttrs] = defineField("title");
const [description, descriptionAttrs] = defineField("description");
const [contactMethods, contactMethodsAttrs] = defineField("contactMethods");
const [expirationDate, expirationDateAttrs] = defineField("expirationDate");
const [rolesPreset, rolesPresetAttrs] = defineField("rolesPreset");
const [roles] = defineField("roles");
const [linkPassword, linkPasswordAttrs] = defineField("password");

const customRolesFile = ref<string>();
const customRoles = ref<Array<OrgRole>>();

watch([rolesPreset, customRoles], ([newPreset]) => {
  if (newPreset === "default") {
    roles.value = defaultRoles;
  } else if (newPreset === "custom") {
    roles.value = customRoles.value ?? [];
  } else if (newPreset === "none") {
    roles.value = [];
  }
});

const stopPersistStateWatch = watch(values, () => {
  persistState(props.storageKey, values, (values) => ({
    ...values,
    expirationDate: values.expirationDate ? serializeDate(values.expirationDate) : undefined,
  }));
});

const newCustomContactMethod = ref("");

const showPasswordHelp = ref(false);

const generateRandomPassword = () => {
  const password = generateDicewarePassphrase(SECRET_LINK_PASSPHRASE_WORDS);

  linkPassword.value = password;

  navigator.clipboard.writeText(password);

  toast.add({
    severity: "info",
    summary: "Password copied",
    detail: `A random ${SECRET_LINK_PASSPHRASE_WORDS}-word passphrase has been copied to your clipboard.`,
    life: TOAST_INFO_TTL,
  });
};

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
  const finalizeForm = () => {
    stopPersistStateWatch();
    resetForm();
  };

  emit("submit", values, finalizeForm);
});

const router = useRouter();

const cancelForm = () => {
  router.back();
  resetForm();
};

const resetForm = () => {
  deleteState(props.storageKey);
  resetFormInner({ values: props.initialValues ?? loadInitialValues() });
};

const addCustomContactMethod = () => {
  if (!newCustomContactMethod.value) {
    return;
  }

  contactMethodOptions.value.push(newCustomContactMethod.value);
  contactMethods.value.push(newCustomContactMethod.value);
  newCustomContactMethod.value = "";
};

watchEffect(() => {
  if (customRolesFile.value === undefined) {
    customRoles.value = undefined;
    return;
  }

  const parsedRoles = parseRolesFile(customRolesFile.value);

  if (returnsError("empty", parsedRoles)) {
    toast.add({
      severity: "error",
      summary: "Failed to upload custom roles",
      detail: "This file is empty!",
      life: TOAST_ERROR_TTL,
    });

    customRoles.value = undefined;
    return;
  }

  if (returnsError("duplicates", parsedRoles)) {
    toast.add({
      severity: "error",
      summary: "Failed to upload custom roles",
      detail: "You have two or more roles with the same name in this file.",
      life: TOAST_ERROR_TTL,
    });

    customRoles.value = undefined;
    return;
  }

  if (isDone(parsedRoles)) {
    toast.add({
      severity: "success",
      summary: "Successfully uploaded custom roles",
      detail: "You can now preview them below.",
      life: TOAST_INFO_TTL,
    });

    customRoles.value = parsedRoles.value;
    return;
  }
});

const uploadCustomRoles = async (event: Pick<FileUploadUploadEvent, "files">) => {
  const file = Array.isArray(event.files) ? event.files[0] : event.files;
  const buf = await file.arrayBuffer();

  try {
    const text = new TextDecoder("utf-8", { fatal: true }).decode(new Uint8Array(buf));
    customRolesFile.value = text;
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to upload custom roles",
      detail: "This file is not a text file!",
      life: TOAST_ERROR_TTL,
    });

    customRolesFile.value = undefined;
    customRoles.value = undefined;
  }
};
</script>

<template>
  <form @submit="submitForm">
    <div>
      <slot name="lead" />
    </div>
    <FormBody>
      <FormBodyInput id="title" label="Name" required :error="errors.title">
        <template #input="{ id, ariaDescribedby }">
          <InputText
            :id="id"
            v-model="title"
            v-bind="titleAttrs"
            type="text"
            size="large"
            placeholder="My Organization"
            :aria-describedby="ariaDescribedby"
          />
        </template>
        <template #help>
          The name of the group or organization you're recruiting for. You can change this later.
        </template>
      </FormBodyInput>

      <FormBodyInput id="description" label="Description" required :error="errors.description">
        <template #input="{ id, ariaDescribedby }">
          <Textarea
            :id="id"
            v-model="description"
            v-bind="descriptionAttrs"
            auto-resize
            placeholder="We're a group that…"
            :aria-describedby="ariaDescribedby"
          />
        </template>
        <template #help>
          Provide some information about your group or organization and what you're looking for. You
          can change this later.
        </template>
      </FormBodyInput>

      <FormBodyInput id="contact" label="Contact methods" required :error="errors.contactMethods">
        <template #input="{ id, ariaDescribedby }">
          <MultiSelect
            :id="id"
            v-model="contactMethods"
            v-bind="contactMethodsAttrs"
            :options="contactMethodOptions"
            placeholder="Email, Signal, Telegram, etc."
            display="chip"
            size="large"
            :aria-describedby="ariaDescribedby"
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
        </template>
        <template #help>
          Specify what contact methods you want respondents to pick from when leaving their contact
          information. If you're an activist, consider using Signal over unencrypted messaging apps.
          You can change this later.
        </template>
      </FormBodyInput>

      <FormBodyInput id="roles-preset" label="Roles" :error="errors.rolesPreset">
        <template #input>
          <Card>
            <template #content>
              <div class="flex flex-col gap-2">
                <div class="flex gap-3 items-center">
                  <RadioButton
                    inputId="roles-preset-none-input"
                    v-model="rolesPreset"
                    v-bind="rolesPresetAttrs"
                    value="none"
                    size="large"
                  />
                  <label class="font-medium" for="roles-preset-none-input">None</label>
                </div>
                <span>Don't offer respondents any roles to choose from.</span>
              </div>
            </template>
          </Card>
          <Card>
            <template #content>
              <div class="flex flex-col gap-2">
                <div class="flex gap-3 items-center">
                  <RadioButton
                    inputId="roles-preset-default-input"
                    v-model="rolesPreset"
                    v-bind="rolesPresetAttrs"
                    value="default"
                    size="large"
                  />
                  <label class="font-medium" for="roles-preset-default-input">Default</label>
                </div>
                <span>
                  Offer a preset list of roles aimed at activist groups. Credit to
                  <a
                    href="https://drdevonprice.substack.com/p/burning-it-all-down-without-burning?open=false#%C2%A7figure-out-your-activist-character-class"
                    target="_blank"
                    >Devon Price</a
                  >
                  for this list.
                </span>
                <Panel
                  v-if="rolesPreset === 'default'"
                  header="Preview default roles"
                  toggleable
                  collapsed
                >
                  <RoleList :roles="defaultRoles" preview />
                </Panel>
              </div>
            </template>
          </Card>
          <Card>
            <template #content>
              <div class="flex flex-col gap-2">
                <div class="flex justify-between items-center gap-2">
                  <div class="flex flex-col gap-2">
                    <div class="flex gap-3 items-center">
                      <RadioButton
                        inputId="roles-preset-custom-input"
                        v-model="rolesPreset"
                        v-bind="rolesPresetAttrs"
                        value="custom"
                        size="large"
                      />
                      <label class="font-medium" for="roles-preset-custom-input">Custom</label>
                    </div>
                    <span>
                      Upload your own list of roles. See
                      <a
                        href="https://github.com/justlark/notwithout.help/blob/main/docs/custom-roles.md"
                        target="_blank"
                        >here</a
                      >
                      for instructions on how to do this.
                    </span>
                  </div>
                  <FileUpload
                    v-if="rolesPreset === 'custom'"
                    mode="basic"
                    chooseLabel="Upload"
                    chooseIcon="pi pi-upload"
                    custom-upload
                    auto
                    @uploader="uploadCustomRoles"
                  />
                </div>
                <Panel
                  v-if="rolesPreset === 'custom' && roles.length > 0"
                  header="Preview custom roles"
                  toggleable
                  collapsed
                >
                  <RoleList :roles="roles" preview />
                </Panel>
              </div>
            </template>
          </Card>
        </template>
        <template #help>
          Show respondents a list of examples of roles they could have in your organization and let
          them pick which they're interested in. You can change this later.
        </template>
      </FormBodyInput>

      <FormBodyInput id="date" label="Expiration date" :error="errors.expirationDate">
        <template #input="{ id, ariaDescribedby }">
          <DatePicker
            :id="id"
            v-model="expirationDate"
            v-bind="expirationDateAttrs"
            size="large"
            :aria-describedby="ariaDescribedby"
            :minDate="new Date()"
            show-button-bar
            icon="pi pi-calendar"
            show-icon
          />
        </template>
        <template #help>
          All submissions will automatically be permanently deleted after this date. This is an
          optional security feature. At any time, you can push the date back, move it up, or disable
          the feature entirely.
        </template>
      </FormBodyInput>

      <div v-if="showPasswordInput === true" class="flex flex-col gap-2">
        <FormBodyInput
          id="secret-link-password"
          label="Set a password (recommended)"
          :error="errors.password"
        >
          <template #input="{ id, ariaDescribedby }">
            <InputGroup>
              <InputText
                :id="id"
                v-model="linkPassword"
                v-bind="linkPasswordAttrs"
                type="password"
                :aria-describedby="ariaDescribedby"
              />
              <InputGroupAddon>
                <Button
                  @click="generateRandomPassword"
                  severity="primary"
                  label="Random"
                  icon="pi pi-refresh"
                />
              </InputGroupAddon>
            </InputGroup>
          </template>
          <template #help>
            To view responses, you'll have a secret link that only you know. For added security, you
            can protect this link with a password. You can change the password later, but you can't
            remove it once set.
            <Button
              @click="showPasswordHelp = true"
              class="!text-sm !font-medium !p-0"
              variant="link"
              label="Why is this recommended?"
            />
          </template>
        </FormBodyInput>
      </div>

      <SecretLinkPasswordHelpDialog v-model="showPasswordHelp" />

      <div class="flex justify-around">
        <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
        <Button
          v-if="props.cancelable"
          @click="cancelForm"
          severity="secondary"
          label="Cancel"
          class="max-w-24"
        />
        <Button @click="resetForm" severity="secondary" label="Reset" class="max-w-24" />
      </div>
    </FormBody>
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

<style scoped></style>
