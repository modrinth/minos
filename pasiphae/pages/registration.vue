<template>
  <h1>Create your account</h1>
  <div class="third-party">
    <Button class="discord-btn" @click="registerDiscord"><DiscordIcon /> <span>Discord</span></Button>
    <Button class="github-btn" @click="registerGithub"><GitHubIcon /> <span>GitHub</span></Button>
    <Button class="microsoft-btn" @click="registerMicrosoft"><MicrosoftIcon /> <span>Microsoft</span></Button>
    <Button class="google-btn" @click="registerGoogle"><GoogleIcon /> <span>Google</span></Button>
    <Button class="apple-btn" @click=""><AppleIcon /> <span>Apple</span></Button>
    <Button class="gitlab-btn" @click=""><GitLabIcon /> <span>GitLab</span></Button>
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
  <input
    v-model="email"
    id="email"
    type="text"
    placeholder="Email or username"
  />
  <label for="username" hidden>Username</label>
  <input
    v-model="username"
    id="username"
    type="text"
    placeholder="Username"
  />
  <label for="password" hidden>Password</label>
  <input
    v-model="password"
    id="password"
    type="password"
    placeholder="Password"
  />
  <label for="confirm-password" hidden>Password</label>
  <input
    v-model="confirmPassword"
    id="confirm-password"
    type="password"
    placeholder="Confirm password"
  />
  <button @click="registerPassword" class="btn btn-primary continue-btn">Create account <RightArrowIcon /></button>
  <p>Already have an account yet? <a class="text-link" :href="loginFlowEndpoint">Sign in.</a></p>
</template>

<script setup>
import DiscordIcon from '@/assets/discord.svg'
import GoogleIcon from '@/assets/google.svg'
import AppleIcon from '@/assets/apple.svg'
import MicrosoftIcon from '@/assets/microsoft.svg'
import GitLabIcon from '@/assets/gitlab.svg'
import { Button, RightArrowIcon, GitHubIcon } from 'omorphia'
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromData,
} from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const route = useRoute()
const { $oryConfig } = useNuxtApp()

const loginFlowEndpoint = ref(config.oryUrl + '/self-service/login/browser')

const oryUiMsgs = ref([])
const email = ref('')
const username = ref('')
const password = ref('')
const confirmPassword = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
$oryConfig
  .getRegistrationFlow({ id: route.query.flow || '' })
  .then((r) => {
    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
  })
  // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
  // Any other error we just leave the page
  .catch((e) => {
    if (e.response.status === 404) {
      navigateTo(config.oryUrl + '/self-service/registration/browser', { external: true })
    } else {
      navigateTo('/')
    }
  })

async function registerPassword() {
  if (password.value !== confirmPassword.value) {
    oryUiMsgs.value = [{ text: 'Passwords do not match!' }]
    return;
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
async function registerGithub() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'github',
  }
  await registerGeneric(registrationFlowBody)
}

async function registerDiscord() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'discord',
  }
  await registerGeneric(registrationFlowBody)
}

async function registerGoogle() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'google',
  }
  await registerGeneric(registrationFlowBody)
}

async function registerMicrosoft() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'microsoft',
  }
  await registerGeneric(registrationFlowBody)
}

// loginFlowBody must match a variant of UpdateLoginFlowWith<method>Method (included are UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod)
async function registerGeneric(registrationFlowBody) {
  // Update registration flow using passed method of choice
  await $oryConfig
    .updateRegistrationFlow({
      flow: route.query.flow,
      updateRegistrationFlowBody: registrationFlowBody,
    })
    .then((_r) => {
      // If return_to exists, return to it, otherwise return to site main page
      const returnUrl = flowData.value.return_to || config.nuxtUrl
      window.location.href = returnUrl
    })
    .catch((e) => {
      // Using Social-integrated login/registration will return a 422: Unprocessable Entity error with a redirection link.
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
<style src="~/assets/login.css"></style>
