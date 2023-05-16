# Minos 

Modrinth authentication backend using Ory Kratos

Includes:
- a small Actix server that demos an authentication process in a middleware (Minos)
- a Nuxt server that goes through the Ory login flow with Modrinth UI

Accessing the endpoints `GET /user` and `GET /user/session` will provide authentication information if request has an Ory cookie attached.  

Uses a different database on the labrinth DB instance- `sqlx database create` must be likewise run here to create the `minos` DB.

# Ory

Read more about Ory Kratos [here](https://www.ory.sh/docs/kratos/quickstart):

# Ports:

- **4000: Minos Actix server.** (staging-minos.modrinth.com)
- **4433: Kratos API** (staging-kratos.modrinth.com)
- 4434: Kratos admin API
- **4436: Mailslurper ports (for demoing email)**
- 4437:4437: Mailslurper ports (for demoing email)
- **4455: Nuxt 3 webserver.** (staging-auth.modrinth.com)
- 24678:24678: Nuxt communication port
- 24679:24679: Nuxt communication port
