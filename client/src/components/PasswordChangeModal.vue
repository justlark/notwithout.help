<script setup lang="ts">
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import FormBodyInput from "./FormBodyInput.vue";
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
  <form @submit="submitForm" class="flex flex-col max-w-2xl gap-2">
    <FormBodyInput id="secret-link-password" :error="errors.password">
      <template #input="{ id, ariaDescribedby }">
        <InputGroup>
          <InputText
            :id="id"
            @keydown.enter="submitForm"
            v-model="password"
            v-bind="passwordAttrs"
            type="password"
            placeholder="Password"
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
        Enter your new password or generate a random one. You'll need this new password to access
        this page going forward. This will generate a new secret link; your old one won't work
        anymore.
      </template>
    </FormBodyInput>

    <Button type="submit" severity="primary" label="Submit" class="max-w-24" />
  </form>
</template>

<style scoped></style>
