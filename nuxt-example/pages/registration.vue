<template>
  <div id="registration">
    <form @submit.prevent="registerPassword">
      <input v-model="email" placeholder="email">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="sign up">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

    <form @submit.prevent="registerGithub">
      <input type="submit" value="register with github">
    </form>
    <form @submit.prevent="registerDiscord">
      <input type="submit" value="register with discord">
    </form>
    <form @submit.prevent="registerMicrosoft">
      <input type="submit" value="register with microsoft">
    </form>
    <form @submit.prevent="registerGoogle">
      <input type="submit" value="register with google">
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
  name: "registration",
  data() {
    return {
      ory_ui_msgs: [
        { text: 'Please register!' },
      ],
      email: "",
      password: ""
    };
  },
  methods: {
    async registerPassword() {
      // There are several preset ways to identify people
      // https://www.ory.sh/docs/kratos/manage-identities/identity-schema
      // -email and password (seems ideal, and the one I have currently setup)
      // - user and password (they cannot reset their email)
      // - newsletter opt in
      // These are differentiated by the traits obj
      // These are *separate* to social logins which get mapped to these presets. 
      let registrationFlowBody = {
          csrf_token: '', // set in generic function
          method: "password",
          password: this.password,
          "traits.email": this.email,
        }
        this.registerGeneric(registrationFlowBody).then(
          
        )

    },
    async registerGithub(){
      let registrationFlowBody = { // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
          csrf_token: '',// set in generic function
          method: "oidc",
          provider: "github",
      }
      this.registerGeneric(registrationFlowBody)
    },
    async registerDiscord(){
      
      let registrationFlowBody = { // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "discord",
      }
      this.registerGeneric(registrationFlowBody)
    },
    async registerGoogle(){
      let registrationFlowBody = { // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "google",
      }
      this.registerGeneric(registrationFlowBody)
    },
    async registerMicrosoft(){
      let registrationFlowBody = { // registrationFlowBody is an instance of UpdateRegistrationFlowWithOidcMethod
          csrf_token: '', // set in generic function
          method: "oidc",
          provider: "microsoft",
      }
      this.registerGeneric(registrationFlowBody)
    },

    async registerGeneric(registrationFlowBody) {
      // getRegistrationFlow
      // -> get login flow object, to get proper query link and csrf token
      // 'id' is id of the login flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getRegistrationFlow({id:this.$route.query.flow})

      let returned_nodes = flow_data.data.ui.nodes;
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name=="csrf_token"){
          registrationFlowBody.csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      await ory.updateRegistrationFlow({
        flow: this.$route.query.flow,
        updateRegistrationFlowBody: registrationFlowBody
      }).then( r => {
          console.log("Successfully register!");
          // If return_to exists, return to it, otherwise return to main page
          const apiUrl = process.env.NUXT_ENV_API_URL || "http://localhost:4455"
          let return_url = flow_data.data.return_to || apiUrl;
          window.location.href = return_url;
      }

      ).catch(e => {
        // Using Social-integrated login/registration will return a 422: Unprocessable Entity error with a redirection link.
        // We use this to continue the flow.
        if (e.response.status == 422) {
          window.location.href = e.response.data.redirect_browser_to;
          return;
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
