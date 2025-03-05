<script setup lang="ts">
import { RouterView } from "vue-router";
import Toast from "primevue/toast";
import Dialog from "primevue/dialog";
import { randomTitleLead } from "./vars";
import { computed, provide, ref, watchEffect } from "vue";
import icon from "./assets/icon.svg";
import PasswordInputModal from "./components/PasswordInputModal.vue";
import { passwordKey } from "./injectKeys";
import useSecretLinkKey from "./composables/useSecretLinkKey";
import { returnsError } from "./types";

const password = ref<string>();
provide(passwordKey, password);

const secretLinkKey = useSecretLinkKey(password);

const titleLead = computed(() => randomTitleLead());

const passwordDialogVisible = computed(() =>
  returnsError(["no-password", "invalid-password", "idle-timeout"], secretLinkKey),
);

watchEffect(() => {
  if (returnsError("idle-timeout", secretLinkKey)) {
    password.value = undefined;
  }
});

const submitPassword = (enteredPassword: string) => {
  password.value = enteredPassword;
};
</script>

<template>
  <header class="flex gap-3 items-center mb-8">
    <img :src="icon" alt="A lifebuoy" class="w-14 h-14" />
    <div class="flex flex-col gap-1">
      <span class="text-muted-color">{{ titleLead }}</span>
      <RouterLink to="/" class="text-lg font-medium">Not Without Help</RouterLink>
    </div>
  </header>

  <RouterView />

  <Toast class="max-w-[90vw]" position="bottom-center" />

  <Dialog
    class="p-2 mx-4"
    v-model:visible="passwordDialogVisible"
    modal
    :closable="false"
    aria-labelledby="password-dialog-name"
  >
    <template #header>
      <span class="flex gap-3 text-xl items-center">
        <i class="pi pi-lock"></i>
        <strong id="password-dialog-name">Enter password</strong>
      </span>
    </template>
    <PasswordInputModal
      :is-invalid="returnsError('invalid-password', secretLinkKey)"
      @submit="submitPassword"
    />
  </Dialog>
</template>

<style scoped></style>
