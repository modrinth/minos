# Minos (nuxt-example)

A Nuxt example that goes through the Ory login flow with customizable UI
	- (There is a version for using Ory accounts, and a not-quite-finished version for hosting the Ory framework ourselves) 

# TL;DR setup:

The easiest/clearest way to set up is using the Ory-hosted backend and the Nuxt.js frontend that is in this repo.

Several optional steps are listed here: these steps are not required to have a working demo site with working login flow, but are required to test external social logins (ie: 'log in with Github!')

1. (Optional) Create an Ory account and get access to the console + project. 
1. Install the Ory CLI depending on your OS [from the link here](https://www.ory.sh/docs/guides/cli/installation)
2. In this directory, run `npm install` (with a recent version of node being used).
3. Run `ory tunnel http://localhost:4455 --project ecstatic-lehmann-onx4dw646f --port 4433 --dev` to access our Ory project from `localhost:4433`. 
4. (Optional) Ory tunnel ask you to log in on the CLI using ory tunnel to configure the project. This is needed for testing passing cookies from social logins (ie: login with Github).
4. In this folder (in another window), `yarn dev` to start Nuxt
5. Go to `localhost:4455`


# Ory Account

- I currently have an Ory account designed for use with the local flow shown above (with the tunnel and so on). I can add emails so we can collaborate on the same one (both for testing and if we decide to use this method). 

- the account project is ecstatic-lehmann-onx4dw646f (but we can make a new one, its easy to swap, just change the slug in the 'tunnel' command above)

# Setup - self-hosted

This one will take more work to slot up to our custom UI, but we can choose to host the Ory flow ourselves (locally in this case) on a local docker instance. For testing (ie: auth, UI design, endpoint design), we probably don't need to worry about this yet.

1. Clone the Ory directory and set up a docker instance with it.
<code>
git clone https://github.com/ory/kratos.git<br>
cd kratos<br>
git checkout v0.11.1<br>
docker-compose -f quickstart.yml -f quickstart-standalone.yml up --build --force-recreate
</code>

2. When it finishes, and starts running, you should be able to connect to the <i>default ORY frontend they provide</i> at ```127.0.0.1:4455```. 

3. You can kill the process and restart the docker with: ```docker-compose -f quickstart.yml -f quickstart-standalone.yml up```

You are done. If you want to change the Ory backend to use the Nuxt frontend and not their default one, run following steps:

3. Move the ```modrinth-quickstart.yml``` to the kratos folder, and then  ```docker-compose -f quickstart.yml -f quickstart-standalone.yml up```.

(All this does right now is simply disable the ory self-service example, but will likely need to set it up to include running the Nuxt as well, see below).

4. Ensure all test env variables and addresses are using 127.0.0.1 and not localhost- the default for Kratos internally.

4. Run it! <b>Warning</b>: this will not allow requests through all the way yet. See notes at the bottom. 

# Notes

- The recovery flow is incomplete for the Ory-hosted version, as there is currently no setup SMTP response server there. (There is a dummy SMTP server on the selfhosted verision)

- The self-hosted version with our UI does not quite work yet, even when all addresses are 127.0.0.1. It can be started but gives CORS errors (I suspect because ports are different and because our UI is not on docker-compose intranet with the other services).