<template>
  <div v-if="flowData" id="verification" >
    Verify your email.
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
import { extractNestedCsrfToken, extractNestedErrorMessagesFromData } from '~/helpers/ory-ui-extract'

const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([])
const code = ref('')

// Attempt to get flow information on page load
const flowData = ref(null);
$oryConfig.getVerificationFlow({ id: route.query.flow || "" })
.then( r =>  {
  flowData.value = r.data;
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
.catch( e => {
  if (e.response.status === 404)  {
    window.location.href = config.oryUrl + '/self-service/settings/browser'
  } else {
    window.location.href = config.nuxtUrl;
  }
});


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
