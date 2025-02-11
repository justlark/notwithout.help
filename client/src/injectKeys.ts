import { inject, type Ref } from "vue";

export const passwordKey = Symbol();

export const injectPassword = () => {
  const maybePassword = inject<Ref<string | undefined>>(passwordKey);

  if (maybePassword === undefined) {
    throw new Error("Expected a password to be provided by a parent component, but it was not.");
  }

  return maybePassword;
};
