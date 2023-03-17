<template>
  <div class="main">
    <div v-if="!session">
      <p>Click on "login" or "Sign Up" below to sign in.</p>
      <li><a :href="loginFlowEndpoint" data-testid="sign-in">Login</a></li>
      <li><a :href="registerFlowEndpoint" data-testid="sign-up">Sign Up</a></li>
      <li><a :href="recoverFlowEndpoint" data-testid="recover">Recover account</a></li>
      <li><a :href="verificationFlowEndpoint" data-testid="verify">Verify email</a></li>

      <!-- The settings button will REDIRECT to the login one if there is no login! -->
      <li><a :href="settingsFlowEndpoint" data-testid="sign-up">Adjust settings</a></li>
    </div>

    <h3 v-if="session">You are logged in!</h3>
    <div v-if="session" class="long">
      <p>
        Use the Ory SDK's <code>toSession()</code> call to receive the session information
        in-browser. Here is a snippet, such as the authentication methods used:
      </p>
      <pre><code data-testid='ory-response'>{{ session }}</code></pre>
      <p>
        This is a good way to check if they are logged in with a valid cookie, as toSession will
        fail if they are not.
      </p>
    </div>

    <h3 v-if="!apiResponse">API Response</h3>
    <div v-if="!apiResponse" class="long">
      <p>
        Has not yet authenticated with Minos. (API response failed. If you are logged in, ensure
        Minos is running, and that your account is verified)
      </p>
    </div>

    <h3 v-if="apiResponse">API Response</h3>
    <div v-if="apiResponse" class="long">
      <p>
        You can make authenticated calls to Minos API by sending the cookie to the test endpoint
        when the server is running <code>/demo</code>. <br />
        If you are seeing this you have successfully connected to Minos, and attempted authorization
        with the result shown below. <br />This will return 401 if it fails, if it succeeds it will
        return 200 with the following session object attached.
      </p>
      <pre><code data-testid='api-response'>{{ apiResponse }}</code></pre>
    </div>

    <h3 v-if="session">Common Actions</h3>
    <ul v-if="session">
      <li><a :href="logoutUrl" data-testid="logout">Logout</a></li>
      <li><a :href="verificationFlowEndpoint" data-testid="verify">Verify email</a></li>

      <li> <a :href="settingsFlowEndpoint" data-testid="settings">Settings</a></li>
    </ul>
  </div>
</template>

<script setup>
const config = useRuntimeConfig()
const { $oryConfig } = useNuxtApp()

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
$oryConfig.toSession().then(({ data }) => {
  session.value = data

  // If the user is logged in, we want to show a logout link!
  $oryConfig.createBrowserLogoutFlow().then(({ data }) => {
    logoutUrl.value = data.logout_url
  })
})

// Make an authenticated request to Minos API
// /demo is a test endpoint that returns 200 if successful authentication, 401 if failure to authenticate.
// (This does the same authentication check as the ory.toSession() above, but remotely in the Minos endpoint on the Rust side)
// response body is that session object
fetch(config.minosUrl + '/demo', {
  // "/"
  // Do not forget to set this - it is required to send the session cookie!
  credentials: 'include',
})
  .then((res) => {
    res.json().then((res) => {
      apiResponse.value = res
    })
  })
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.main {
  max-width: 400px;
  margin: 0 auto;
}
</style>
