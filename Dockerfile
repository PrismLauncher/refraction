FROM docker.io/library/node:17-alpine
WORKDIR /app
COPY . .
RUN yarn install --frozen-lockfile
CMD [ "yarn", "start" ]
