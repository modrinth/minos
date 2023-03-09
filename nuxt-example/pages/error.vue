<template>
  <div id="error">
      {{ ory_ui_error }}
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
  name: "error",
  data() {
    return {
      ory_ui_error: "Loading error...",
    };
  },
  mounted() {
      
  ory.getFlowError({id:this.$route.query.id}).then(r => {
    this.ory_ui_error =r.data.error  // Just for example here, display entire json in above variable
  });

  }
};
</script>
