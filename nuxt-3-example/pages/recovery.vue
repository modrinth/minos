<template>
  <div id="recovery">
    <form @submit.prevent="recovery">
      <input v-model="email" placeholder="email" />
      <input type="submit" value="send email recovery link" />
    </form>
    <form @submit.prevent="submitCode">
      <input v-model="code" placeholder="code" />
      <input type="submit" value="recover using code" />
    </form>

    <li v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
      {{ oryUiMsg.text }}
    </li>

    <NuxtLink to="/">Home page</NuxtLink>

    
  </div>
</template>

<script setup>
import { extractNestedCsrfToken, extractNestedErrorMessagesFromError } from '~/helpers/ory-ui-extract'
const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([{ text: 'Attempt to recover your account!' }])
const email = ref('')
const code = ref('')

// Send recovery email to the set 'email'
async function recovery() {
  // Get recovery flow object from flow id parameter
  const flowData = await $oryConfig.getRecoveryFlow({ id: route.query.flow })

  // Directly extract csrf_token from nested returned Ory UI elements
  const csrfToken = extractNestedCsrfToken(flowData.data)

  // updateRecoveryFlow, which will send an code+link to the provided email
  await $oryConfig
    .updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: csrfToken, // must be directly set
        email: email.value, // MUST be an email identifier, not just a usernmae
        method: 'code',
      },
    })
    .then((_r) => {
      console.log('Successful recovery - part 1')
      oryUiMsgs.value = [{ text: 'Sent recovery email.' }]
    })
    .catch((e) => {
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}

// Attempts to recover an account with the given 'email' and 'code' (sent to an email with the recovery() function)
async function submitCode() {
  // Get recovery flow object from flow id parameter
  const flowData = await $oryConfig.getRecoveryFlow({ id: route.query.flow })

  // Directly extract csrf_token from nested returned Ory UI elements
  const csrfToken = extractNestedCsrfToken(flowData.data)

  // updateRecoveryFlow, but pass the 'code' field to attempt to recover using that code
  await $oryConfig
    .updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: csrfToken, // must be directly set
        // email: email.value, // MUST be an email identifier, not just a usernmae
        method: 'code',
        code: code.value,
      },
    })
    .then((_r) => {
      console.log('Successful recovery!')
      // If return_to exists, return to it, otherwise return to main page
      const returnUrl = flowData.data.return_to || nuxtUrl
      window.location.href = returnUrl
    })
    .catch((e) => {
      // May return a 422: Unprocessable Entity error with a redirection link.
      // We use this to continue the flow.
      // (TODO: this is weird, is this a bug?)
      if (e.response.status === 422) {
        window.location.href = e.response.data.redirect_browser_to
        return
      }

      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}
</script>
