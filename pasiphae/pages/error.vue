<template>
  <div class="json-container">
    <ul class="json-list">
      <li v-for="(value, key) in oryUiError">
        <b>{{ key }}</b
        >: <span class="json-value" v-html="formattedValue(value)"></span>
      </li>
    </ul>
  </div>

  <br />
  <NuxtLink class="text-link" to="/">Home page</NuxtLink>
</template>

<style>
.json-container {
  white-space: pre-line;
}
.json-list {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
.json-list li::before {
  content: '\A';
}
</style>

<script setup>
const oryUiError = ref('Loading error...')
const { $oryConfig } = useNuxtApp()
const route = useRoute()

$oryConfig.getFlowError({ id: route.query.id }).then((r) => {
  oryUiError.value = r.data.error // Just for demo here, display passed error json in display variable
})

const formattedValue = (value) => {
  if (typeof value === 'string') {
    return value.replace(/\\n/g, '<br />').replace(/\\t/g, '&nbsp;&nbsp;&nbsp;&nbsp;')
  } else {
    return value
  }
}
</script>
