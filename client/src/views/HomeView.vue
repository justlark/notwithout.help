<script setup lang="ts">
import Button from "primevue/button";
import { RouterLink } from "vue-router";
</script>

<template>
  <main aria-labelledby="main-heading" class="flex flex-col gap-8 items-center">
    <h1 id="main-heading" class="text-center mb-0">Not Without Help</h1>
    <Button as="router-link" to="/start" label="Start a Group" />
    <div class="text-justify max-w-3xl flex flex-col">
      <p class="text-center">🚧 This site is under construction 🚧</p>
      <p>
        Not Without Help is a tool for connecting organizations with people who want to help. In
        this early iteration, it's just a form where volunteers can leave their contact information
        for organizers to get in touch.
      </p>
      <h2 class="text-center">How it works</h2>
      <ul>
        <li>
          You'll fill out some information about your group and what kinds of help you're looking
          for.
        </li>
        <li>
          You'll get a link to share around and collect responses from people who want to help.
        </li>
        <li>
          Folks who are interested leave their contact information so you can reach out to them
          directly.
        </li>
      </ul>
      <h2 class="text-center">Privacy</h2>
      <ul>
        <li>
          Only you can see the information people send you. Neither us nor our hosting provider have
          access to it.
        </li>
        <li>
          You don't need any sort of account to use this tool. You don't need to provide a name,
          email address, or phone number to start a group.
        </li>
        <li>
          You have control over who you authorize to view the information people send you, and you
          can revoke that access at any time.
        </li>
      </ul>
    </div>
    <p class="flex justify-center gap-2 text-center">
      <RouterLink to="/contact">Contact</RouterLink>
      <span>•</span>
      <a href="https://github.com/justlark/notwithout.help" target="_blank">GitHub</a>
    </p>
  </main>
</template>

<style scoped></style>
