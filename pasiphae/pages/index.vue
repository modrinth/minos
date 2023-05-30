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
app.$oryConfig
  .toSession()
  .then(({ data }) => {
    session.value = data
    // If the user is logged in, we want to show a logout link!
    app.$oryConfig.createBrowserLogoutFlow().then(({ data }) => {
      logoutUrl.value = data.logout_url
    })
  })
  .catch((e) => {
    if ((e.response && e.response.status === 404) || e.response.status === 403) {
      // 403 likely means another level of auth is needed- either way, reauthenticate with a new flow
      navigateTo(config.oryUrl + '/self-service/login/browser?aal=aal2', { external: true })
    }
  })

// Make an authenticated request to Minos API
// /demo is a test endpoint that returns 200 if successful authentication, 401 if failure to authenticate.
// (This does the same authentication check as the ory.toSession() above, but remotely in the Minos endpoint on the Rust side)
// response body is that session object
fetch(config.minosUrl + '/user/session', {
  // "/"
  // Do not forget to set this - it is required to send the session cookie!
  credentials: 'include',
}).then((res) => {
  res.json().then((res) => {
    apiResponse.value = res
  })
})
</script>
