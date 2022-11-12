import { createClient } from 'redis';

const ONE_MONTH_IN_SECONDS = 2629746;

const client = createClient({ url: process.env.REDIS_URI });

export const get = async (key: string) => {
  await client.connect();
  const data = await client.get(key);
  await client.disconnect();
  return data;
};

export const set = async (key: string, value: string, autoExpire = false) => {
  await client.connect();
  await client.set(key, value);
  if (autoExpire && (await client.ttl(key)) === -1) {
    await client.expire(key, ONE_MONTH_IN_SECONDS);
  }
  await client.disconnect();
};

export const incr = async (key: string, autoExpire = false) => {
  await client.connect();

  await client.incr(key);
  if (autoExpire && (await client.ttl(key)) === -1) {
    await client.expire(key, ONE_MONTH_IN_SECONDS);
  }

  await client.disconnect();
};
