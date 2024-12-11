<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import SplitButton from "primevue/splitbutton";
import Dialog from "primevue/dialog";
import { isDone } from "@/types";
import { computed, ref, watchEffect } from "vue";
import api from "@/api";
import {
  deriveKeys,
  generateSecretLinkKey,
  sealKeyComment,
  unsealKeyComment,
  wrapPrivatePrimaryKey,
  type FormId,
  type SecretLinkKey,
} from "@/crypto";
import { decodeUtf8, encodeBase64Url, encodeUtf8 } from "@/encoding";
import useAccessToken from "@/composables/useAccessToken";
import usePrivatePrimaryKey from "@/composables/usePrivatePrimaryKey";
import { useToast } from "primevue";
import { TOAST_ERROR_TTL, TOAST_INFO_TTL } from "@/vars";
import useForm from "@/composables/useForm";

interface SecretKeyInfo {
  comment: string;
  accessedAt: Date | undefined;
}

const props = defineProps<{
  formId: FormId;
  secretLinkKey: SecretLinkKey;
}>();

const toast = useToast();

const secretKeys = ref<Array<SecretKeyInfo>>([]);
const count = computed(() => secretKeys.value.length);

const newLinkComment = ref("");
const newSecretLink = ref<string>();
const newSecretLinkModalIsVisible = computed<boolean>({
  get() {
    return newSecretLink.value !== undefined;
  },

  set(value) {
    if (!value) {
      newSecretLink.value = undefined;
    }
  },
});

const form = useForm();
const accessToken = useAccessToken();
const privatePrimaryKey = usePrivatePrimaryKey();

watchEffect(async () => {
  secretKeys.value = [];

  if (!isDone(accessToken) || !isDone(form) || !isDone(privatePrimaryKey)) {
    return;
  }

  const keys = await api.listKeys({ formId: props.formId, accessToken: accessToken.value.value });

  secretKeys.value = keys.map((key) => ({
    comment: decodeUtf8(
      unsealKeyComment(
        key.encryptedComment,
        form.value.value.publicPrimaryKey,
        privatePrimaryKey.value.value,
      ),
    ),
    accessedAt: key.accessedAt ? new Date(key.accessedAt) : undefined,
  }));
});

const createSecretLink = async () => {
  if (
    !newLinkComment.value ||
    !isDone(accessToken) ||
    !isDone(form) ||
    !isDone(privatePrimaryKey)
  ) {
    return;
  }

  const newSecretLinkKey = generateSecretLinkKey();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  const encryptedComment = sealKeyComment(
    encodeUtf8(newLinkComment.value),
    form.value.value.publicPrimaryKey,
  );
  const wrappedPrivatePrimaryKey = wrapPrivatePrimaryKey(
    privatePrimaryKey.value.value,
    derivedKeys.secretWrappingKey,
  );

  let newClientKeyId;
  try {
    const { clientKeyId } = await api.postKey({
      formId: props.formId,
      publicSigningKey: derivedKeys.publicSigningKey,
      wrappedPrivatePrimaryKey,
      encryptedComment,
      accessToken: accessToken.value.value,
    });

    newClientKeyId = clientKeyId;
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to create secret link",
      detail: "Something unexpected happened.",
      life: TOAST_ERROR_TTL,
    });

    return;
  }

  secretKeys.value.push({
    comment: newLinkComment.value,
    accessedAt: undefined,
  });

  // TODO: Deduplicate logic for formatting secret links.
  newSecretLink.value = `${window.location.origin}/view/#/${props.formId}/${newClientKeyId}/${encodeBase64Url(newSecretLinkKey)}`;
  newLinkComment.value = "";
};

const createSecretAdminLink = () => {
  // TODO: Implement this.
};

const copyNewSecretLink = async () => {
  if (newSecretLink.value) {
    await navigator.clipboard.writeText(newSecretLink.value);
  }

  toast.add({
    severity: "warn",
    summary: "Secret link copied",
    detail: "Be careful who you share this link with.",
    life: TOAST_INFO_TTL,
  });
};

const secretLinkActions = [
  {
    label: "Admin",
    icon: "pi pi-shield",
    command: createSecretAdminLink,
  },
];
</script>

<template>
  <section v-if="count > 0" aria-labelledby="secret-links-list-heading">
    <Panel
      toggleable
      collapsed
      :toggle-button-props="{
        severity: 'secondary',
        text: true,
        rounded: true,
        'aria-label': 'Expand',
      }"
    >
      <template #header>
        <span class="flex items-center gap-3">
          <i class="pi pi-lock !text-xl"></i>
          <h1 id="secret-links-list-heading" class="text-lg mb-0">Secret links ({{ count }})</h1>
        </span>
      </template>
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <SecretLinkListItem
            v-for="(secretKey, index) in secretKeys"
            :key="index"
            :comment="secretKey.comment"
            :form-id="props.formId"
            :accessed-at="secretKey.accessedAt"
            :is-admin="false"
          />
        </div>
        <div class="flex flex-col gap-2">
          <label for="new-link-comment" class="text-sm">Create a new secret link</label>
          <span class="flex gap-8 justify-between">
            <InputText
              id="new-link-comment"
              class="grow"
              v-model="newLinkComment"
              type="text"
              placeholder="Who are you sharing this link with?"
              size="small"
            />
            <SplitButton
              @click="createSecretLink"
              icon="pi pi-plus"
              :button-props="{ 'aria-label': 'Create' }"
              :menu-button-props="{ 'aria-label': 'More options' }"
              :model="secretLinkActions"
              :disabled="!newLinkComment"
              size="small"
            />
          </span>
        </div>
      </div>
    </Panel>
  </section>
  <Dialog v-model:visible="newSecretLinkModalIsVisible" modal header="New secret link">
    <div class="flex gap-8 items-center justify-between max-w-xl">
      <a :href="newSecretLink" target="_blank" class="break-all">{{ newSecretLink }}</a>
      <Button @click="copyNewSecretLink" label="Copy" icon="pi pi-clipboard" class="min-w-24" />
    </div>
  </Dialog>
</template>

<style scoped></style>
