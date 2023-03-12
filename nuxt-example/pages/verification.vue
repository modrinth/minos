<template>
  <div id="verification">
    <form @submit.prevent="verification">
      <input v-model="code" placeholder="code">
      <input type="submit" value="verification code">
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

let login_flow_endpoint = basePath + '/self-service/login/browser';


export default {
  name: "verification",
  data() {
    return {
      ory_ui_msgs: [
        { text: 'Please enter your verification code!' },
      ],
      code: '',
    }
  },
  mounted() {
    // On load, do a quick check for status of the verification flow
    // If they clicked on the email link and the flow is still the same, the flow.data.ui object
    // will contain 'code' amongst its UI nodes with the verification code- which ideally can be put automatically
    // into the field so they can just verify it and continue.
    let flow_data = ory.getVerificationFlow({id:this.$route.query.flow}).then(
      flow => {
        console.log(flow);
        let returned_nodes = flow.data.ui.nodes;
        console.log(returned_nodes)
        for (let i = 0; i < returned_nodes.length; i++){
          if (returned_nodes[i].group=="code" && returned_nodes[i].attributes.name=="code"){
            this.code=returned_nodes[i].attributes.value;
            break;
          }
        }
      }
    )

  },


  methods: {
    async verification() {
      // getVerificationFlow
      // -> get verification flow object, to get proper query link and csrf token
        // 'id' is id of the verification flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getVerificationFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name=="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }
  
      // updateVerificationFlow 
      let recovery_attempt = await ory.updateVerificationFlow({
        flow: this.$route.query.flow,
        updateVerificationFlowBody: {
          csrf_token: csrf_token, // must be directly sent
          code: this.code,
          method: "code",
      }
      }).then( r => {
          console.log("Successfully verified account!");
          this.ory_ui_msgs = [{text: "Email successfully verified."}]
          // Success! We can maybe then direct them to the login page? 
      }
      ).catch(e => {
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
    },
  }
};
</script>
