<script setup lang="ts">
import api, { type SubmissionBody } from "@/api";
import ResponseForm, { type FormValues } from "@/components/ResponseForm.vue";
import { sealSubmissionBody, type PublicPrimaryKey } from "@/crypto";
import { encodeUtf8, parseShareLinkFragment } from "@/encoding";
import type { ContactMethodCode } from "@/vars";
import { computed, watchEffect } from "vue";
import { ref } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const linkParts = computed(() => parseShareLinkFragment(route.hash));

const orgName = ref("");
const description = ref("");
const contactMethods = ref<Array<ContactMethodCode>>([]);
const publicPrimaryKey = ref<PublicPrimaryKey>();

const postSubmission = async (values: FormValues) => {
  if (!publicPrimaryKey.value) {
    return;
  }

  const submissionBody: SubmissionBody = {
    name: values.name,
    contact: values.contact,
    contact_method: values.contactMethod,
  };

  const encryptedBody = sealSubmissionBody(
    encodeUtf8(JSON.stringify(submissionBody)),
    publicPrimaryKey.value,
  );

  const { formId } = linkParts.value;

  api.postSubmission({ formId, encryptedBody });
};

watchEffect(async () => {
  const { formId } = linkParts.value;

  const formData = await api.getForm({ formId });

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
    <ResponseForm @submit="postSubmission" :contact-methods="contactMethods" />
  </main>
</template>

<style scoped></style>
