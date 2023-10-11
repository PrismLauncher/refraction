import { createClient } from 'redis';

import config from './config';

const client = createClient({
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
    .expire(key, 300, 'NX')
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

export const connect = () => {
  client.connect();
};
