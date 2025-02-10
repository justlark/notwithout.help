<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";

type Emits = {
  (eventName: "submit", values: FormValues["password"]): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  password: z.string().min(1, { message: "You must provide the password." }),
});

export type FormValues = z.infer<typeof schema>;

const { defineField, errors, handleSubmit } = useForm<FormValues>({
  validationSchema: toTypedSchema(schema),
  initialValues: {
    password: "",
  },
});

const [password, passwordAttrs] = defineField("password");

const submitForm = handleSubmit((values) => {
  emit("submit", values.password);
});
</script>

<template>
  <div class="flex flex-col max-w-2xl gap-6">
    <div class="flex flex-col gap-2">
      <label for="secret-link-password-input" class="flex gap-2">
        <span>What is the password?</span>
        <span class="text-red-600 dark:text-red-500">*</span>
      </label>
      <InputText
        id="secret-link-password-input"
        v-model="password"
        v-bind="passwordAttrs"
        type="text"
        aria-describedby="secret-link-password-help"
      />
      <Message v-if="errors.password" severity="error" size="small" variant="simple">
        {{ errors.password }}
      </Message>
      <span id="secret-link-password-help" class="text-muted-color text-sm font-medium">
        This link is password-protected. Whoever gave you this link should have given you a password
        as well.
      </span>
    </div>
    <Button @click="submitForm" type="submit" severity="primary" label="Submit" class="max-w-24" />
  </div>
</template>

<style scoped></style>
