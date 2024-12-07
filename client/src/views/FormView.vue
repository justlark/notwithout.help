<script setup lang="ts">
import api from "@/api";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import { sealSubmissionBody } from "@/crypto";
import { encodeUtf8, parseShareLinkFragment } from "@/encoding";
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const { formId } = parseShareLinkFragment(route.hash);

const body = ref<FormValues>();

const postSubmission = async () => {
  const { publicPrimaryKey } = await api.getForm(formId);

  const encryptedBody = sealSubmissionBody(
    encodeUtf8(JSON.stringify(body.value)),
    publicPrimaryKey,
  );

  api.postSubmission({ formId, encryptedBody });
};
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Get Involved</h1>
    <ResponseForm @submit="postSubmission" v-model="body" />
  </main>
</template>

<style scoped></style>
