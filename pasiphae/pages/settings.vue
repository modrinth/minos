<template>
  <template v-if="flowData">
    <h1>Edit your password</h1>
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <input v-model="password" placeholder="Password" type="password" />
    <input v-model="confirmPassword" placeholder="Confirm Password" type="password" />
    <button @click="updatePassword" class="btn btn-primary continue-btn">Reset password</button>
    <div class="text-divider">
      <div></div>
    </div>
    <div v-if="linkProviders.length > 0">
      <h1>Link a social account</h1>
      <div class="third-party-link">
        <Button
          v-for="provider in linkProviders"
          :key="provider"
          :class="`${provider}-btn`"
          @click="link(provider)"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
    </div>
    <div v-if="unlinkProviders.length > 0">
      <div class="text-divider">
        <div></div>
      </div>
    </div>
    <div v-if="unlinkProviders.length > 0">
      <h1>Unlink a social account</h1>
      <div class="third-party-unlink">
        <Button
          v-for="provider in unlinkProviders"
          :key="provider"
          :class="`${provider}-btn`"
          @click="unlink(provider)"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
    </div>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromData,
  extractOidcLinkProviders,
  extractOidcUnlinkProviders,
} from '~/helpers/ory-ui-extract'
import { Button, GitHubIcon } from 'omorphia'
import DiscordIcon from '@/assets/discord.svg'
import GoogleIcon from '@/assets/google.svg'
import AppleIcon from '@/assets/apple.svg'
import MicrosoftIcon from '@/assets/microsoft.svg'
import GitLabIcon from '@/assets/gitlab.svg'

const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([])
const password = ref('')
const confirmPassword = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const linkProviders = ref([])
const unlinkProviders = ref([])

async function updateFlow() {
  $oryConfig
    .getSettingsFlow({ id: route.query.flow || '' })
    .then((r) => {
      console.log('testy temp')
      flowData.value = r.data
      linkProviders.value = extractOidcLinkProviders(r.data)
      unlinkProviders.value = extractOidcUnlinkProviders(r.data)
      oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
    })
    // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
    // Any other error we just leave the page
    .catch((e) => {
      console.log('go')
      console.log(e)
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        window.location.href = e.response.data.redirect_browser_to
      } else if ('response' in e && e.response.status === 404) {
        navigateTo(config.oryUrl + '/self-service/settings/browser', { external: true })
      } else {
        console.log(e)
        navigateTo('/')
      }
    })
}
updateFlow()

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

async function link(provider) {
  await linkOidc(provider, 'link')
}
async function unlink(provider) {
  await linkOidc(provider, 'unlink')
}

function getIcon(provider) {
  return icons[provider]
}

async function linkOidc(provider, link_or_unlink) {
  let update = {
    flow: route.query.flow,
    updateSettingsFlowBody: {
      csrf_token: extractNestedCsrfToken(flowData.value),
      method: 'profile',
      traits: flowData.value.identity.traits,
    },
  }
  if (link_or_unlink == 'link') {
    update.updateSettingsFlowBody.link = provider
  } else {
    update.updateSettingsFlowBody.unlink = provider
  }
  console.log(update)

  $oryConfig
    .updateSettingsFlow(update)
    .then((_r) => {
      // If return_to exists, return to it, otherwise refresh data
      const returnUrl = flowData.value.return_to
      if (returnUrl) {
        window.location.href = returnUrl
      } else {
        updateFlow()
      }
    })
    .catch((e) => {
      console.log('hi')
      console.log(e)
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

// Uses settings flow to update a logged-in user's password
async function updatePassword() {
  if (password.value !== confirmPassword.value) {
    oryUiMsgs.value = [{ text: 'Passwords do not match!' }]
    return
  }

  // updateSettingsFlow can match one of:
  // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
  // For different ways to things to change - some lookup value, or password.
  // In this case, we use UpdateSettingsFlowWithPasswordMethod to update password
  await $oryConfig
    .updateSettingsFlow({
      flow: route.query.flow,
      updateSettingsFlowBody: {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        method: 'password',
        password: password.value,
      },
    })
    .then((_r) => {
      oryUiMsgs.value = [{ text: 'Successful pass change.' }]
    })
    .catch((e) => {
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        window.location.href = e.response.data.redirect_browser_to
      } else {
        // Get displayable error messsages from nested returned Ory UI elements
        oryUiMsgs.value = extractNestedErrorMessagesFromError(e)
      }
    })
}
</script>
<style src="~/assets/login.css"></style>
