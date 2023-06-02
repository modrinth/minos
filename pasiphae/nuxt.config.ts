import svgLoader from 'vite-svg-loader'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  runtimeConfig: {
    public: {
      oryUrl: process.env.NUXT_ENV_ORY_URL || 'http://127.0.0.1:4433',
      nuxtUrl: process.env.NUXT_ENV_API_URL || 'http://127.0.0.1:4455',
    },
  },
  plugins: ['@/plugins/ory-config.js'],
  vite: {
    plugins: [
      svgLoader({
        svgoConfig: {
          plugins: [
            {
              name: 'preset-default',
              params: {
                overrides: {
                  removeViewBox: false,
                },
              },
            },
          ],
        },
      }),
    ],
  },
})
