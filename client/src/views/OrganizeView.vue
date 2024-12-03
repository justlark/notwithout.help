<script setup lang="ts">
import { computed, ref } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import type { ClientKeyId, FormId, SecretLinkKey } from "@/types";
import { useToast } from "primevue/usetoast";
import { TOAST_TTL } from "@/vars";
import Toast from "primevue/toast";

// TODO: Example of a Form ID.
const formId = ref("VmrfdKsn" as FormId);

// TODO: Example of a Client Key ID.
const clientKeyId = ref("0" as ClientKeyId);

// TODO: Example of a base64url-encoded Secret Link Key.
const secretLinkKey = ref("6IKYnsjFRUkvZfpcLjlaMEAvo61IUO44y8EIcCbV2rM=" as SecretLinkKey);

const origin = computed(() => window.location.origin);
const shareLink = computed(() => new URL(`${origin.value}/share/#/${formId.value}`));
const secretLink = computed(
  () =>
    new URL(`${origin.value}/view/#/${formId.value}/${clientKeyId.value}/${secretLinkKey.value}`),
);

const toast = useToast();

const copyShareLink = () => {
  toast.removeGroup("link-copy");

  toast.add({
    severity: "info",
    summary: "Link copied",
    detail: "Share this link to collect responses.",
    life: TOAST_TTL,
    group: "link-copy",
  });
};

const copySecretLink = () => {
  toast.removeGroup("link-copy");

  toast.add({
    severity: "warn",
    summary: "Secret link copied",
    detail: "Be careful who you share this link with.",
    life: TOAST_TTL,
    group: "link-copy",
  });
};
</script>

<template>
  <Toast position="bottom-center" group="link-copy" />
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
    <div class="max-w-xl mx-auto flex flex-col gap-8">
      <LinkAdmonition :link="shareLink" @click="copyShareLink">
        <template #title>
          <span class="flex gap-3 items-center">
            <i class="pi pi-share-alt"></i>
            <strong>Share this link</strong>
          </span>
        </template>

        <template #details>
          <ul>
            <li>Send this link to anyone you want to fill out your survey.</li>
            <li>
              People with this link can respond to your survey, but cannot view other responses.
            </li>
            <li>
              Copy this link down in a safe place, because it will disappear when you leave this
              page.
            </li>
          </ul>
        </template>
      </LinkAdmonition>

      <LinkAdmonition :link="secretLink" @click="copySecretLink">
        <template #title>
          <span class="flex gap-3 items-center">
            <i class="pi pi-lock"></i>
            <strong>Keep this link secret</strong>
          </span>
        </template>

        <template #details>
          <ul>
            <li>Use this link to view responses to your survey.</li>
            <li>
              This link is like a password. <strong>Anyone</strong> with this link can access your
              survey responses.
            </li>
            <li>
              Copy this link down in a safe place, because it will disappear when you leave this
              page.
            </li>
          </ul>
        </template>
      </LinkAdmonition>
    </div>
  </main>
</template>

<style scoped></style>
