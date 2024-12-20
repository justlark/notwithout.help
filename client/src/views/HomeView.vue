<script setup lang="ts">
import Button from "primevue/button";
import Card from "primevue/card";
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
      <h2 class="text-center my-4">How it works</h2>
      <ul class="list-none flex flex-col gap-4 px-0">
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-pencil text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  You'll fill out some information about your group and what kinds of help you're
                  looking for.
                </span>
              </div>
            </template>
          </Card>
        </li>
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-link text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  You'll get a link to share around and collect responses from people who want to
                  help.
                </span>
              </div>
            </template>
          </Card>
        </li>
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-address-book text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  Folks who are interested leave their contact information so you can reach out to
                  them directly.
                </span>
              </div>
            </template>
          </Card>
        </li>
      </ul>
      <h2 class="text-center my-4">Privacy</h2>
      <ul class="list-none flex flex-col gap-4 px-0">
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-eye-slash text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  Only you can see the information people send you. Neither us nor our hosting
                  provider have access to it.
                </span>
              </div>
            </template>
          </Card>
        </li>
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-ban text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  You don't need any sort of account to use this tool. You don't need to provide a
                  name, email address, or phone number to start a group.
                </span>
              </div>
            </template>
          </Card>
        </li>
        <li>
          <Card>
            <template #content>
              <div class="flex gap-4 items-center">
                <i class="pi pi-user-edit text-muted-color !text-3xl" aria-hidden></i>
                <span>
                  You have control over who you authorize to view the information people send you,
                  and you can revoke that access at any time.
                </span>
              </div>
            </template>
          </Card>
        </li>
      </ul>
    </div>
    <p class="flex justify-center gap-2 text-center">
      <RouterLink to="/contact">Contact</RouterLink>
      <span>•</span>
      <RouterLink to="/security">Security</RouterLink>
      <span>•</span>
      <a href="https://github.com/justlark/notwithout.help" target="_blank">GitHub</a>
    </p>
  </main>
</template>

<style scoped></style>
