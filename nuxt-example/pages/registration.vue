<template>
  <div id="registration">
    <form @submit.prevent="register">
      <input v-model="email" placeholder="email">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="sign up">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

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
    async register() {
      // getRegistrationFlow
      // -> get registration flow object, to get proper query link and csrf token
      // 'id' is id of the registration flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getRegistrationFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      // There are several preset ways to identify people
      // https://www.ory.sh/docs/kratos/manage-identities/identity-schema
      // -email and password (seems ideal)
      // - user and password (they cannot reset their email)
      // - newsletter opt in
      // These are differentiated by the traits obj described above
      // These are *separate* to social logins which get mapped to these presets. (ie if we want modrinth github, we map them over to this)
      // let traits = {
      //   email: this.email,
      //   tos: true,
      // };
      let traits = {
        "traits.username": this.email,
        "traits.tos": true,
      };

      // updateLoginFlowBody can match one of:
      // UpdateRegistrationFlowWithOidcMethod | UpdateRegistrationFlowWithPasswordMethod | UpdateRegistrationFlowWithWebAuthnMethod
      // For different ways to authenticate
      // In this case, it matches UpdateRegistrationFlowWithPasswordMethod
      const password = this.password;
      let login_attempt = await ory.updateRegistrationFlow({
        flow: this.$route.query.flow,
        updateRegistrationFlowBody: {
          csrf_token: csrf_token, // must be directly sent
          // identifier: email,
          method: "password",
          password: password,
          "traits.username": this.email,
        }
        //xSessionToken?
        //cookie?
      }).then( r => {
        // If return_to exists, return to it, otherwise return to main page
        // (or this could redirect to some sort of registration success)
        // (For registration, return_to makes more sense if we have auto login after registration enabled)
        const apiUrl = process.env.NUXT_ENV_API_URL || "http://test:3000"
        let return_url = flow_data.data.return_to || apiUrl;
        window.location.href = return_url;
      }

        // Worked! redirect back to main page OR the page you started at
        // (optional: very easy to make you stay logged in after initial registeraation)
      ).catch(e => {
        console.log(e.response)
        let ary = []
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
