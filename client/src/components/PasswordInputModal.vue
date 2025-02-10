<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import InputText from "primevue/inputtext";
import Message from "primevue/message";
import Button from "primevue/button";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";

type Emits = {
  (eventName: "submit", values: FormValues["password"]): void;
};

const emit = defineEmits<Emits>();

const schema = z.object({
  password: z.string().min(1),
});

export type FormValues = z.infer<typeof schema>;

const { defineField, handleSubmit } = useForm<FormValues>({
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
          @click="submitForm"
          :disabled="!password"
          type="submit"
          severity="primary"
          label="Submit"
          class="max-w-24"
        />
      </InputGroupAddon>
    </InputGroup>
    <span id="secret-link-password-help" class="text-muted-color text-sm font-medium">
      This link is password-protected. Whoever gave you this link should have given you a password
      as well.
    </span>
  </div>
</template>

<style scoped></style>
