import { createClient } from 'redis';

import config from './config';

export const client = createClient({
  url: config.redisUrl,
});

const githubContributionsKey = (owner: string, repo: string) =>
  `github-contributions:${owner}:${repo}`;

export const storeToken = async (
  userId: string,
  accessToken: string,
  refreshToken: string
) => {
  await client.hSet(`user-tokens:${userId}`, { accessToken, refreshToken });
};

export const storeGitHubContributors = async (
  owner: string,
  repo: string,
  contributorIds: string[]
) => {
  const key = githubContributionsKey(owner, repo);
  await client
    .multi()
    .del(key)
    .sAdd(key, contributorIds)
    .expire(key, config.github.cacheSec, 'NX')
    .exec();
};

export const contributorsStored = async (owner: string, repo: string) => {
  const key = githubContributionsKey(owner, repo);
  return (await client.exists(key)) >= 1;
};

export const areContributors = async (
  owner: string,
  repo: string,
  contributorIds: string[]
) => {
  const key = githubContributionsKey(owner, repo);
  return (await client.smIsMember(key, contributorIds)).includes(true);
};

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
