# Minos (nuxt-example)

A Nuxt example that goes through the Ory login flow with customizable UI - (There is a version for using Ory accounts, and a not-quite-finished version for hosting the Ory framework ourselves)

# Setup

To setup the Nuxt.js server, do the following:

1. In this directory, run `npm install` (with a recent version of node being used).
2. In this folder (in another window), `yarn dev` to start Nuxt
3. Go to `localhost:4455`, and test!

You will need to do the instructions [here](../README.md#setup) to have Kratos/Ory working as well (if you are running the frontend this way, use `docker-compose-kratos-only.yml`)

# Ory-hosted setup:

To setup using the Ory-hosted backend and this repo:.

1. Create an Ory account and get access to the console + project.
2. Install the Ory CLI depending on your OS [from the link here](https://www.ory.sh/docs/guides/cli/installation)
3. In this directory, run `npm install` (with a recent version of node being used).
4. Run `ory tunnel http://localhost:4455 --project ecstatic-lehmann-onx4dw646f --port 4433 --dev` to access our Ory project from `localhost:4433`.
5. Ory tunnel ask you to log in on the CLI using ory tunnel to configure the project. This is needed for testing passing cookies from social logins (ie: login with ithub).
6. In this folder (in another window), `yarn dev` to start Nuxt
7. Go to `localhost:4455`, and test!

Note: we are not using this method, but are doing the self-hosted method. The Ory-hosted version is useful for configuration and testing, though.

# Ory Account

- I currently have an Ory account designed for use with the local flow shown above (with the tunnel and so on). I can add emails so we can collaborate on the same one (both for testing and if we decide to use this method).

- the account project is ecstatic-lehmann-onx4dw646f (but we can make a new one, its easy to swap, just change the slug in the 'tunnel' command above)
