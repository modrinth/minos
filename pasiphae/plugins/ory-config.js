import { FrontendApi, Configuration } from '@ory/client'

export default defineNuxtPlugin((_nuxtApp) => {
  const config = useRuntimeConfig()
  return {
    provide: {
      oryConfig: new FrontendApi(
        new Configuration({
          basePath: config.oryUrl,
          baseOptions: {
            withCredentials: true, // Ensures we send cookies in the CORS requests.
          },
        })
      ),
    },
  }
})
