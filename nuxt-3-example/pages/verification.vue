<template>
  <div id="verification">
    <form @submit.prevent="verification">
      <input v-model="code" placeholder="code" />
      <input type="submit" value="verify with code" />
    </form>

    <li v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
      {{ oryUiMsg.text }}
    </li>

    <NuxtLink to="/">Home page</NuxtLink>

  </div>
</template>

<script setup>
import { extractNestedCsrfToken } from '~/helpers/ory-ui-extract'

const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([{ text: 'Please verification code!' }])
const code = ref('')

// On load, do a quick check for status of the verification flow
// If they clicked on the email link and the flow is still the same, the flow.data.ui object
// will contain 'code' amongst its UI nodes with the verification code- which ideally can be put automatically
// into the field so they can just verify it and continue.
$oryConfig.getVerificationFlow({ id: route.query.flow }).then((flow) => {
  const returnedNodes = flow.data.ui.nodes
  for (let i = 0; i < returnedNodes.length; i++) {
    if (returnedNodes[i].group === 'code' && returnedNodes[i].attributes.name === 'code') {
      code.value = returnedNodes[i].attributes.value
      break
    }
  }
})

// Attempts to verify an account with the given 'code' (sent to an email with the registration flow)
async function verification() {
  // Get recovery flow object from flow id parameter
  const flowData = await $oryConfig.getVerificationFlow({ id: route.query.flow })

  // Directly extract csrf_token from nested returned Ory UI elements
  const csrfToken = extractNestedCsrfToken(flowData.data)

  // updateVerificationFlow, submitting and checking with verification code
  await $oryConfig
    .updateVerificationFlow({
      flow: route.query.flow,
      updateVerificationFlowBody: {
        csrfToken, // must be directly set
        code: code.value,
        method: 'code',
      },
    })
    .then((_r) => {
      oryUiMsgs.value = [{ text: 'Email successfully verified.' }]
      // Success! We can maybe then direct them to the login page?
    })
    .catch((e) => {
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}
</script>
