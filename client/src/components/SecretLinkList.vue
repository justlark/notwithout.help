<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import SecretLinkAdmonition from "@/components/SecretLinkAdmonition.vue";
import InputText from "primevue/inputtext";
import SplitButton from "primevue/splitbutton";
import Dialog from "primevue/dialog";
import { isDone } from "@/types";
import { computed, ref, watchEffect } from "vue";
import api, { ApiError, type AccessRole } from "@/api";
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

  try {
    const keys = await api.listKeys({ formId: props.formId, accessToken: accessToken.value.value });
    secretKeys.value = keys.map((key) => ({
      comment: decodeUtf8(
        unsealKeyComment(
          key.encryptedComment,
          form.value.value.publicPrimaryKey,
          privatePrimaryKey.value.value,
        ),
      ),
      role: key.role,
      clientKeyId: key.clientKeyId,
      accessedAt: key.accessedAt ? new Date(key.accessedAt) : undefined,
    }));
  } catch (error) {
    // Users need admin permission to list keys. If they don't have this permission, we don't show
    // them. No need for an error toast.
    if (error instanceof ApiError && error.kind === "forbidden") {
      secretKeys.value = [];
    } else {
      toast.add({
        severity: "error",
        summary: "Failed to list secret links",
        detail: "Something unexpected happened.",
        life: TOAST_ERROR_TTL,
      });

      return;
    }
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
      role,
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
      <p>
        This is a list of links that can be used to access this page. Create new secret links to
        grant access to your fellow organizers, and revoke them at any time.
      </p>
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
  <Dialog class="p-2 mx-4" v-model:visible="newSecretLinkModalIsVisible" modal>
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
