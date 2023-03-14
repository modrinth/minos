<template>
  <div id="settings">
    <form @submit.prevent="updatePassword">
      <input v-model="password" placeholder="password" type="password">
      <input type="submit" value="change">
    </form>

    <li v-for="ory_ui_msg in ory_ui_msgs">
      {{ ory_ui_msg.text }}
    </li>

  </div>
</template>

<script setup>

import { extract_nested_csrf_token} from "~/helpers/ory-ui-extract"

let config = useRuntimeConfig();
let route = useRoute();
let { $oryConfig } = useNuxtApp();

let ory_ui_msgs = ref( [
        { text: 'Attempt to change your password!' },
]);
let password = ref("");


// Uses settings flow to update a logged-in user's password
async function updatePassword() {
  // Get settings flow object from flow id parameter 
  let flow_data = await $oryConfig.getSettingsFlow({id:route.query.flow})

  // Directly extract csrf_token from nested returned Ory UI elements
  let csrf_token = extract_nested_csrf_token(flow_data.data);

  // updateSettingsFlow can match one of:
  // UpdateSettingsFlowWithLookupMethod | UpdateSettingsFlowWithOidcMethod | UpdateSettingsFlowWithPasswordMethod | UpdateSettingsFlowWithProfileMethod
  // For different ways to things to change - some lookup value, or password.
  // In this case, we use UpdateSettingsFlowWithPasswordMethod to update password
  await $oryConfig.updateSettingsFlow({
    flow: route.query.flow,
    updateSettingsFlowBody : {
      csrf_token: csrf_token, // must be directly set
      method: "password",
      password: password.value,
    },
  }).then( r => {
      console.log("Successful pass change.");
      ory_ui_msgs.value = [{text: "Successful pass change."}]
  }
  ).catch(e => {
    // Get displayable error messsages from nested returned Ory UI elements
    ory_ui_msgs.value = extract_nested_error_messages_from_error(e);
  });
}
</script>
