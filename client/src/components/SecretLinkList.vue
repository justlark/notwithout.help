<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import SecretLinkAdmonition from "@/components/SecretLinkAdmonition.vue";
import Divider from "primevue/divider";
import InputText from "primevue/inputtext";
import SplitButton from "primevue/splitbutton";
import Dialog from "primevue/dialog";
import { isDone } from "@/types";
import { computed, ref, watchEffect } from "vue";
import api, { type AccessRole } from "@/api";
import {
  deriveKeys,
  generateSecretLinkKey,
  sealKeyComment,
  unsealKeyComment,
  wrapPrivatePrimaryKey,
  type ClientKeyId,
  type FormId,
  type SecretLinkKey,
} from "@/crypto";
import { decodeUtf8, encodeUtf8 } from "@/encoding";
import useAccessToken from "@/composables/useAccessToken";
import usePrivatePrimaryKey from "@/composables/usePrivatePrimaryKey";
import { useToast } from "primevue";
import { TOAST_ERROR_TTL } from "@/vars";
import useForm from "@/composables/useForm";

interface SecretKeyInfo {
  comment: string;
  clientKeyId: ClientKeyId;
  role: AccessRole;
  accessedAt: Date | undefined;
}

const props = defineProps<{
  formId: FormId;
  clientKeyId: ClientKeyId;
  secretLinkKey: SecretLinkKey;
}>();

const toast = useToast();

const secretKeys = ref<Array<SecretKeyInfo>>([]);
const count = computed(() => secretKeys.value.length);

const newSecretLinkComment = ref("");
const newSecretLinkParts = ref<{ clientKeyId: ClientKeyId; secretLinkKey: SecretLinkKey }>();
const newSecretLinkModalIsVisible = computed<boolean>({
  get() {
    return newSecretLinkParts.value !== undefined;
  },

  set(value) {
    if (!value) {
      newSecretLinkParts.value = undefined;
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

  const { token } = accessToken.value.value;

  try {
    const keys = await api.listKeys({ formId: props.formId, accessToken: token });
    secretKeys.value = await Promise.all(
      keys.map(async (key) => ({
        comment: decodeUtf8(
          await unsealKeyComment(
            key.encryptedComment,
            form.value.value.publicPrimaryKey,
            privatePrimaryKey.value.value,
          ),
        ),
        role: key.role,
        clientKeyId: key.clientKeyId,
        accessedAt: key.accessedAt ? new Date(key.accessedAt) : undefined,
      })),
    );
  } catch {
    toast.add({
      severity: "error",
      summary: "Failed to list secret links",
      detail: "Something unexpected happened.",
      life: TOAST_ERROR_TTL,
    });
  }
});

const createSecretLink = async (role: AccessRole) => {
  if (
    !newSecretLinkComment.value ||
    !isDone(accessToken) ||
    !isDone(form) ||
    !isDone(privatePrimaryKey)
  ) {
    return;
  }

  const { token } = accessToken.value.value;

  const newSecretLinkKey = await generateSecretLinkKey();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  const encryptedComment = await sealKeyComment(
    encodeUtf8(newSecretLinkComment.value),
    form.value.value.publicPrimaryKey,
  );
  const wrappedPrivatePrimaryKey = await wrapPrivatePrimaryKey(
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
      role,
      accessToken: token,
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
    comment: newSecretLinkComment.value,
    role,
    clientKeyId: newClientKeyId,
    accessedAt: undefined,
  });

  newSecretLinkParts.value = {
    clientKeyId: newClientKeyId,
    secretLinkKey: newSecretLinkKey,
  };

  newSecretLinkComment.value = "";
};

const removeSecretLinkFromList = (index: number) => {
  secretKeys.value.splice(index, 1);
};

const secretLinkActions = [
  {
    label: "Read-only",
    icon: "pi pi-tag",
    command: async () => createSecretLink("read"),
  },
];
</script>

<template>
  <section aria-labelledby="secret-links-list-heading">
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
      <ul>
        <li>Create a new secret link for each person you want to give access to this page.</li>
        <li>
          Links can be made read-only so they can't be used to make edits or generate new secret
          links.
        </li>
        <li>Links can be revoked (disabled) when they're lost or no longer needed.</li>
      </ul>
      <Divider />
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <SecretLinkListItem
            v-for="(secretKey, index) in secretKeys"
            :key="index"
            :index="index"
            :comment="secretKey.comment"
            :form-id="props.formId"
            :client-key-id="secretKey.clientKeyId"
            :active-client-key-id="props.clientKeyId"
            :accessed-at="secretKey.accessedAt"
            :role="secretKey.role"
            :count="count"
            @revoke="removeSecretLinkFromList"
          />
        </div>
        <div class="flex flex-col gap-2">
          <label for="new-link-comment" class="text-sm">Create a new secret link</label>
          <span class="flex flex-col sm:flex-row gap-x-8 gap-y-2 justify-between">
            <InputText
              id="new-link-comment"
              class="grow"
              v-model="newSecretLinkComment"
              type="text"
              placeholder="Who are you sharing this link with?"
              size="small"
            />
            <SplitButton
              class="self-end"
              @click="createSecretLink('admin')"
              icon="pi pi-plus"
              :button-props="{ 'aria-label': 'Create' }"
              :menu-button-props="{ 'aria-label': 'More options' }"
              :model="secretLinkActions"
              :disabled="!newSecretLinkComment"
              size="small"
            />
          </span>
        </div>
      </div>
    </Panel>
  </section>
  <Dialog
    class="p-2 mx-4"
    v-model:visible="newSecretLinkModalIsVisible"
    modal
    aria-labelledby="share-link-dialog-name"
  >
    <template #header>
      <span class="flex gap-3 text-xl items-center">
        <i class="pi pi-lock"></i>
        <strong id="share-link-dialog-name">Keep this link secret</strong>
      </span>
    </template>
    <SecretLinkAdmonition
      v-if="newSecretLinkParts"
      :form-id="props.formId"
      :client-key-id="newSecretLinkParts.clientKeyId"
      :secret-link-key="newSecretLinkParts.secretLinkKey"
    />
  </Dialog>
</template>

<style scoped></style>
