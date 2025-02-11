<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import Message from "primevue/message";

const props = defineProps<{
  validator: (password: string) => Promise<boolean>;
}>();

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

const submitForm = handleSubmit(async (values, actions) => {
  if (await props.validator(values.password)) {
    emit("submit", values.password);
  } else {
    actions.setErrors({ password: "That password is incorrect." });
  }
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
    <Message v-if="errors.password" severity="error" size="small" variant="simple">
      {{ errors.password }}
    </Message>
    <span id="secret-link-password-help" class="text-muted-color text-sm font-medium">
      This link is password-protected. Whoever gave you this link should have given you a password
      as well.
    </span>
  </div>
</template>

<style scoped></style>
