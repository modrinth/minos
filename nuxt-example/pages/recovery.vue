<template>
  <div id="recovery">
    <form @submit.prevent="recovery">
      <input v-model="email" placeholder="email">
      <input type="submit" value="send email recovery link">
    </form>
    <form @submit.prevent="code">
      <input v-model="code" placeholder="code">
      <input type="submit" value="recover using code and email">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

  </div>
</template>

<script>
import { FrontendApi, Configuration } from "@ory/client"
const basePath = process.env.NUXT_ENV_ORY_URL || "http://test:4000"
const ory = new FrontendApi(
    new Configuration({
      basePath,
      baseOptions: {
        withCredentials: true, // Ensures we send cookies in the CORS requests.
      },
    }),
  )

export default {
  name: "recovery",
  data() {
    return {
      ory_ui_msgs: [
        { text: 'Attempt to recover your account!' },
      ],
      email: "",
      code: ""
    };
  },
  methods: {
    async recovery() {
      // getRecoveryFlow
      // -> get recovery flow object, to get proper query link and csrf token
        // 'id' is id of the recovery flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getRecoveryFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }
  
      // updateRecoveryFlowBody can match one of:

      // For different ways to authenticate
      // In this case, it matches UpdateRecoveryFlowWithPasswordMethod
      const email = this.email;
      const code = this.code;
      let recovery_attempt = await ory.updateRecoveryFlow({
        flow: this.$route.query.flow,
        updateRecoveryFlowBody: {
          csrf_token: csrf_token, // must be directly sent
          email: email, // MUST be an email identifier, not just a usernmae
          method: "code",
      }
      }).then( r => {
          console.log("Successful recovery - part 1");
          this.ory_ui_msgs = [{text: "Sent recovery email."}]
      }
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
    },

    async code() {
      // getRecoveryFlow
      // -> get recovery flow object, to get proper query link and csrf token
      // 'id' is id of the recovery flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getRecoveryFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      // updateRecoveryFlowBody can match one of:
      // UpdateRecoveryFlowWithLookupSecretMethod | UpdateRecoveryFlowWithOidcMethod | UpdateRecoveryFlowWithPasswordMethod | UpdateRecoveryFlowWithTotpMethod | UpdateRecoveryFlowWithWebAuthnMethod
      // For different ways to authenticate
      // In this case, it matches UpdateRecoveryFlowWithPasswordMethod
      const email = this.email;
      const code = this.code;
      let recovery_attempt = await ory.updateRecoveryFlow({
        flow: this.$route.query.flow,
        updateRecoveryFlowBody: {
          csrf_token: csrf_token, // must be directly sent
          email: email, // MUST be an email identifier, not just a usernmae
          method: "code",
          code: code
      }
      }).then( r => {
          console.log("Successful recovery!");
          // If return_to exists, return to it, otherwise return to main page
          const apiUrl = process.env.NUXT_ENV_API_URL || "http://test:3000"
          let return_url = flow_data.data.return_to || apiUrl;
          window.location.href = return_url;
      }

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
