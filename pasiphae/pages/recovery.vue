<template>
  <template v-if="flowData">
    <h1>Reset your password</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <template v-if="mode === 0">
      <p>
        Enter your email below and we'll send a recovery link to allow you to recover your account.
      </p>
      <label for="email" hidden>Email</label>
      <input v-model="email" id="email" type="text" placeholder="Email" />
      <button @click="recovery" class="btn btn-primary continue-btn">Send recovery email</button>
    </template>
    <template v-else-if="mode === 1">
      <p>
        A recovery email has been sent to <strong>{{ email }}</strong
        >.
      </p>
      <p>Check your email and enter the code from it below.</p>
      <input id="code" v-model="code" type="text" placeholder="Enter code" />
      <button @click="submitCode" class="btn btn-primary continue-btn">Recover</button>
    </template>
    <template v-else-if="mode === 2">
      <p>
        You are resetting the password for the Modrinth account associated with
        <strong>{{ email }}</strong
        >.
      </p>
      <label for="password" hidden>Password</label>
      <input id="password" type="text" placeholder="Password" />
      <label for="confirm-password" hidden>Password</label>
      <input id="confirm-password" type="text" placeholder="Confirm password" />
      <button class="btn btn-primary continue-btn">Reset password</button>
    </template>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromData,
} from '~/helpers/ory-ui-extract'
const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([])
const email = ref('')
const code = ref('')
const mode = ref(0)

// Attempt to get flow information on page load
const flowData = ref(null)
$oryConfig
  .getRecoveryFlow({ id: route.query.flow })
  .then((r) => {
    console.log(r.data)
    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
  })
  // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
  // Any other error we just leave the page
  .catch((e) => {
    if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
      window.location.href = e.response.data.redirect_browser_to
    } else if (e.response.status === 404) {
      navigateTo(config.oryUrl + '/self-service/recovery/browser', { external: true })
    } else {
      navigateTo('/')
    }
  })

// Send recovery email to the set 'email'
async function recovery() {
  // updateRecoveryFlow, which will send an code+link to the provided email
  await $oryConfig
    .updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        email: email.value, // MUST be an email identifier, not just a usernmae
        method: 'code',
      },
    })
    .then((_r) => {
      oryUiMsgs.value = []
      mode.value = 1
    })
    .catch((e) => {
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        window.location.href = e.response.data.redirect_browser_to
      } else {
        // Get displayable error messsages from nested returned Ory UI elements
        oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
      }
    })
}

// Attempts to recover an account with the given 'email' and 'code' (sent to an email with the recovery() function)
async function submitCode() {
  // updateRecoveryFlow, but pass the 'code' field to attempt to recover using that code
  await $oryConfig
    .updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        // email: email.value, // MUST be an email identifier, not just a usernmae
        method: 'code',
        code: code.value,
      },
    })
    .then((_r) => {
      oryUiMsgs.value = []
      // If return_to exists, return to it, otherwise return to main page
      const returnUrl = flowData.value.return_to || nuxtUrl
      window.location.href = returnUrl
    })
    .catch((e) => {
      // May return a 422: Unprocessable Entity error with a redirection link.
      // We use this to continue the flow.
      // (TODO: this is weird, is this a bug?)
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        window.location.href = e.response.data.redirect_browser_to
        return
      }
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    })
}
</script>
