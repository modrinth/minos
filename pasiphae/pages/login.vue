<template>
  <template v-if="flowData">
    <h1>Continue with</h1>
    <div class="third-party">
      <Button
        v-for="provider in providers"
        :key="provider"
        :class="`${provider}-btn`"
        @click="login(provider)"
      >
        <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
      </Button>
    </div>
    <div class="text-divider">
      <div></div>
      <span>or</span>
      <div></div>
    </div>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
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

const oryUiMsgs = ref([])
const email = ref('')
const password = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const providers = ref([])

try {
  const r = await $oryConfig
      .getLoginFlow({ id: route.query.flow || '' })

  flowData.value = r.data
  providers.value = extractOidcProviders(r.data)
  oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)
} catch (e) {
  if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
    navigateTo(e.response.data.redirect_browser_to, { external: true })
  } else if (e.response && e.response.status === 404) {
    navigateTo(config.oryUrl + '/self-service/login/browser', { external: true })
  } else {
    navigateTo('/')
  }
}

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

async function login(provider) {
  await loginOidc(provider)
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
  await loginGeneric(loginFlowBody)
}

async function loginOidc(provider) {
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: provider,
  }
  await loginGeneric(loginFlowBody)
}

// loginFlowBody must match a variant of UpdateLoginFlowWith<method>Method (included are UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod)
async function loginGeneric(loginFlowBody) {
  // Update login flow using passed method of choice
  await $oryConfig
    .updateLoginFlow({
      flow: route.query.flow,
      updateLoginFlowBody: loginFlowBody,
    })
    .then((_r) => {
      // If return_to exists, return to it, otherwise return to main page
      const returnUrl = flowData.value.return_to || config.nuxtUrl
      window.location.href = returnUrl
    })
    .catch((e) => {
      // Using Social-integrated login/registration will return a 422: Unprocessable Entity error with a redirection link.
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
