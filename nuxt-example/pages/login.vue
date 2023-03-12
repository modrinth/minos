<template>
  <div id="login">
    <form @submit.prevent="loginPassword">
      <input v-model="email" placeholder="email">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="log in">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

    <form @submit.prevent="loginGithub">
      <input type="submit" value="log in with github">
    </form>

    <form @submit.prevent="loginDiscord">
      <input type="submit" value="log in with discord">
    </form>

    <form @submit.prevent="loginMicrosoft">
      <input type="submit" value="log in with microsoft">
    </form>

    <form @submit.prevent="loginGoogle">
      <input type="submit" value="log in with google">
    </form>

    <p> Note: authorizing via  google will only work for a couple accounts that I've preauthorized. (I can add more, just message me.)</p>
    <li>wyatt@modrinth.com</li>
        <li>jai@modrinth.com</li>
        <li>wverchere@gmail.com</li>

      <p> This is because Google does not allow signin with any account until the oauth app is reviewed and published.</p>
      



  </div>
</template>

<script>
import { FrontendApi, Configuration } from "@ory/client"
const basePath = process.env.NUXT_ENV_ORY_URL || "http://localhost:4433"
const ory = new FrontendApi(
    new Configuration({
      basePath,
      baseOptions: {
        withCredentials: true, // Ensures we send cookies in the CORS requests.
      },
    }),
  )
  
export default {
  name: "login",
  data() {
    return {
      ory_ui_msgs: [
        { text: 'Please login!' },
      ],
      email: "",
      password: ""
    };
  },

  methods: {
    async loginPassword() {
      // loginFlowBody is an instance of UpdateLoginFlowWithPasswordMethod
      let loginFlowBody = {
        csrf_token: '', // set in generic function
        identifier: this.email,
        method: "password",
        password: this.password,
      }
        await this.loginGeneric(loginFlowBody);
    },
    async loginGithub() {
      // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
        let loginFlowBody = {
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "github",
        }
        await this.loginGeneric(loginFlowBody);
    },

    async loginDiscord() {
      // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
        let loginFlowBody = {
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "discord",
        }
        await this.loginGeneric(loginFlowBody);
    },
    async loginGoogle() {
      // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
        let loginFlowBody = {
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "google",
        }
        await this.loginGeneric(loginFlowBody);
    },
    async loginMicrosoft() {
      // loginFlowBody is an instance of UpdateLoginFlowWithOidcMethod
        let loginFlowBody = {
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "microsoft",
        }
        await this.loginGeneric(loginFlowBody);
    },




    async loginGeneric(loginFlowBody) {
      // getLoginFlow
      // -> get login flow object, to get proper query link and csrf token
      // 'id' is id of the login flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getLoginFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name=="csrf_token"){
          loginFlowBody.csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      // updateLoginFlowBody can match one of:
      // UpdateLoginFlowWithLookupSecretMethod | UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod | UpdateLoginFlowWithTotpMethod | UpdateLoginFlowWithWebAuthnMethod
      // For different ways to authenticate

      await ory.updateLoginFlow({
        flow: this.$route.query.flow,
        updateLoginFlowBody: loginFlowBody
        //xSessionToken?
        //cookie?
      }).then( r => {
          console.log("Successful login!");
          // If return_to exists, return to it, otherwise return to main page
          const apiUrl = process.env.NUXT_ENV_API_URL || "http://localhost:4455"
          let return_url = flow_data.data.return_to || apiUrl;
          window.location.href = return_url;
      }

      ).catch(e => {

        // IMPORTANT: The redirect browser page might be contained with an error.
        console.log(e.response)
        console.log(e.response.data.redirect_browser_to);
        if("data" in e.response){
          if ("redirect_browser_to" in e.response) {
            window.location.href = e.response.data.redirect_browser_to;
          }
        }

        console.log(e)
        console.log(e.response)
        let ary = []
        if ("data" in e.response)
          if ("messages" in e.response.data.ui) {
            ary = ary.concat(e.response.data.ui.messages)
          } else if ("nodes" in e.response.data.ui) {
            for(let i = 0; i < e.response.data.ui.nodes.length; i ++){
              let node = e.response.data.ui.nodes[i];
              ary = ary.concat(node.messages)
            }
          }
        this.ory_ui_msgs = ary;
      });
    }

  }
};
</script>
