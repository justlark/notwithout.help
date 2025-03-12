<script setup lang="ts">
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import Message from "primevue/message";
import { generateDicewarePassphrase, SECRET_LINK_PASSPHRASE_WORDS } from "@/crypto";
import { TOAST_INFO_TTL } from "@/vars";
import { toTypedSchema } from "@vee-validate/zod";
import { useToast } from "primevue";
import { useForm } from "vee-validate";
import { z } from "zod";

type Emits = {
  (eventName: "submit", password: FormValues["password"]): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  password: z.string().min(1, { message: "Password cannot be empty." }),
});

export type FormValues = z.infer<typeof schema>;

const { defineField, errors, handleSubmit } = useForm<FormValues>({
  validationSchema: toTypedSchema(schema),
  initialValues: {
    password: "",
  },
});

const [password, passwordAttrs] = defineField("password");

const submitForm = handleSubmit(async (values) => {
  emit("submit", values.password);
});

const toast = useToast();

const generateRandomPassword = () => {
  const newPassword = generateDicewarePassphrase(SECRET_LINK_PASSPHRASE_WORDS);

  password.value = newPassword;

  navigator.clipboard.writeText(newPassword);

  toast.add({
    severity: "info",
    summary: "Password copied",
    detail: `A random ${SECRET_LINK_PASSPHRASE_WORDS}-word passphrase has been copied to your clipboard.`,
    life: TOAST_INFO_TTL,
  });
};
</script>

<template>
  <div class="flex flex-col max-w-2xl gap-2">
    <InputGroup>
      <InputText
        id="secret-link-password-input"
        @keydown.enter="submitForm"
        v-model="password"
        v-bind="passwordAttrs"
        type="password"
        placeholder="Password"
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
      Enter your new password or generate a random one. You'll need this new password to access this
      page going forward. This will generate a new secret link; your old one won't work anymore.
    </span>
    <Button @click="submitForm" type="submit" severity="primary" label="Submit" class="max-w-24" />
  </div>
</template>

<style scoped></style>
