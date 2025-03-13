<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { useForm } from "vee-validate";
import { z } from "zod";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import InputGroup from "primevue/inputgroup";
import InputGroupAddon from "primevue/inputgroupaddon";
import FormBodyInput from "./FormBodyInput.vue";
import { watchEffect } from "vue";

const props = defineProps<{
  isInvalid: boolean;
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

const submitForm = handleSubmit(async (values) => {
  emit("submit", values.password);
});

watchEffect(() => {
  if (props.isInvalid) {
    errors.value.password = "That password is incorrect.";
  }
});
</script>

<template>
  <FormBodyInput id="secret-link-password" :error="errors.password">
    <template #input="{ id, ariaDescribedby }">
      <div class="max-w-2xl">
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
              @click="submitForm"
              :disabled="!password"
              type="submit"
              severity="primary"
              label="Submit"
              class="max-w-24"
            />
          </InputGroupAddon>
        </InputGroup>
      </div>
    </template>
    <template #help>
      This link is password-protected. Whoever gave you this link should have given you a password
      as well.
    </template>
  </FormBodyInput>
</template>

<style scoped></style>
