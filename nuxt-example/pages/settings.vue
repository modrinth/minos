<template>
  <div id="settings">
    <form @submit.prevent="settings">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="change">
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
  name: "settings",
  data() {
    return {
      ory_ui_msgs: [
        { text: 'Please change your password!' },
      ],
      password: ""
    };
  },
  methods: {
    async settings() {
      // getSettingsFlow
      // -> get settings flow object, to get proper query link and csrf token
      // 'id' is id of the settings flow, which is passed as a parameter if redirected to this page through Ory 
      // (If non-existent, we could eventually send them back to the Ory flow generation link (to redirect here with flow id), or back to the index page, etc)
      let flow_data = await ory.getSettingsFlow({id:this.$route.query.flow})

      // get csrf token. probably a better way to do this, but its returned directly in flow_data above
      let returned_nodes = flow_data.data.ui.nodes;
      let csrf_token = '';
      for (let i = 0; i < returned_nodes.length; i++){
        if (returned_nodes[i].attributes.name="csrf_token"){
          csrf_token=returned_nodes[i].attributes.value;
          break;
        }
      }

      // updateSettingsFlow can match one of:
      // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
      // For different ways to things to change - some lookup value, or password.
      // In this case, it matches UpdateSettingsFlowWithPasswordMethod to update password
      const password = this.password;
      let login_attempt = await ory.updateSettingsFlow({
        flow: this.$route.query.flow,
        updateSettingsFlowBody : {
          csrf_token: csrf_token, // must be directly sent
          method: "password",
          password: password,
        },
      }).then( r => {
          console.log("Successful pass change.");
          this.ory_ui_msgs = [{text: "Successful pass change."}]
      }
      ).catch(e => {
        console.log(e)
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
