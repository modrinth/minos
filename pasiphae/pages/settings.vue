<template>
  <div v-if="flowData" id="settings" >
    Edit your settings (attempt to change your password)
    <form @submit.prevent="updatePassword">
      <input v-model="password" placeholder="password" type="password" />
      <input type="submit" value="change" />
    </form>

    <li v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
      {{ oryUiMsg.text }}
    </li>

    <NuxtLink to="/">Home page</NuxtLink>
  </div>
</template>

<script setup>
import { extractNestedCsrfToken, extractNestedErrorMessagesFromError, extractNestedErrorMessagesFromData } from '~/helpers/ory-ui-extract'

const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([])
const password = ref('')

// Attempt to get flow information on page load
const flowData = ref(null);
$oryConfig.getSettingsFlow({ id: route.query.flow || "" })
.then( r =>  {
  flowData.value = r.data;
  oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)})
// Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
// Any other error we just leave the page
.catch( e => {
  if (e.response.status === 404)  {
    window.location.href = config.oryUrl + '/self-service/settings/browser'
  } else {
    window.location.href = config.nuxtUrl;
  }
});

// Uses settings flow to update a logged-in user's password
async function updatePassword() {
  // updateSettingsFlow can match one of:
  // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
  // For different ways to things to change - some lookup value, or password.
  // In this case, we use UpdateSettingsFlowWithPasswordMethod to update password
  await $oryConfig
    .updateSettingsFlow({
      flow: route.query.flow,
      updateSettingsFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        method: 'password',
        password: password.value,
      },
    })
    .then((_r) => {
      oryUiMsgs.value = [{ text: 'Successful pass change.' }]
    })
    .catch((e) => {
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}
</script>
