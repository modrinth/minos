<template>
  <template v-if="flowData">
    <div v-if="oryUiMsgs.length > 0" class="errors">
      <p v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
        {{ oryUiMsg.text }}
      </p>
    </div>
    <h1>Edit your password</h1>
    <input v-model="password" placeholder="Password" type="password" />
    <input v-model="confirmPassword" placeholder="Confirm Password" type="password" />
    <button @click="updatePassword" class="btn btn-primary continue-btn">Reset password</button>
    <div class="text-divider">
      <div></div>
      <span>or</span>
      <div></div>
    </div>
    <div v-if="linkProviders.length > 0">
      <h1>Link a social account</h1>
      <div class="third-party-link">
        <Button
          v-for="provider in linkProviders"
          :key="provider"
          :class="`${provider}-btn`"
          @click="linkOidc(provider, 'link')"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
    </div>
    <div v-if="unlinkProviders.length > 0">
      <div class="text-divider">
        <div></div>
        <span>or</span>
        <div></div>
      </div>
    </div>
    <div v-if="unlinkProviders.length > 0">
      <h1>Unlink a social account</h1>
      <div class="third-party-link">
        <Button
          v-for="provider in unlinkProviders"
          :key="provider"
          :class="`${provider}-btn`"
          @click="linkOidc(provider, 'unlink')"
        >
          <component :is="getIcon(provider)" /> <span>{{ capitalizeFirstLetter(provider) }}</span>
        </Button>
      </div>
    </div>
    <div class="text-divider">
      <div></div>
      <span>or</span>
      <div></div>
    </div>
    <template v-if="totpQRImage">
      <h1>Connect a secondary authentication provider</h1>
      <img v-bind:src="totpQRImage" :width="totpQRWidth" :height="totpQRHeight" alt="QR Image" />
      <p>Currently, there is no authenticator app linked to your account.</p>
      <p>
        To add one, scan the QR code with your authenticator app, or enter the following secret
        manually.
      </p>
      <div>{{ totpSecret }}</div>
      <br />
      <p>Confirm your provider by entering the provider's code below:</p>
      <input v-model="totpCode" placeholder="TOTP Code" type="text" />
      <button @click="linkAuthenticator('link')" class="btn btn-primary continue-btn">
        Link authenticator app
      </button>
    </template>
    <template v-else>
      <p>Unlink your authenticator app and remove 2FA from your account.</p>
      <template v-if="lookupEnabled">
        <p>Backup codes are enabled.</p>
        <p>
          If you unlink, you should also disable your backup codes, or your account will still
          require 2FA.
        </p>
      </template>
      <button @click="linkAuthenticator('unlink')" class="btn btn-primary continue-btn">
        Unlink authenticator app
      </button>
    </template>
    <div class="text-divider">
      <div></div>
      <span>or</span>
      <div></div>
    </div>

    <template v-if="lookupCodes.length > 0">
      <div class="totp-codes">
        <p v-for="code in lookupCodes" :key="code">
          {{ code }}
        </p>
      </div>
      <br />

      <p>
        Press 'confirm' to confirm and save these codes (overwriting the last ones). These will not
        be shown again, so store them in a safe place!
      </p>

      <button @click="generateCodes(true)" class="btn btn-primary continue-btn">
        Confirm backup codes
      </button>
    </template>

    <template v-if="showLookupRegenerate || showLookupDisable">
      <button @click="generateCodes(false)" class="btn btn-primary continue-btn">
        Regenerate backup codes
      </button>
    </template>
    <template v-if="totpQRImage && showLookupDisable">
      <p>
        <b>We recommend you disable your backup codes, or you add an authenticator app. </b>
      </p>
      <p>
        Currently, the lookup secrets are your only form of 2FA, which may result in you getting
        locked out of your account.
      </p>
    </template>

    <template v-if="showLookupDisable">
      <button @click="disableLookupSecrets()" class="btn btn-primary continue-btn">
        Disable backup codes
      </button>
    </template>
  </template>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromUiData,
  extractNestedLookupCodes,
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
const totpCode = ref('')

