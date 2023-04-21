<template>
  <div v-if="flowData" id="registration" >
    Register for a new account.

    <form @submit.prevent="registerPassword">
      <input v-model="email" placeholder="email" />
      <input v-model="password" placeholder="password" type="password" />
      <input type="submit" value="sign up" />
    </form>

    <li v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
      {{ oryUiMsg.text }}
    </li>

    <form @submit.prevent="registerDiscord">
      <input type="submit" value="register with discord" />
    </form>
   <form @submit.prevent="registerGithub">
      <input type="submit" value="register with github" />
    </form>
    <form @submit.prevent="registerGoogle">
      <input type="submit" value="register with google" />
    </form> 
    <form @submit.prevent="registerMicrosoft">
      <input type="submit" value="register with microsoft (requires https redirect uri)" />
    </form>
 

    <p>
      Note: authorizing via google will only work for a couple accounts that I've preauthorized. (I
      can add more, just message me.)
    </p>
    <li>wyatt@modrinth.com</li>
    <li>jai@modrinth.com</li>
    <li>wverchere@gmail.com</li>

    <p>
      This is because Google does not allow signin with any account until the oauth app is reviewed
      and published.
    </p>

    <NuxtLink to="/">Home page</NuxtLink>

  </div>
</template>

<script setup>
import {
  extractNestedCsrfToken,
  extractNestedErrorMessagesFromError,
  extractNestedErrorMessagesFromData
} from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([])
const email = ref('')
const password = ref('')

// Attempt to get flow information on page load
const flowData = ref(null);
$oryConfig.getRegistrationFlow({ id: route.query.flow || "" })
.then( r =>  {
  flowData.value = r.data;
  oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)
})
// Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
// Any other error we just leave the page
.catch( e => {
  if (e.response.status === 404)  {
    window.location.href = config.oryUrl + '/self-service/registration/browser'
  } else {
    window.location.href = config.nuxtUrl;
  }
});


async function registerPassword() {
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
  }
  await registerGeneric(registrationFlowBody);
}
async function registerGithub() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'github',
  }
  await registerGeneric(registrationFlowBody);
}

async function registerDiscord() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'discord',
  }
  await registerGeneric(registrationFlowBody);
}

async function registerGoogle() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'google',
  }
  await registerGeneric(registrationFlowBody);
}

async function registerMicrosoft() {
  const registrationFlowBody = {
    // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
    csrf_token: extractNestedCsrfToken(flowData.value),
    method: 'oidc',
    provider: 'microsoft',
  }
  await registerGeneric(registrationFlowBody);
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
