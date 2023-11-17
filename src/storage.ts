import { createClient } from 'redis';

export const client = createClient({
  url: process.env.REDIS_URL || 'redis://localhost:6379',
});

export const storeUserPlurality = async (userId: string) => {
  // Just store some value. We only care about the presence of this key
  await client
    .multi()
    .set(`user:${userId}:pk`, '0')
    .expire(`user:${userId}:pk`, 7 * 24 * 60 * 60)
    .exec();
};

export const isUserPlural = async (userId: string) => {
  return (await client.exists(`user:${userId}:pk`)) > 0;
};

export const connect = () => {
  client.connect();
};
