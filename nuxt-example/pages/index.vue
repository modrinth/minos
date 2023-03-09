
<template>
  <div class="main">

    <div v-if="!session">
      <p>Click on "login" or "Sign Up" below to sign in.</p>
      <li><a :href="login_flow_endpoint" data-testid="sign-in">Login</a></li>
      <li><a :href="register_flow_endpoint" data-testid="sign-up">Sign Up</a></li>
      <li><a :href="recover_flow_endpoint" data-testid="sign-up">Recover account</a></li>

      <!-- The settings button will REDIRECT to the login one if there is no login! -->
      <li><a :href="settings_flow_endpoint" data-testid="sign-up">Adjust settings</a></li>
      
    </div>

    <h3 v-if="session">Calling <code>toSession()</code></h3>
    <div v-if="session" class="long">
      <p>
        Use the Ory SDK's <code>toSession()</code> call to receive the session
        information in-browser. Here is a snippet, such as the authentication methods used:
      </p>
      <pre><code data-testid='ory-response'>{{ session.authentication_methods }}</code></pre>
      <p> This is a good way to check if they are logged in with a valid cookie, as toSession will fail if they are not.</p>
    </div>



    <h3 v-if="!apiResponse">API Response</h3>
    <div v-if="!apiResponse" class="long">
      <p>
          Has not yet authenticated with Minos. (API response failed. If you are logged in, ensure Minos is running)
      </p>
    </div>


    <h3 v-if="apiResponse">API Response</h3>
    <div v-if="apiResponse" class="long">
      <p>
        You can make authenticated calls to Minos API by sending the cookie to the test endpoint when the server is running <code>/demo</code>.
        <br> If you are seeing this, it was successful and successfully authenticated.
      <br>This will return 401 if it fails, if it succeeds it will return 200 with the following session object attached.
      </p>
      <pre><code data-testid='api-response'>{{ apiResponse }}</code></pre>
    </div>

    <h3 v-if="session">Common Actions</h3>
    <ul v-if="session">
      <li><a :href="logoutUrl" data-testid="logout">Logout</a></li>
      <li>
        <a :href="settings_flow_endpoint" data-testid="settings">Settings</a>
      </li>
    </ul>

    <h3>Essential Links</h3>
    <ul>
      <li><a href="https://www.ory.sh">Ory Website</a></li>
      <li><a href="https://github.com/ory">Ory GitHub</a></li>
      <li><a href="https://www.ory.sh/docs">Documentation</a></li>
    </ul>
  </div>
</template>

<script>
import { FrontendApi, Configuration } from "@ory/client"

// The basePath points to the location of Ory's APIs.
// You can use https://<slug>.projects.oryapis.com/ here because cookies can not
// easily be shared across different domains.
//
// In the next step, we will run a process to mirror Ory's APIs
// on your local machine using the Ory Tunnel at http://localhost:4000
const basePath = process.env.NUXT_ENV_ORY_URL || "http://localhost:4433"
const ory = new FrontendApi(
    new Configuration({
      basePath,
      baseOptions: {
        withCredentials: true, // Ensures we send cookies in the CORS requests.
      },
    }),
  )

// These links will *generate* a flow (like how it creates one in ory.createBrowserLogoutFlow()) and redirect back to our UI
// ory.createBrowserLoginFlow() will also work for this if you want to keep it all in one page, but these links will
// redirect back to our UI (and the URLS set in the ory manager) with the generated flow.
let login_flow_endpoint = basePath + '/self-service/login/browser';
let register_flow_endpoint = basePath + '/self-service/registration/browser'
let recover_flow_endpoint = basePath + '/self-service/recovery/browser'
let settings_flow_endpoint = basePath + '/self-service/settings/browser' // this one redirects to login if necessary, otherwise creates a new session flow

export default {
  name: "ory-kratos",
  data() {
    return {
      session: null,
      logoutUrl: null,
      apiResponse: null,
      basePath,
      login_flow_endpoint,
      register_flow_endpoint,
      recover_flow_endpoint,
      settings_flow_endpoint,
    }
  },


  mounted() {
    // Fetch the session directly from Ory
    // Authentication is successful if cookie represents a valid Ory Session
    ory.toSession().then(({ data }) => {
      this.session = data

      // If the user is logged in, we want to show a logout link!
      ory.createBrowserLogoutFlow().then(({ data }) => {
        this.logoutUrl = data.logout_url
      })
    })

    // Makean authenticated request to Minos API
    // /demo is a test endpoint that returns 200 if successful authentication, 401 if failure to authenticate.
    // (This does the same authentication check as the ory.toSession() above, but remotely in the Minos endpoint on the Rust side)
    // response body is that session object
    const minosUrl = process.env.NUXT_ENV_MINOS_URL || "http://localhost:4000"
    fetch(minosUrl + "/demo", { // "/"
      // Do not forget to set this - it is required to send the session cookie!
      credentials: "include",
    })
      .then(
        (res) => {
          console.log("Successfully made contact and authenticated with Minos!");
          res.json().then((res) => {
            this.apiResponse = res
          });
        }
      ).catch(
        (e) => {
          console.log("Failed to authenticate with Minos!");
          console.log(e.data);
        }
      )


  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.main {
  max-width: 400px;
  margin: 0 auto;
}
</style>