<template>
  <div class="page-container">
    <div class="json-container">
      <ul class="json-list">
        <li v-for="(value, key) in oryUiError" :key="key">
          <b>{{ key }}</b
          >: <span class="json-value" v-html="value"></span>
        </li>
      </ul>
    </div>

    <br />
    <NuxtLink class="text-link" to="/">Home page</NuxtLink>
  </div>
</template>

<script setup>
import { getOryCookies } from '~/helpers/ory-ui-extract'

const oryUiError = ref({ code: 'Loading error...' })
const { $oryConfig } = useNuxtApp()
const route = useRoute()

const formattedValue = (value) => {
  if (typeof value === 'string') {
    return value.replace(/\\n/g, '<br />').replace(/\\t/g, '&nbsp;&nbsp;&nbsp;&nbsp;')
  } else {
    return value
  }
}

try {
  if (!process.server) {
    const r = await $oryConfig.getFlowError({ id: route.query.id, cookie: getOryCookies() })
    oryUiError.value = formattedValue(r.data.error)
  }
} catch (e) {
  oryUiError.value = JSON.stringify(e)
}
</script>

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
