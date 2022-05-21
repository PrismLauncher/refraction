FROM node:17-alpine as build-image
WORKDIR /app
COPY package.json yarn.lock .
RUN yarn install --frozen-lockfile
COPY . .
RUN yarn build

FROM node:17-alpine
WORKDIR /app
COPY --from=build-image /app/index.js /app/index.js
EXPOSE 3000
CMD [ "node", "index.js" ]
