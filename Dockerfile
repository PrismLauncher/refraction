FROM docker.io/library/node:20-alpine
RUN corepack enable
RUN corepack prepare pnpm@latest --activate

WORKDIR /app

COPY package.json pnpm-lock.yaml .
RUN pnpm install

COPY . .
CMD [ "pnpm", "run", "start" ]
