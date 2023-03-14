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

<script setup>

import { extract_nested_csrf_token} from "~/helpers/ory-ui-extract"

let config = useRuntimeConfig();
let { $oryConfig } = useNuxtApp();
let route = useRoute();

let ory_ui_msgs = ref( [
        { text: 'Please verification code!' },
]);
let code = ref("");

// On load, do a quick check for status of the verification flow
// If they clicked on the email link and the flow is still the same, the flow.data.ui object
// will contain 'code' amongst its UI nodes with the verification code- which ideally can be put automatically
// into the field so they can just verify it and continue.
$oryConfig.getVerificationFlow({id:route.query.flow}).then(
  flow => {
    let returned_nodes = flow.data.ui.nodes;
    for (let i = 0; i < returned_nodes.length; i++){
      if (returned_nodes[i].group=="code" && returned_nodes[i].attributes.name=="code"){
        code.value=returned_nodes[i].attributes.value;
        break;
      }
    }
  }
)

// Attempts to verify an account with the given 'code' (sent to an email with the registration flow)
async function verification() {
  // Get recovery flow object from flow id parameter 
  let flow_data = await $oryConfig.getVerificationFlow({id:route.query.flow})

  // Directly extract csrf_token from nested returned Ory UI elements
  let csrf_token = extract_nested_csrf_token(flow_data.data);

  // updateVerificationFlow, submitting and checking with verification code
  await $oryConfig.updateVerificationFlow({
    flow: route.query.flow,
    updateVerificationFlowBody: {
      csrf_token: csrf_token, // must be directly set
      code: code.value,
      method: "code",
  }
  }).then( r => {
      console.log("Successfully verified account!");
      ory_ui_msgs.value = [{text: "Email successfully verified."}]
      // Success! We can maybe then direct them to the login page? 
  }
  ).catch(e => {
    // Get displayable error messsages from nested returned Ory UI elements
    ory_ui_msgs.value = extract_nested_error_messages_from_error(e);
  });
}
</script>
