<template>
  <div id="error">
    {{ oryUiError }}
  </div>
</template>

<script setup>
const oryUiError = ref('Loading error...')
const { $oryConfig } = useNuxtApp()
const route = useRoute()

$oryConfig.getFlowError({ id: route.query.id }).then((r) => {
  oryUiError.value = r.data.error // Just for demo here, display passed error json in display variable
})
</script>
