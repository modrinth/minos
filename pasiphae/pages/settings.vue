<template>
  <template v-if="flowData">
    <h1>Edit your password</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <input v-model="password" placeholder="Password" type="password" />
    <input v-model="confirmPassword" placeholder="Confirm Password" type="password" />

    <button @click="updatePassword" class="btn btn-primary continue-btn">Reset password</button>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromData,
} from '~/helpers/ory-ui-extract'

const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([])
const password = ref('')
const confirmPassword = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
$oryConfig
  .getSettingsFlow({ id: route.query.flow || '' })
  .then((r) => {
    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
  })
  // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
  // Any other error we just leave the page
  .catch((e) => {
    if (e.response.status === 404) {
      navigateTo(config.oryUrl + '/self-service/settings/browser', { external: true })
    } else {
      navigateTo('/')
    }
  })

// Uses settings flow to update a logged-in user's password
async function updatePassword() {
  if (password.value !== confirmPassword.value) {
    oryUiMsgs.value = [{ text: 'Passwords do not match!' }]
    return
  }

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
