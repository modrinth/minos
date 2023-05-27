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
          @click="linkOidc(provider,'link')"
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
          @click="linkOidc(provider,'unlink')"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
    </div>
    <div class="text-divider">
    </div>
    <div class="totp" v-if="totpQRImage">
    <h1>Connect a secondary authentication provider</h1>
        <img v-bind:src="totpQRImage" :width="totpQRWidth" :height="totpQRHeight" alt="QR Image">
        <p>Currently, there is no authenticator app linked to your account.</p>
        <p>To add one, scan the QR code with your authenticator app, or enter the following secret manually.</p>
    {{ totpSecret }}
    <p>Confirm your provider by entering the provider's code below:</p>
    <input v-model="totp_code" placeholder="TOTP Code" type="text" />
    <button @click="linkAuthenticator('link')" class="btn btn-primary continue-btn">Link authenticator app</button>
  </div>
  <div v-else>
    <button @click="linkAuthenticator('unlink')" class="btn btn-primary continue-btn">Unlink authenticator app</button>
  </div>
  <div class="text-divider"></div>
  <div class="totp">
       <button @click="generateCodes" class="btn btn-primary continue-btn">:Generate backup codes</button>
    </div>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromUiData,
  extractNestedTotpData,
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
const totp_code = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const linkProviders = ref([])
const unlinkProviders = ref([])

const totpQRImage = ref(null)
const totpQRWidth = ref(null)
const totpQRHeight = ref(null)
const totpSecret = ref(null)



async function updateFlow() {
  $oryConfig
    .getSettingsFlow({ id: route.query.flow || '' })
    .then((r) => {
      flowData.value = r.data
      linkProviders.value = extractOidcLinkProviders(r.data)
      unlinkProviders.value = extractOidcUnlinkProviders(r.data)
      oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)
      console.log(JSON.stringify(r.data))

      let totp = extractNestedTotpData(r.data)
      if (totp.image && totp.secret) {
        totpQRImage.value = totp.image.src
        totpQRWidth.value = totp.image.width
        totpQRHeight.value = totp. image.height
        totpSecret.value = totp.secret
      }
        
    })
    // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
    // Any other error we just leave the page
    .catch((e) => {
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        window.location.href = e.response.data.redirect_browser_to
      } else if ('response' in e && e.response.status === 404) {
        navigateTo(config.oryUrl + '/self-service/settings/browser', { external: true })
      } else {
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

function getIcon(provider) {
  return icons[provider]
}


async function linkOidc(provider, link_or_unlink) {
  let updateSettingsFlowBody = {
      method: 'profile',
      traits: flowData.value.identity.traits,
  };

  if (link_or_unlink == 'link') {
    updateSettingsFlowBody.link = provider
  } else {
    updateSettingsFlowBody.unlink = provider
  }

  await sendUpdate(updateSettingsFlowBody)

}

// Attempt to link to an authentication app (or unlink if already connected)
// Should only be able to link to one (if unlink button is displayed, link should not be)
async function linkAuthenticator(link_or_unlink) {
  let updateSettingsFlowBody = {
      method: 'totp',
      totp_code: totp_code.value, 
      totp_unlink: link_or_unlink == 'unlink',
    };
  await sendUpdate(updateSettingsFlowBody)
}

async function generateCodes() {
  // stub, TODO
}


// Uses settings flow to attempt to update a logged-in user's password
async function updatePassword() {
  if (password.value !== confirmPassword.value) {
    oryUiMsgs.value = [{ text: 'Passwords do not match!' }]
    return
  }

  let updateSettingsFlowBody = {
        csrf_token: extractNestedCsrfToken(flowData.value), // must be directly set
        method: 'password',
        password: password.value,
      }
  await sendUpdate(updateSettingsFlowBody)
}

  // updateSettingsFlow can match one of:
  // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
  // For different ways to things to change - some lookup value, or password.
  // For example, we use UpdateSettingsFlowWithPasswordMethod to update password
async function sendUpdate(updateSettingsFlowBody) {
  let csrf_token = extractNestedCsrfToken(flowData.value); // must be directly set
  updateSettingsFlowBody.csrf_token = csrf_token;

  await $oryConfig
    .updateSettingsFlow({
      flow: route.query.flow,
      updateSettingsFlowBody: updateSettingsFlowBody,
    })
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
