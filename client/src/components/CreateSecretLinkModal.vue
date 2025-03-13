<script setup lang="ts">
import { ref } from "vue";
import SelectButton from "primevue/selectbutton";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import SecretLinkPasswordHelpDialog from "./SecretLinkPasswordHelpDialog.vue";
import { z } from "zod";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import { generateDicewarePassphrase, SECRET_LINK_PASSPHRASE_WORDS } from "@/crypto";
import { useToast } from "primevue";
import { TOAST_INFO_TTL } from "@/vars";
import FormBodyInput from "./FormBodyInput.vue";

type Emits = {
  (eventName: "submit", values: FormValues): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  comment: z.string().min(1, { message: "You must provide a name for the link." }),
  role: z.enum(["admin", "read"], { message: "You must select a link type." }),
  password: z.string().optional(),
});

export type FormValues = z.infer<typeof schema>;

const { defineField, errors, handleSubmit } = useForm<FormValues>({
  validationSchema: toTypedSchema(schema),
  initialValues: {
    comment: "",
    role: undefined,
    password: undefined,
  },
});

const [linkComment, linkCommentAttrs] = defineField("comment");
const [linkRole, linkRoleAttrs] = defineField("role");
const [linkPassword, linkPasswordAttrs] = defineField("password");

const linkTypeOptions = ref([
  { label: "Admin", icon: "pi pi-shield", value: "admin" },
  { label: "Read-only", icon: "pi pi-eye", value: "read" },
]);

const submitForm = handleSubmit((values) => {
  emit("submit", values);
});

const showPasswordHelp = ref(false);

const toast = useToast();

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
</script>

<template>
  <form @submit="submitForm" class="flex flex-col max-w-2xl gap-6">
    <FormBodyInput
      id="secret-link-comment"
      label="Who are you sharing this link with?"
      required
      :error="errors.comment"
    >
      <template #input="{ id, ariaDescribedby }">
        <InputText
          :id="id"
          v-model="linkComment"
          v-bind="linkCommentAttrs"
          type="text"
          placeholder="Alice"
          :aria-describedby="ariaDescribedby"
        />
      </template>
      <template #help>
        Enter the name of the person you're sharing this link with, so you can remember who has
        which link and revoke it later if you need to.
      </template>
    </FormBodyInput>

    <FormBodyInput
      id="secret-link-role"
      label="What kind of link is this?"
      required
      :error="errors.role"
    >
      <template #input="{ id, ariaDescribedby }">
        <SelectButton
          :id="id"
          v-model="linkRole"
          v-bind="linkRoleAttrs"
          :options="linkTypeOptions"
          optionLabel="label"
          optionValue="value"
          dataKey="value"
          :aria-describedby="ariaDescribedby"
        >
          <template #option="slotProps">
            <span class="flex gap-2 items-center">
              <i :class="slotProps.option.icon"></i>
              <span>{{ slotProps.option.label }}</span>
            </span>
          </template>
        </SelectButton>
      </template>
      <template #help>
        Read-only users can't make edits or generate new secret links. Admin users can.
      </template>
    </FormBodyInput>

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
        For added security, you can set a password that the person you send this link to will need
        to know to access the page. They can change their password later, but cannot remove it.
        <Button
          @click="showPasswordHelp = true"
          class="!text-sm !font-medium !p-0"
          variant="link"
          label="Why is this recommended?"
        />
      </template>
    </FormBodyInput>

    <SecretLinkPasswordHelpDialog v-model="showPasswordHelp" />

    <Button type="submit" severity="primary" label="Create" class="max-w-24" />
  </form>
</template>

<style scoped></style>
