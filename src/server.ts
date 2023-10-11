import {
  REST,
  Routes,
  OAuth2Routes,
  type RESTPutAPICurrentUserApplicationRoleConnectionJSONBody,
  type RESTGetAPICurrentUserConnectionsResult,
  ConnectionService,
} from 'discord.js';

import { Octokit } from '@octokit/core';
import { paginateRest } from '@octokit/plugin-paginate-rest';
import { throttling } from '@octokit/plugin-throttling';
import { retry } from '@octokit/plugin-retry';

import axios from 'axios';
import express from 'express';

import config from './config';

const MyOctokit = Octokit.plugin(paginateRest, throttling, retry);

const octokit = new MyOctokit({
  throttle: {
    onRateLimit: (retryAfter, options) => {
      octokit.log.warn(
        `Request quota exhausted for request ${options.method} ${options.url}`
      );

      // Retry twice after hitting a rate limit error, then give up
      if (options.request.retryCount <= 2) {
        console.log(`Retrying after ${retryAfter} seconds!`);
        return true;
      }
    },
    onSecondaryRateLimit: (retryAfter, options, octokit) => {
      // does not retry, only logs a warning
      octokit.log.warn(
        `Secondary quota detected for request ${options.method} ${options.url}`
      );
    },
  },
});

const makeRestAPI = (accessToken: string) => {
  return new REST({
    version: '10',
    authPrefix: 'Bearer',
  }).setToken(accessToken);
};

// TODO: add state param
const generateAuthorizeUrl = () => {
  const url = new URL(OAuth2Routes.authorizationURL);
  url.searchParams.append('client_id', config.discord.clientId);
  url.searchParams.append('redirect_uri', config.discord.oauth2.redirectUri);
  url.searchParams.append('response_type', 'code');
  url.searchParams.append('scope', config.discord.oauth2.scope);
  return url.toString();
};

const getTokensFromOAuth = async (code: string) => {
  return await axios.post(
    OAuth2Routes.tokenURL,
    {
      client_id: config.discord.clientId,
      client_secret: config.discord.clientSecret,
      code,
      grant_type: 'authorization_code',
      redirect_uri: config.discord.oauth2.redirectUri,
      scope: config.discord.oauth2.scope,
    },
    { headers: { 'Content-Type': 'application/x-www-form-urlencoded' } }
  );
};

const getGitHubConnections = async (rest: REST) => {
  const connections = (await rest.get(
    Routes.userConnections()
  )) as RESTGetAPICurrentUserConnectionsResult;

  return connections
    .filter((connection) => connection.type == ConnectionService.GitHub)
    .map((connection) => connection.id);
};

const getGitHubContributors = async (owner: string, repo: string) => {
  return await octokit.paginate('GET /repos/{owner}/{repo}/contributors', {
    owner,
    repo,
  });
};

export const listenApp = () => {
  const app = express();

  app.get('/oauth2/authorize', (_, response) => {
    response.redirect(generateAuthorizeUrl());
  });

  app.get('/oauth2/callback', async (request, response) => {
    const { code } = request.query;

    if (!code) {
      response.sendStatus(400);
      return;
    }

    const tokenResponse = await getTokensFromOAuth(code.toString());

    // TODO: store tokens in Redis

    const userRest = makeRestAPI(tokenResponse.data.access_token);
    const githubUserIds = await getGitHubConnections(userRest);

    const metadata: RESTPutAPICurrentUserApplicationRoleConnectionJSONBody = {
      metadata: {},
    };

    for (const repo of config.github.repos) {
      const key = `contributed_${repo.key}`;
      metadata.metadata![key] = 'false';

      const contributors = await getGitHubContributors(repo.owner, repo.repo);

      if (
        contributors.find((contributor) =>
          githubUserIds.includes(contributor.id!.toString())
        )
      ) {
        metadata.metadata![key] = 'true';
      }
    }

    // potentially add platform_name and platform_username

    const discordResponse = await userRest.put(
      Routes.userApplicationRoleConnection(config.discord.clientId),
      {
        body: metadata,
      }
    );

    console.log(discordResponse);
    response.sendStatus(200);
  });
  app.listen(config.expressPort);
};
