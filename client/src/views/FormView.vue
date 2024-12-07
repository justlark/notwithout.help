<script setup lang="ts">
import api from "@/api";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import { sealSubmissionBody, type PublicPrimaryKey } from "@/crypto";
import { encodeUtf8, parseShareLinkFragment } from "@/encoding";
import type { ContactMethodCode } from "@/vars";
import { onBeforeMount } from "vue";
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const { formId } = parseShareLinkFragment(route.hash);

const body = ref<FormValues>();

const orgName = ref("");
const description = ref("");
const contactMethods = ref<Array<ContactMethodCode>>([]);
const publicPrimaryKey = ref<PublicPrimaryKey>();

const postSubmission = async () => {
  if (!publicPrimaryKey.value) {
    return;
  }

  const encryptedBody = sealSubmissionBody(
    encodeUtf8(JSON.stringify(body.value)),
    publicPrimaryKey.value,
  );

  api.postSubmission({ formId, encryptedBody });
};

onBeforeMount(async () => {
  const formData = await api.getForm(formId);

  orgName.value = formData.orgName;
  description.value = formData.description;
  contactMethods.value = formData.contactMethods;
  publicPrimaryKey.value = formData.publicPrimaryKey;
});
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">{{ orgName }}</h1>
    <p class="text-jusitfy max-w-xl mx-auto">{{ description }}</p>
    <ResponseForm @submit="postSubmission" v-model="body" :contact-methods="contactMethods" />
  </main>
</template>

<style scoped></style>
