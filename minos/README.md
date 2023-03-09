# Minos 

An Actix server that allows authentication through the Ory backend in Rust using easily-accessed endpoints.

Currently it only has the demo endpoint `GET /demo` which returns 200 if and only if:

- CORS checks pass.

- Cookie with Ory session identifier (from the same domain, such as the Nuxt example running) is attached as a header.

If it passes, it returns the Ory sessoin as a JSON. Otherwise, it returns 401.

# TL;DR setup:

The easiest/clearest way to set up is using the Ory-hosted backend.

1. Install the Ory CLI depending on your OS [from the link here](https://www.ory.sh/docs/guides/cli/installation)
2. In this directory, run `npm install` (with a recent version of node being used).
3. Run `ory tunnel http://localhost:4455 --project ecstatic-lehmann-onx4dw646f --port 4433 --dev` to access our Ory project from `localhost:4433`. If it asks you to log in, you do not need to for this purpose. (If the tunnel already running, you don't have to run it again).
4. In this folder (in another window), `cargo run` to start the server.

# Ory Account

- I currently have an Ory account designed for use with the local flow shown above (with the tunnel and so on). I can add emails so we can collaborate on the same one (both for testing and if we decide to use this method). 

- the account project is ecstatic-lehmann-onx4dw646f (but we can make a new one, its easy to swap, just change the slug in the 'tunnel' command above)


# Setup - self-hosted

Alternatively, we don't need to use the Ory-hosted backend, we can host it ourselves.

[See the Nuxt example self-hosted instructions here](../nuxt-example/README.md#setup---self-hosted).

