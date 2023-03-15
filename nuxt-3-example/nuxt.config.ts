// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  runtimeConfig: {
    public: {
      oryUrl: process.env.NUXT_ENV_ORY_URL || 'http://localhost:4433',
      minosUrl: process.env.NUXT_ENV_MINOS_URL || 'http://localhost:4000',
      nuxtUrl: process.env.NUXT_ENV_API_URL || 'http://localhost:4455',
    },
  },
  plugins: ['@/plugins/ory-config.js'],
})
