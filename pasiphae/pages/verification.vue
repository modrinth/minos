<template v-if="flowData">
  <div v-if="flowData.state === 'sent_email'" class="page-container">
    <h1>Verify your email</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <p>Enter the code sent to your email to verify it.</p>
    <input v-model="code" placeholder="Enter code" type="text" />
    <button class="btn btn-primary continue-btn" @click="verify">Verify Email</button>
    <p><a class="text-link" :href="recoverFlowEndpoint" data-testid="sign-in">Resend email</a></p>
  </div>
  <div v-else-if="flowData.state === 'choose_method'" class="page-container">
    <h1>Verify your email</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <p>Enter the email you want to verify.</p>
    <input v-model="code" placeholder="Enter email" type="text" />
    <button class="btn btn-primary continue-btn" @click="verify">Send code</button>
  </div>
  <div v-else-if="flowData.state === 'passed_challenge'" class="page-container">
    <h1>Successfully verified email.</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <p><a class="text-link" :href="homeEndpoint" data-testid="sign-in">Return to home page</a></p>
  </div>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromUiData,
  extractNestedErrorMessagesFromError,
} from '~/helpers/ory-ui-extract'

const { $oryConfig } = useNuxtApp()
const route = useRoute()

const config = useRuntimeConfig()
const homeEndpoint = ref(config.public.nuxtUrl)
const recoverFlowEndpoint = ref(config.public.oryUrl + '/self-service/verification/browser')

const oryUiMsgs = ref([])
const code = ref(route.query.code ?? '')

// Attempt to get flow information on page load
const flowData = ref(null)
async function updateFlow() {
  try {
    console.error('got v flow')

    const r = await $oryConfig.getVerificationFlow({
      id: route.query.flow || '',
    })
    console.error('hello111')
    console.error(r)

    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromUiData(flowData.value)

    // // If they clicked on the email link and the flow is still the same, the flow.data.ui object
    // // will contain 'code' amongst its UI nodes with the verification code- which ideally can be put automatically
    // // into the field so they can just verify it and continue.
    if (flowData.value && flowData.value.state === 'sent_email') {
      console.error('hello2')
      const returnedNodes = r.data.ui.nodes
      for (let i = 0; i < returnedNodes.length; i++) {
        if (returnedNodes[i].group === 'code' && returnedNodes[i].attributes.name === 'code') {
          code.value = returnedNodes[i].attributes.value
          break
        }
      }
    }
  } catch (e) {
    // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
    // Any other error we just leave the page
    if (e && 'response' in e) {
      if ('data' in e.response && 'redirect_browser_to' in e.response.data) {
        navigateTo(e.response.data.redirect_browser_to, { external: true })
      } else if (e.response.status === 404) {
        navigateTo(config.public.oryUrl + '/self-service/settings/browser', { external: true })
      } else {
        navigateTo('/')
      }
    } else {
      navigateTo('/')
    }
  }
}
if (!process.server) {
  await updateFlow()
}

// Attempts to verify an account with the given 'code' (sent to an email with the registration flow)
async function verify() {
  console.error('verify')

  let body
  if (flowData.value.state === 'sent_email') {
    body = {
      flow: route.query.flow,
      csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
      method: 'code',
      code: code.value,
    }
  } else if (flowData.value.state === 'choose_method') {
    body = {
      flow: route.query.flow,

      csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
      method: 'code',
      email: code.value,
    }
  }

  try {
    const r = await $oryConfig.updateVerificationFlow({
      flow: route.query.flow,
      updateVerificationFlowBody: body,
    })
    oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)
    // Success!
    await updateFlow()
  } catch (e) {
    // updateVerificationFlow, submitting and checking with verification code
    // Get displayable error messsages from nested returned Ory UI elements
    oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
  }
}
</script>
