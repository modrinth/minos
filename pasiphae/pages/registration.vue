<template>
  <div class="page-container">
    <h1>Create your account</h1>
    <div class="third-party">
      <Button
        v-for="provider in providers"
        :key="provider"
        :class="`${provider}-btn`"
        @click="register(provider)"
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
    <input id="email" v-model="email" type="text" placeholder="Email or username" />
    <label for="username" hidden>Username</label>
    <input id="username" v-model="username" type="text" placeholder="Username" />
    <label for="password" hidden>Password</label>
    <input id="password" v-model="password" type="password" placeholder="Password" />
    <label for="confirm-password" hidden>Password</label>
    <input
      id="confirm-password"
      v-model="confirmPassword"
      type="password"
      placeholder="Confirm password"
    />
    <button class="btn btn-primary continue-btn" @click="registerPassword">
      Create account <RightArrowIcon />
    </button>
    <p>Already have an account yet? <a class="text-link" :href="loginFlowEndpoint">Sign in.</a></p>
  </div>
</template>

<script setup>
import { Button, RightArrowIcon, GitHubIcon } from 'omorphia'
import DiscordIcon from '@/assets/discord.svg'
import GoogleIcon from '@/assets/google.svg'
import AppleIcon from '@/assets/apple.svg'
import MicrosoftIcon from '@/assets/microsoft.svg'
import GitLabIcon from '@/assets/gitlab.svg'
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromUiData,
  extractOidcProviders,
  getOryCookies,
} from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const route = useRoute()
const { $oryConfig } = useNuxtApp()

const loginFlowEndpoint = ref(config.public.oryUrl + '/self-service/login/browser')

const oryUiMsgs = ref([])
const email = ref('')
const username = ref('')
const password = ref('')
const confirmPassword = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const providers = ref([])

try {
  if (!process.server) {

    const r = await $oryConfig.getRegistrationFlow({
      id: route.query.flow || '',
      cookie: getOryCookies(),
    })
    flowData.value = r.data
    providers.value = extractOidcProviders(r.data)
    oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)
  }
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

async function registerPassword() {
  if (password.value !== confirmPassword.value) {
    oryUiMsgs.value = [{ text: 'Passwords do not match!' }]
    return
  }

  // There are several preset ways to identify people
  // https://www.ory.sh/docs/kratos/manage-identities/identity-schema
  // -email and password (seems ideal, and the one I have currently setup)
  // - user and password (they cannot reset their email)
  // - newsletter opt in
  // These are differentiated by the traits obj
  // These are *separate* to social logins which get mapped to these presets.
  const registrationFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'password',
    password: password.value,
    'traits.email': email.value,
    'traits.username': username.value,
  }
  await registerGeneric(registrationFlowBody)
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

async function register(provider) {
  await registerOidc(provider)
}

function getIcon(provider) {
  return icons[provider]
}

async function registerOidc(provider) {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider,
  }
  await registerGeneric(registrationFlowBody)
}

// loginFlowBody must match a variant of UpdateLoginFlowWith<method>Method (included are UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod)
async function registerGeneric(registrationFlowBody) {
  try {
    await $oryConfig.updateRegistrationFlow({
      flow: route.query.flow,
      updateRegistrationFlowBody: registrationFlowBody,
    })
    const returnUrl = flowData.value.return_to || config.public.nuxtUrl
    navigateTo(returnUrl, { external: true })
  } catch (e) {
    // Using Social-integrated login/registration will return a 422: Unprocessable Entity error with a redirection link.
    // We use this to continue the flow.
    // (TODO: this is weird, is this a bug?)
    if (e && 'response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
      navigateTo(e.response.data.redirect_browser_to, { external: true })
      return
    }

    // Get displayable error messsages from nested returned Ory UI elements
    oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
  }
}
</script>
<style src="~/assets/login.css"></style>
