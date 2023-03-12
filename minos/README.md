# Minos 

An Actix server that allows authentication through the Ory backend in Rust using easily-accessed endpoints.

Currently it only has the demo endpoint `GET /demo` which returns 200 if and only if:

- CORS checks pass.

- Cookie with Ory session identifier (from the same domain, such as the Nuxt example running) is attached as a header.

If it passes, it returns the Ory sessoin as a JSON. Otherwise, it returns 401.

# TL;DR setup:

The easiest/clearest way to set up is using the Ory-hosted backend.

1. [Follow the Nuxt setup instructions here]([See the Nuxt example self-hosted instructions here](../nuxt-example/README.md#tldr-setup)) to set up the site and create a tunnel to the Ory console.
3. In this folder (in another window), `cargo run` to start the server.

# Ory Account

- I currently have an Ory account designed for use with the local flow shown above (with the tunnel and so on). I can add emails so we can collaborate on the same one (both for testing and if we decide to use this method). 

- the account project is ecstatic-lehmann-onx4dw646f (but we can make a new one, its easy to swap, just change the slug in the 'tunnel' command above)


# Setup - self-hosted

Alternatively, we don't need to use the Ory-hosted backend, we can host it ourselves.

[See the Nuxt example self-hosted instructions here](../nuxt-example/README.md#setup---self-hosted).

