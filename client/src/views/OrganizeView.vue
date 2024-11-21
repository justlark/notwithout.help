<script setup lang="ts">
import { computed, ref } from "vue";
import LinkAdmonition from "@/components/LinkAdmonition.vue";
import type { SurveyId, SurveySecret } from "@/types";

// TODO: These are example values.
const surveyId = ref<SurveyId>("VmrfdKsn");
const surveySecret = ref<SurveySecret>("kqCmJEdzk7pK5OiS71ZKHbyYgP5sXzg0");

const origin = computed(() => window.location.origin);
const shareLink = computed(() => new URL(`${origin.value}/share/#/${surveyId.value}`));
const secretLink = computed(
  () => new URL(`${origin.value}/view/#/${surveyId.value}/${surveySecret.value}`),
);
</script>

<template>
  <main aria-labelledby="main-heading">
    <h1 id="main-heading" class="text-center mb-10">Organize a group</h1>
    <div class="max-w-xl mx-auto flex flex-col gap-8">
      <LinkAdmonition :link="shareLink" summary="Share this link" icon="pi pi-share-alt">
        <ul>
          <li>Send this link to anyone you want to fill out your survey.</li>
          <li>
            People with this link can respond to your survey, but cannot view other responses.
          </li>
          <li>
            Copy this link down in a safe place, because it will disappear when you leave this page.
          </li>
        </ul>
      </LinkAdmonition>
      <LinkAdmonition :link="secretLink" summary="Keep this link secret" icon="pi pi-lock">
        <ul>
          <li>Use this link to view responses to your survey.</li>
          <li>
            This link is like a password. <strong>Anyone</strong> with this link can access your
            survey responses.
          </li>
          <li>
            Copy this link down in a safe place, because it will disappear when you leave this page.
          </li>
        </ul>
      </LinkAdmonition>
    </div>
  </main>
</template>

<style scoped></style>
