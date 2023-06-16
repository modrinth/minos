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
import { getOryCookies } from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const app = useNuxtApp()

const loginFlowEndpoint = ref(config.public.oryUrl + '/self-service/login/browser')
const registerFlowEndpoint = ref(config.public.oryUrl + '/self-service/registration/browser')
const recoverFlowEndpoint = ref(config.public.oryUrl + '/self-service/recovery/browser')
const settingsFlowEndpoint = ref(config.public.oryUrl + '/self-service/settings/browser')
const verificationFlowEndpoint = ref(config.public.oryUrl + '/self-service/verification/browser')

const session = ref(null)
const logoutUrl = ref(null)

// Fetch the session directly from Ory
// Authentication is successful if cookie represents a valid Ory Session
try {
  if (!process.server) {
    session.value = await app.$oryConfig.toSession({ cookie: getOryCookies() })
    console.error('very outta here')

    const { data: logOutData } = await app.$oryConfig.createBrowserLogoutFlow({
      cookie: getOryCookies(),
    })
    console.error('very outta here')

    logoutUrl.value = logOutData.logout_url
    console.error('hi')

  }
} catch (e) {
  if (e.response && (e.response.status === 404 || e.response.status === 403)) {
    // 403 likely means another level of auth is needed- either way, reauthenticate with a new flow
    navigateTo(config.public.oryUrl + '/self-service/login/browser?aal=aal2', { external: true })
  }
}
console.error('done')
</script>
