<template>
  <div v-if="flowData" class="page-container">
    <h1>Reset your password</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <template v-if="flowData.state == 'choose_method'">
      <p>
        Enter your email below and we'll send a recovery link to allow you to recover your account.
      </p>
      <label for="email" hidden>Email</label>
      <input id="email" v-model="email" type="text" placeholder="Email" />
      <button class="btn btn-primary continue-btn" @click="recovery">Send recovery email</button>
    </template>
    <template v-else-if="flowData.state == 'sent_email'">
      <p>
        A recovery email has been sent to <strong>{{ email }}</strong
        >.
      </p>
      <p>Check your email and enter the code from it below.</p>
      <input id="code" v-model="code" type="text" placeholder="Enter code" />
      <button class="btn btn-primary continue-btn" @click="submitCode">Recover</button>
    </template>
    <template v-else-if="flowData.state == 'passed_challenge'">
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
  </div>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromUiData,
} from '~/helpers/ory-ui-extract'
const { $oryConfig } = useNuxtApp()
const route = useRoute()

const oryUiMsgs = ref([])
const email = ref('')
const code = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
async function updateFlow() {
  try {
    const r = await $oryConfig.getRecoveryFlow({ id: route.query.flow })

    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)
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

// Send recovery email to the set 'email'
async function recovery() {
  try {
    // updateRecoveryFlow, which will send an code+link to the provided email
    await $oryConfig.updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        email: email.value, // MUST be an email identifier, not just a usernmae
        method: 'code',
      },
    })
    oryUiMsgs.value = []
    await updateFlow()
  } catch (e) {
    if (e && 'response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
      navigateTo(e.response.data.redirect_browser_to, { external: true })
    } else {
      // Get displayable error messsages from nested returned Ory UI elements
      oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
    }
  }
}

// Attempts to recover an account with the given 'email' and 'code' (sent to an email with the recovery() function)
async function submitCode() {
  // updateRecoveryFlow, but pass the 'code' field to attempt to recover using that code
  try {
    await $oryConfig.updateRecoveryFlow({
      flow: route.query.flow,
      updateRecoveryFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        method: 'code',
        code: code.value,
      },
    })

    await updateFlow()
  } catch (e) {
    // Get displayable error messsages from nested returned Ory UI elements
    oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
  }
}
</script>
