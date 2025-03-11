<script setup lang="ts">
import Dialog from "primevue/dialog";

const showPasswordHelp = defineModel<boolean>();
</script>

<template>
  <Dialog
    class="p-2 mx-4 max-w-2xl"
    v-model:visible="showPasswordHelp"
    modal
    dismissable-mask
    header="Why is setting a password recommended?"
  >
    <ul>
      <li>
        Without a password, <strong>anyone</strong> who gets access to a secret link can view the
        responses.
      </li>
      <li>
        This means the secret link could be exposed through your browser history, particularly if
        you share a computer with other people.
      </li>
      <li>
        If you accidentally paste the wrong link in the wrong place, having a password provides an
        extra layer of protection.
      </li>
    </ul>
  </Dialog>
</template>

<style scoped></style>
