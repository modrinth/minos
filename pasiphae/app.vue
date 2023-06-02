<script setup>
import { TextLogo } from 'omorphia'
</script>
<template>
  <div class="app-wrapper">
    <div class="app">
      <div class="page-container">
        <TextLogo class="modrinth-logo" />
        <RouterView />
      </div>
    </div>
  </div>
</template>
<style src="omorphia/dist/style.css"></style>
<style src="./assets/styles.css"></style>
<style>
.modrinth-logo {
  height: 1.5rem;
  width: auto;
  color: var(--color-contrast);
  margin-right: auto;
}
</style>