// Attempt to get flow information on page load
const flowData = ref(null)
const linkProviders = ref([])
const unlinkProviders = ref([])

const totpQRImage = ref(null)
const totpQRWidth = ref(null)
const totpQRHeight = ref(null)
const totpSecret = ref(null)

const lookupCodes = ref([])
const showLookupRegenerate = ref(false)
const showLookupDisable = ref(false)

async function updateFlow() {
  try {
    const r = await $oryConfig
    .getSettingsFlow({ id: route.query.flow || '' });

      flowData.value = r.data
      linkProviders.value = extractOidcLinkProviders(r.data)
      unlinkProviders.value = extractOidcUnlinkProviders(r.data)
      oryUiMsgs.value = extractNestedErrorMessagesFromUiData(r.data)

      let totp = extractNestedTotpData(r.data)
      if (totp.image && totp.secret) {
        totpQRImage.value = totp.image.src
        totpQRWidth.value = totp.image.width
        totpQRHeight.value = totp.image.height
        totpSecret.value = totp.secret
      } else {
        totpQRImage.value = null
        totpQRWidth.value = null
        totpQRHeight.value = null
        totpSecret.value = null
      }

      let look = extractNestedLookupCodes(r.data)
      if (look) {
        lookupCodes.value = look.codes
        showLookupRegenerate.value = look.regenerateButton
        showLookupDisable.value = look.disableButton
      }
    }
    // Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
    // Any other error we just leave the page
    catch(e) {
      if ('response' in e && 'data' in e.response && 'redirect_browser_to' in e.response.data) {
        navigateTo(e.response.data.redirect_browser_to, { external: true })
      } else if ('response' in e && e.response.status === 404) {
        navigateTo(config.oryUrl + '/self-service/settings/browser', { external: true })
      } else {
        navigateTo('/')
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

async function linkOidc(provider, link_or_unlink) {
  let updateSettingsFlowBody = {
    method: 'profile',
    traits: flowData.value.identity.traits,
  }

  if (link_or_unlink == 'link') {
    updateSettingsFlowBody.link = provider
    await sendUpdate(updateSettingsFlowBody)
  } else {
    updateSettingsFlowBody.unlink = provider
    await sendUpdate(updateSettingsFlowBody)
  }
}

// Attempt to link to an authentication app (or unlink if already connected)
// Should only be able to link to one (if unlink button is displayed, link should not be)
async function linkAuthenticator(link_or_unlink) {
  let updateSettingsFlowBody = {
    method: 'totp',
    totp_code: totpCode.value,
    totp_unlink: link_or_unlink == 'unlink',
  }
  await sendUpdate(updateSettingsFlowBody)
}

// Regenerates (or generates) backup codes
// Previous codes are invalidated
async function generateCodes(confirm) {
  let regenerate = !confirm
  let updateSettingsFlowBody = {
    method: 'lookup',
    lookup_secret_regenerate: regenerate,
    lookup_secret_confirm: confirm,
  }

  if (confirm) {
    updateSettingsFlowBody.lookup_secret_disable = false
  }
  await sendUpdate(updateSettingsFlowBody)
}

// Enables or disables backup codes
// If enabling, generates a set of codes
async function disableLookupSecrets() {
  let updateSettingsFlowBody = {
    method: 'lookup',
    lookup_secret_disable: true,
    lookup_secret_regenerate: false,
  }
  await sendUpdate(updateSettingsFlowBody)
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

// updateSettingsFlow can `match one of:
// UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
// For different ways to things to change - some lookup value, or password.
// For example, we use UpdateSettingsFlowWithPasswordMethod to update password
async function sendUpdate(updateSettingsFlowBody) {
  let csrf_token = extractNestedCsrfToken(flowData.value) // must be directly set
  updateSettingsFlowBody.csrf_token = csrf_token

  try {
    await $oryConfig
    .updateSettingsFlow({
      flow: route.query.flow,
      updateSettingsFlowBody: updateSettingsFlowBody,
    })
    const returnUrl = flowData.value.return_to
      if (returnUrl) {
        navigateTo(returnUrl, { external: true })

      } else {
        await updateFlow()
      }

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
<style src="~/assets/settings.css"></style>
