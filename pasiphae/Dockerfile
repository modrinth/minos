# use node 16 alpine image
FROM node:16-alpine

# create work directory in app folder
WORKDIR /app

# install required packages for node image
RUN apk --no-cache add openssh g++ make python3 git

# copy over package.json files
COPY package.json /app/
COPY yarn.lock /app/

# install all depencies
RUN yarn install

# copy over all files to the work directory
ADD . /app

# Copy the configuration files into the image
COPY minos-configs/ /etc/config/kratos/

# build the project
RUN yarn run build

# run the build project with node
ENTRYPOINT ["node", ".output/server/index.mjs"]
