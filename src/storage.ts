import { createClient } from 'redis';

import config from './config';

const client = createClient({
  url: config.redisUrl,
});

export const storeToken = async (
  userId: string,
  accessToken: string,
  refreshToken: string
) => {
  await client.hSet(`user-tokens:${userId}`, { accessToken, refreshToken });
};

export const connect = () => {
  client.connect();
};
