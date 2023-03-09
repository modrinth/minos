<template>
  <div id="login">
    <form @submit.prevent="login">
      <input v-model="email" placeholder="email">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="log in">
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
    async login() {
      // getLoginFlow
      // -> get login flow object, to get proper query link and csrf token
      // 'id' is id of the login flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getLoginFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      // updateLoginFlowBody can match one of:
      // UpdateLoginFlowWithLookupSecretMethod | UpdateLoginFlowWithOidcMethod | UpdateLoginFlowWithPasswordMethod | UpdateLoginFlowWithTotpMethod | UpdateLoginFlowWithWebAuthnMethod
      // For different ways to authenticate
      // In this case, it matches UpdateLoginFlowWithPasswordMethod
      const email = this.email;
      const password = this.password;
      await ory.updateLoginFlow({
        flow: this.$route.query.flow,
        updateLoginFlowBody: {
          csrf_token: csrf_token, // must be directly sent
          identifier: email,
          method: "password",
          password: password,
        }
        //xSessionToken?
        //cookie?
      }).then( r => {
          console.log("Successful login!");
          // If return_to exists, return to it, otherwise return to main page
          const apiUrl = process.env.NUXT_ENV_API_URL || "http://localhost:4455"
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
