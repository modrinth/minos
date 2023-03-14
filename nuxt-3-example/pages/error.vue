<template>
  <div id="error">
      {{ ory_ui_error }}
  </div>
</template>

<script setup>

let ory_ui_error = ref("Loading error...");
let { $oryConfig } = useNuxtApp();
let route = useRoute();


$oryConfig.getFlowError({id:route.query.id}).then(r => {
    ory_ui_error.value = r.data.error  // Just for demo here, display passed error json in display variable
});

</script>
