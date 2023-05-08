<script setup>
import { TextLogo } from 'omorphia'
</script>
<style src="omorphia/dist/style.css"></style>
<style src="./assets/styles.css"></style>
<style>
main {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--gap-md);
}

.modrinth-logo {
  height: 1.5rem;
  width: auto;
  color: var(--color-contrast);
}
</style>
<template>
  <div class="app-wrapper">
    <div class="app">
      <main>
        <TextLogo class="modrinth-logo" />
        <RouterView />
      </main>
    </div>
  </div>
</template>
