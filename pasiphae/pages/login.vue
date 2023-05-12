<template>
  <template v-if="flowData">
    <h1>Continue with</h1>
    <div class="third-party">
      <Button class="discord-btn" @click="loginDiscord"
        ><DiscordIcon /> <span>Discord</span></Button
      >
      <Button class="github-btn" @click="loginGithub"><GitHubIcon /> <span>GitHub</span></Button>
      <Button class="microsoft-btn" @click="loginMicrosoft"
        ><MicrosoftIcon /> <span>Microsoft</span></Button
      >
      <Button class="google-btn" @click="loginGoogle"><GoogleIcon /> <span>Google</span></Button>
      <Button class="apple-btn" @click="loginApple"><AppleIcon /> <span>Apple</span></Button>
      <Button class="gitlab-btn" @click="loginGitlab"><GitLabIcon /> <span>GitLab</span></Button>
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
  extractNestedErrorMessagesFromData,
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
$oryConfig
  .getLoginFlow({ id: route.query.flow || '' })
  .then((r) => {
    flowData.value = r.data
    oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
  })
  // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
  // Any other error we just leave the page
  .catch((e) => {
    if (e.response.status === 404) {
      navigateTo(config.oryUrl + '/self-service/login/browser', { external: true })
    } else {
      navigateTo('/')
    }
  })

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

async function loginGithub() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'github',
  }
  await loginGeneric(loginFlowBody)
}

async function loginDiscord() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'discord',
  }
  await loginGeneric(loginFlowBody)
}

async function loginGoogle() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'google',
  }
  await loginGeneric(loginFlowBody)
}

async function loginApple() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'apple',
  }
  await loginGeneric(loginFlowBody)
}

async function loginGitlab() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'gitlab',
  }
  await loginGeneric(loginFlowBody)
}

async function loginMicrosoft() {
  // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
  const loginFlowBody = {
    csrf_token: extractNestedCsrfToken(flowData.value), // set in generic function
    method: 'oidc',
    provider: 'microsoft',
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
<style scoped>
.account-options {
  display: flex;
  width: 100%;
}

.account-options a {
  margin-left: auto;
}
</style>
