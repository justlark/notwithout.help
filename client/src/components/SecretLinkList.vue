<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import SecretLinkAdmonition from "@/components/SecretLinkAdmonition.vue";
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

  const keys = await api.listKeys({ formId: props.formId, accessToken: accessToken.value.value });

  secretKeys.value = keys.map((key) => ({
    comment: decodeUtf8(
      unsealKeyComment(
        key.encryptedComment,
        form.value.value.publicPrimaryKey,
        privatePrimaryKey.value.value,
      ),
    ),
    clientKeyId: key.clientKeyId,
    accessedAt: key.accessedAt ? new Date(key.accessedAt) : undefined,
  }));
});

const createSecretLink = async () => {
  if (
    !newSecretLinkComment.value ||
    !isDone(accessToken) ||
    !isDone(form) ||
    !isDone(privatePrimaryKey)
  ) {
    return;
  }

  const newSecretLinkKey = generateSecretLinkKey();
  const derivedKeys = await deriveKeys(newSecretLinkKey);

  const encryptedComment = sealKeyComment(
    encodeUtf8(newSecretLinkComment.value),
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
    comment: newSecretLinkComment.value,
    clientKeyId: newClientKeyId,
    accessedAt: undefined,
  });

  newSecretLinkParts.value = {
    clientKeyId: newClientKeyId,
    secretLinkKey: newSecretLinkKey,
  };

  newSecretLinkComment.value = "";
};

const createSecretAdminLink = () => {
  // TODO: Implement this.
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
            :client-key-id="secretKey.clientKeyId"
            :active-client-key-id="props.clientKeyId"
            :accessed-at="secretKey.accessedAt"
            :is-admin="false"
            :count="count"
          />
        </div>
        <div class="flex flex-col gap-2">
          <label for="new-link-comment" class="text-sm">Create a new secret link</label>
          <span class="flex gap-8 justify-between">
            <InputText
              id="new-link-comment"
              class="grow"
              v-model="newSecretLinkComment"
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
              :disabled="!newSecretLinkComment"
              size="small"
            />
          </span>
        </div>
      </div>
    </Panel>
  </section>
  <Dialog class="p-2" v-model:visible="newSecretLinkModalIsVisible" modal>
    <template #header>
      <span class="flex gap-3 items-center">
        <i class="pi pi-lock"></i>
        <strong>Keep this link secret</strong>
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
