<template>
  <div id="settings">
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
import { extractNestedCsrfToken, extractNestedErrorMessagesFromError } from '~/helpers/ory-ui-extract'

const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([{ text: 'Attempt to change your password!' }])
const password = ref('')

// Uses settings flow to update a logged-in user's password
async function updatePassword() {
  // Get settings flow object from flow id parameter
  const flowData = await $oryConfig.getSettingsFlow({ id: route.query.flow })

  // Directly extract csrf_token from nested returned Ory UI elements
  const csrfToken = extractNestedCsrfToken(flowData.data)

  // updateSettingsFlow can match one of:
  // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
  // For different ways to things to change - some lookup value, or password.
  // In this case, we use UpdateSettingsFlowWithPasswordMethod to update password
  await $oryConfig
    .updateSettingsFlow({
      flow: route.query.flow,
      updateSettingsFlowBody: {
        csrf_token: csrfToken, // must be directly set
        method: 'password',
        password: password.value,
      },
    })
    .then((_r) => {
      console.log('Successful pass change.')
      oryUiMsgs.value = [{ text: 'Successful pass change.' }]
    })
    .catch((e) => {
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}
</script>
