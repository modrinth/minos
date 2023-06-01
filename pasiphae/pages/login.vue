<template>
  <template v-if="flowData">
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>

    <template v-if="flowData.requested_aal == 'aal2'">
      <h1>Two-factor authentication</h1>
      <p>Please enter the code shown in your authentication app.</p>
      <label for="mfa" hidden>Authentication code</label>
      <input v-model="mfa" id="mfa" type="text" placeholder="Authentication code" />
      <button class="btn btn-primary continue-btn" @click="loginTotp()">
        Continue <RightArrowIcon />
      </button>

      <p><a class="text-link" :href="logoutUrl" data-testid="logout">Logout</a></p>

      <div class="text-divider">
        <div></div>
        <span>or</span>
        <div></div>
      </div>
      <p>Enter one of your stored lookup secrets.</p>
      <label for="lookupSecret" hidden>Authentication backup code</label>
      <input v-model="lookupSecret" id="lookupSecret" type="text" placeholder="Lookup secret" />
      <button class="btn btn-primary continue-btn" @click="loginLookupSecret()">
        Continue <RightArrowIcon />
      </button>
    </template>
    <template v-if="flowData.requested_aal == 'aal1'">
      <h1>Continue with</h1>
      <div class="third-party">
        <Button
          v-for="provider in providers"
          :key="provider"
          :class="`${provider}-btn`"
          @click="loginOidc(provider)"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
      <div class="text-divider">
        <div></div>
        <span>or</span>
        <div></div>
      </div>
      <label for="email" hidden>Email</label>
      <input v-model="email" id="email" type="text" placeholder="Email" />
      <label for="password" hidden>Password</label>
      <input v-model="password" id="password" type="password" placeholder="Password" />
      <div class="account-options">
        <a class="text-link" :href="recoverFlowEndpoint">Forgot password?</a>
      </div>
      <button class="btn btn-primary continue-btn" @click="loginPassword()">
        Continue <RightArrowIcon />
      </button>
      <p>
        Don't have an account yet?
        <a class="text-link" :href="registerFlowEndpoint">Create one.</a>
      </p>
    </template>
  </template>
</template>

<script setup>
import { Button, GitHubIcon, RightArrowIcon } from 'omorphia'
import DiscordIcon from '@/assets/discord.svg'
import GoogleIcon from '@/assets/google.svg'
import AppleIcon from '@/assets/apple.svg'
import MicrosoftIcon from '@/assets/microsoft.svg'
import GitLabIcon from '@/assets/gitlab.svg'
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractOidcProviders,
  extractNestedErrorMessagesFromUiData,
} from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const route = useRoute()
const { $oryConfig } = useNuxtApp()

const recoverFlowEndpoint = ref(config.oryUrl + '/self-service/recovery/browser')
const registerFlowEndpoint = ref(config.oryUrl + '/self-service/registration/browser')
const logoutUrlEndpoint = ref(null)

const oryUiMsgs = ref([])
const email = ref('')
const password = ref('')
const mfa = ref('')
const lookupSecret = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const providers = ref([])

async function updateFlow() {
  try {
    const r = await $oryConfig
    .getLoginFlow({ id: route.query.flow || '' });
    flowData.value = r.data
      providers.value = extractOidcProviders(r.data)
      oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)

      // Show a logout link (in particular)
      if (r.data.requested_aal == 'aal2') {
        const data = await app.$oryConfig.createBrowserLogoutFlow()
          logoutUrl.value = data.logout_url
      }

  } catch(e) {
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        navigateTo( e.response.data.redirect_browser_to, { external: true })
      } else if ((e.response && e.response.status === 404) || e.response.status === 403) {
        // 403 likely means another level of auth is needed- either way, reauthenticate with a new flow
        navigateTo(config.oryUrl + '/self-service/login/browser', { external: true })
      } else {
        oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
      }
  }
}
await updateFlow()

const icons = {
  discord: DiscordIcon,
  google: GoogleIcon,
  apple: AppleIcon,
  microsoft: MicrosoftIcon,
  gitlab: GitLabIcon,
  github: GitHubIcon,
}

function capitalizeFirstLetter(string) {
  return string.charAt(0).toUpperCase() + string.slice(1)
}

function getIcon(provider) {
  return icons[provider]
}

async function loginPassword() {
  // loginFlowBody is an instance of UpdateLoginFlowWithPasswordMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    identifier: email.value,
    method: 'password',
    password: password.value,
  }
  await sendUpdate(loginFlowBody)
}

async function loginOidc(provider) {
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: provider,
  }
  await sendUpdate(loginFlowBody)
}

async function loginTotp() {
  const loginFlowBody = {
    // csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'totp',
    totp_code: mfa.value,
  }
  await sendUpdate(loginFlowBody)
}

async function loginLookupSecret() {
  const loginFlowBody = {
    // csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'lookup_secret',
    lookup_secret: lookupSecret.value,
  }
  await sendUpdate(loginFlowBody)
}

// loginFlowBody must match a variant of UpdateLoginFlowWith<method>Method (included are UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod)
async function sendUpdate(loginFlowBody) {
  let csrf_token = extractNestedCsrfToken(flowData.value) // must be directly set
  loginFlowBody.csrf_token = csrf_token
  try {
  await $oryConfig
    .updateLoginFlow({
      flow: route.query.flow,
      updateLoginFlowBody: loginFlowBody,
    });

    await updateFlow()
      // If return_to exists, return to it, otherwise refresh data
      const returnUrl = flowData.value.return_to || config.nuxtUrl
      navigateTo(returnUrl, { external: true })
  } catch(e) {

      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        navigateTo(e.response.data.redirect_browser_to, { external: true })

      } else {
        // Get displayable error messsages from nested returned Ory UI elements
        oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
      }
    }
}
</script>

<style src="~/assets/login.css"></style>
<style scoped>
.account-options {
  display: flex;
  width: 100%;
}

.account-options a {
  margin-left: auto;
}
</style>
