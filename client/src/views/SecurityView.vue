<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>

<template>
  <main aria-labelledby="main-heading" class="flex flex-col items-center">
    <h1 id="main-heading" class="text-center mb-8">Security</h1>
    <div class="text-justify max-w-3xl">
      <p>
        This app is designed to mitigate risks to your privacy and security, but it has not been
        audited for security, so it cannot make strong guarantees.
        <strong
          >If your safety or freedom are at risk, do not depend on this software to protect
          you</strong
        >.
      </p>
      <p>
        If you are technically inclined, you can review the
        <a
          href="https://github.com/justlark/notwithout.help/blob/main/docs/security-whitepaper.md"
          target="_blank"
          >security whitepaper</a
        >
        detailing how the system is designed.
      </p>
      <p>
        You can report security vulnerabilities
        <a href="https://github.com/justlark/notwithout.help/security" target="_blank">on GitHub</a>
        or by <RouterLink to="/contact">contacting us directly</RouterLink>.
      </p>
    </div>
  </main>
</template>

<style scoped></style>
