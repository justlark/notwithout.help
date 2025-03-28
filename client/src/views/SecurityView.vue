<script setup lang="ts"></script>

<template>
  <main aria-labelledby="main-heading" class="flex flex-col items-center">
    <h1 id="main-heading" class="text-center mb-8">Security</h1>
    <div class="text-justify max-w-3xl">
      <p>
        <strong>This system has not been audited for security.</strong> If your safety or freedom
        are at risk, do not depend on this software to protect you.
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
      Additionally, anytime you're depending on on web-based cryptography to protect you, it's worth
      <a href="https://www.devever.net/~hl/webcrypto" target="_blank">understanding the caveats</a>.
      <p></p>
      <p>
        You can report security vulnerabilities
        <a href="https://github.com/justlark/notwithout.help/security" target="_blank">on GitHub</a>
        or by <RouterLink to="/contact">contacting us directly</RouterLink>.
      </p>
    </div>
  </main>
</template>

<style scoped></style>
