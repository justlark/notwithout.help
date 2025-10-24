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
      <p>
        Not Without Help is a tool for connecting activist groups with people who want to help. It's
        an inbox for organizers to collect contact information from folks who are interested, and
        where volunteers can share information about what kinds of help they can offer.
      </p>
      <p>
        This tool was inspired by Devon Price's concept of
        <a
          href="https://drdevonprice.substack.com/p/burning-it-all-down-without-burning?open=false#%C2%A7figure-out-your-activist-character-class"
          target="_blank"
          >activist "character classes,"</a
        >
        with the goal of broadening both organizers' and volunteers' understanding of what kinds of
        support are valuable in activism and including people who might not otherwise think they
        have a role.
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
                  You have control over who can access the information people send you, and you can
                  revoke that access at any time.
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
      <RouterLink to="/tips">Tip Jar</RouterLink>
      <span>•</span>
      <a href="https://github.com/justlark/notwithout.help" target="_blank">GitHub</a>
    </p>
  </main>
</template>

<style scoped></style>
