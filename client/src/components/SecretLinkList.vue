<script setup lang="ts">
import Panel from "primevue/panel";
import SecretLinkListItem from "@/components/SecretLinkListItem.vue";
import InputText from "primevue/inputtext";
import SplitButton from "primevue/splitbutton";
import { isDone } from "@/types";
import { computed, ref, watchEffect } from "vue";
import api from "@/api";
import { unsealKeyComment, type FormId, type SecretLinkKey } from "@/crypto";
import { decodeUtf8 } from "@/encoding";
import useAccessToken from "@/composables/useAccessToken";
import usePrivatePrimaryKey from "@/composables/usePrivatePrimaryKey";
import useForm from "@/composables/useForm";

interface SecretKeyInfo {
  comment: string;
  accessedAt: Date;
}

const props = defineProps<{
  formId: FormId;
  secretLinkKey: SecretLinkKey;
}>();

const secretKeys = ref<Array<SecretKeyInfo>>([]);
const count = computed(() => secretKeys.value.length);

const newLinkComment = ref("");

const form = useForm();
const accessToken = useAccessToken();
const privatePrimaryKey = usePrivatePrimaryKey();

const createSecretLink = () => {
  // TODO: Implement this.
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
    accessedAt: new Date(key.accessedAt),
  }));
});
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
            :formId="props.formId"
            :accessedAt="secretKey.accessedAt"
            :isAdmin="false"
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
              size="small"
            />
          </span>
        </div>
      </div>
    </Panel>
  </section>
</template>

<style scoped></style>
