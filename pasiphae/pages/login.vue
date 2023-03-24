<template>
  <div v-if="flowData" id="login" >
    Login to your account.
    <form @submit.prevent="loginPassword">
      <input v-model="email" placeholder="email" />
      <input v-model="password" placeholder="password" type="password" />
      <input type="submit" value="log in" />
    </form>

    <li v-for="oryUiMsg in oryUiMsgs" :key="oryUiMsg">
      {{ oryUiMsg.text }}
    </li>

    <form @submit.prevent="loginDiscord">
      <input type="submit" value="log in with discord" />
    </form>
  
   <form @submit.prevent="loginGithub">
      <input type="submit" value="log in with github" />
    </form>
    <form @submit.prevent="loginGoogle">
      <input type="submit" value="log in with google" />
    </form> 
    <form @submit.prevent="loginMicrosoft">
      <input type="submit" value="log in with microsoft (requires https redirect uri)" />
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
} from '~/helpers/ory-ui-extract'

const config = useRuntimeConfig()
const route = useRoute()
const { $oryConfig } = useNuxtApp()

const oryUiMsgs = ref([])
const email = ref('')
const password = ref('')

// Attempt to get flow information on page load
const flowData = ref(null);
$oryConfig.getLoginFlow({ id: route.query.flow || "" })
.then( r =>  {
  flowData.value = r.data;
  oryUiMsgs.value = extractNestedErrorMessagesFromData(r.data)})
// Failure to get flow information means a valid flow does not exist as a query parameter, so we redirect to regenerate it
// Any other error we just leave the page
.catch( e => {
  if (e.response.status === 404)  {
    window.location.href = config.oryUrl + '/self-service/login/browser'
  } else {
    window.location.href = config.nuxtUrl;
  }
});

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
    .then(( _r ) => {
    
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
