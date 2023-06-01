<template>
  <div>
    <div v-if="!session">
      <p>Click on "login" or "Sign Up" below to sign in.</p>
      <p><a class="text-link" :href="loginFlowEndpoint" data-testid="sign-in">Login</a></p>
      <p><a class="text-link" :href="registerFlowEndpoint" data-testid="sign-up">Sign Up</a></p>
      <p>
        <a class="text-link" :href="recoverFlowEndpoint" data-testid="recover">Recover account</a>
      </p>
      <p>
        <a class="text-link" :href="verificationFlowEndpoint" data-testid="verify">Verify email</a>
      </p>

      <!-- The settings button will REDIRECT to the login one if there is no login! -->
      <p>
        <a class="text-link" :href="settingsFlowEndpoint" data-testid="sign-up">Adjust settings</a>
      </p>
    </div>

    <h3 v-if="session">You are logged in!</h3>
    <div v-if="session">
      <p><a class="text-link" :href="logoutUrl" data-testid="logout">Logout</a></p>
      <p>
        <a class="text-link" :href="verificationFlowEndpoint" data-testid="verify">Verify email</a>
      </p>

      <p><a class="text-link" :href="settingsFlowEndpoint" data-testid="settings">Settings</a></p>
    </div>
  </div>
</template>
<script setup>
const config = useRuntimeConfig()
const app = useNuxtApp()

const loginFlowEndpoint = ref(config.oryUrl + '/self-service/login/browser')
const registerFlowEndpoint = ref(config.oryUrl + '/self-service/registration/browser')
const recoverFlowEndpoint = ref(config.oryUrl + '/self-service/recovery/browser')
const settingsFlowEndpoint = ref(config.oryUrl + '/self-service/settings/browser')
const verificationFlowEndpoint = ref(config.oryUrl + '/self-service/verification/browser')

const session = ref(null)
const logoutUrl = ref(null)
const apiResponse = ref(null)

// Fetch the session directly from Ory
// Authentication is successful if cookie represents a valid Ory Session
try {
  const { data } = await app.$oryConfig.toSession()
  session.value = data

  const { data: newData }  = await app.$oryConfig.createBrowserLogoutFlow()
  logoutUrl.value = newData.logout_url
} catch (err) {
  console.error(err)
}

try {
  const { res }  = await fetch(config.minosUrl + '/user/session', {
    credentials: 'include',
  })

  const json = await res.json()
  apiResponse.value = json
} catch (err) {
  console.error(err)
}

// Make an authenticated request to Minos API
// /demo is a test endpoint that returns 200 if successful authentication, 401 if failure to authenticate.
// (This does the same authentication check as the ory.toSession() above, but remotely in the Minos endpoint on the Rust side)
// response body is that session object

</script>
