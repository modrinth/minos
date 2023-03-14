<template>
  <div id="recovery">
    <form @submit.prevent="recovery">
      <input v-model="email" placeholder="email">
      <input type="submit" value="send email recovery link">
    </form>
    <form @submit.prevent="submitCode">
      <input v-model="code" placeholder="code">
      <input type="submit" value="recover using code and email">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

  </div>
</template>

<script setup>
import { extract_nested_csrf_token} from "~/helpers/ory-ui-extract"
import oryConfig from "~~/plugins/ory-config";
login_flow_endpoint
let config = useRuntimeConfig();
let { $oryConfig } = useNuxtApp();
let route = useRoute();

let ory_ui_msgs = ref( [
        { text: 'Attempt to recover your account!' },
]);
let email = ref("");
let code = ref("");


// Send recovery email to the set 'email'
async function recovery() {
  // Get recovery flow object from flow id parameter 
  let flow_data = await $oryConfig.getRecoveryFlow({id:route.query.flow})

  // Directly extract csrf_token from nested returned Ory UI elements
  let csrf_token = extract_nested_csrf_token(flow_data.data);

  // updateRecoveryFlow, which will send an code+link to the provided email
  await $oryConfig.updateRecoveryFlow({
    flow: route.query.flow,
    updateRecoveryFlowBody: {
      csrf_token: csrf_token, // must be directly set
      email: email.value, // MUST be an email identifier, not just a usernmae
      method: "code",
  }
  }).then( r => {
      console.log("Successful recovery - part 1");
      ory_ui_msgs.value = [{text: "Sent recovery email."}]
  }
  ).catch(e => {
    // Get displayable error messsages from nested returned Ory UI elements
    ory_ui_msgs.value = extract_nested_error_messages_from_error(e);
  });
}

// Attempts to recover an account with the given 'email' and 'code' (sent to an email with the recovery() function)
async function submitCode() {
  // Get recovery flow object from flow id parameter 
  let flow_data = await $oryConfig.getRecoveryFlow({id:this.route.query.flow})

  // Directly extract csrf_token from nested returned Ory UI elements
  let csrf_token = extract_nested_csrf_token(flow_data.data);

  // updateRecoveryFlow, but pass the 'code' field to attempt to recover using that code
  await $oryConfig.updateRecoveryFlow({
    flow: this.route.query.flow,
    updateRecoveryFlowBody: {
      csrf_token: csrf_token, // must be directly set
      email: email.value, // MUST be an email identifier, not just a usernmae
      method: "code",
      code: code.value
  }
  }).then( r => {
      console.log("Successful recovery!");
      // If return_to exists, return to it, otherwise return to main page
      let return_url = flow_data.data.return_to || nuxtUrl;
      window.location.href = return_url;
  }

  ).catch(e => {
    // Get displayable error messsages from nested returned Ory UI elements
    ory_ui_msgs.value = extract_nested_error_messages_from_error(e);
  });
}
</script>
