<script setup lang="ts">
import { ref } from "vue";
import SelectButton from "primevue/selectbutton";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
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
  <div class="flex flex-col max-w-2xl gap-6">
    <div class="flex flex-col gap-2">
      <label for="secret-link-comment-input" class="flex gap-2">
        <span>Who are you sharing this link with?</span>
        <span class="text-red-600 dark:text-red-500">*</span>
      </label>
      <InputText
        id="secret-link-comment-input"
        v-model="linkComment"
        v-bind="linkCommentAttrs"
        type="text"
        placeholder="Alice"
        aria-describedby="secret-link-comment-help"
      />
      <Message v-if="errors.comment" severity="error" size="small" variant="simple">
        {{ errors.comment }}
      </Message>
      <span id="secret-link-comment-help" class="text-muted-color text-sm font-medium">
        Enter the name of the person you're sharing this link with, so you can remember who has
        which link and revoke it later if you need to.
      </span>
    </div>
    <div class="flex flex-col gap-2">
      <label for="secret-link-role-input" class="flex gap-2">
        <span>What kind of link is this?</span>
        <span class="text-red-600 dark:text-red-500">*</span>
      </label>
      <SelectButton
        id="secret-link-role-input"
        v-model="linkRole"
        v-bind="linkRoleAttrs"
        :options="linkTypeOptions"
        optionLabel="label"
        optionValue="value"
        dataKey="value"
        aria-describedby="secret-link-role-help"
      >
        <template #option="slotProps">
          <span class="flex gap-2 items-center">
            <i :class="slotProps.option.icon"></i>
            <span>{{ slotProps.option.label }}</span>
          </span>
        </template>
      </SelectButton>
      <Message v-if="errors.role" severity="error" size="small" variant="simple">
        {{ errors.role }}
      </Message>
      <span id="secret-link-role-help" class="text-muted-color text-sm font-medium">
        Read-only users can't make edits or generate new secret links. Admin users can.
      </span>
    </div>
    <div class="flex flex-col gap-2">
      <label for="secret-link-password-input">Set a password (recommended)</label>
      <InputGroup>
        <InputText
          id="secret-link-password-input"
          v-model="linkPassword"
          v-bind="linkPasswordAttrs"
          type="password"
          aria-describedby="secret-link-password-help"
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
      <Message v-if="errors.password" severity="error" size="small" variant="simple">
        {{ errors.password }}
      </Message>
      <span id="secret-link-password-help" class="text-muted-color text-sm font-medium">
        For added security, you can set a password that the person you send this link to will need
        to know to access the page.
        <Button
          @click="showPasswordHelp = true"
          class="!text-sm !font-medium !p-0"
          variant="link"
          label="Why is this recommended?"
        />
      </span>
    </div>
    <SecretLinkPasswordHelpDialog v-model="showPasswordHelp" />
    <Button @click="submitForm" type="submit" severity="primary" label="Create" class="max-w-24" />
  </div>
</template>

<style scoped></style>
