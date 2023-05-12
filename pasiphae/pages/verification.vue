<template>
  <template v-if="flowData">
    <h1>Verify your email</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <p>Enter the code sent to your email to verify it.</p>
    <input v-model="code" placeholder="Enter code" type="text" />
    <button @click="verification" class="btn btn-primary continue-btn">Verify Email</button>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromData,
} from '~/helpers/ory-ui-extract'

const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([])
const code = ref(route.query.code ?? '')

// Attempt to get flow information on page load
const flowData = ref(null)
$oryConfig
  .getVerificationFlow({ id: route.query.flow || '' })
  .then((r) => {
    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)

    // // If they clicked on the email link and the flow is still the same, the flow.data.ui object
    // // will contain 'code' amongst its UI nodes with the verification code- which ideally can be put automatically
    // // into the field so they can just verify it and continue.
    // const returnedNodes = r.data.ui.nodes
    //   for (let i = 0; i < returnedNodes.length; i++) {
    //     if (returnedNodes[i].group === 'code' && returnedNodes[i].attributes.name === 'code') {
    //       code.value = returnedNodes[i].attributes.value
    //       break
    //     }
    //   }
  })
  // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
  // Any other error we just leave the page
  .catch((e) => {
    if (e.response.status === 404) {
      navigateTo(config.oryUrl + '/self-service/settings/browser', { external: true })
    } else {
      navigateTo('')
    }
  })

// Attempts to verify an account with the given 'code' (sent to an email with the registration flow)
async function verification() {
  // updateVerificationFlow, submitting and checking with verification code
  await $oryConfig
    .updateVerificationFlow({
      flow: route.query.flow,
      updateVerificationFlowBody: {
        csrfToken: extractNestedCsrfToken(flowData.value), // must be directly set
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
