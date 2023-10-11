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

const makeRestAPI = (token = config.discord.botToken, bot = true) => {
  return new REST({
    version: '10',
    authPrefix: bot ? 'Bot' : 'Bearer',
  }).setToken(token);
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

    const tokenResponse = await axios.post(
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

    // TODO: store tokens in Redis

    const rest = makeRestAPI(tokenResponse.data.access_token, false);

    const connections = (await rest.get(
      Routes.userConnections()
    )) as RESTGetAPICurrentUserConnectionsResult;
    const githubUserIds = connections
      .filter((connection) => connection.type == ConnectionService.GitHub)
      .map((connection) => connection.id);

    // Ask GitHub for contributions
    const metadata: RESTPutAPICurrentUserApplicationRoleConnectionJSONBody = {
      metadata: {
        contributed_launcher: 'true',
        contributed_translations: 'true',
      },
    };

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

    for (const repo of config.github.repos) {
      const key = `contributed_${repo.key}`;
      metadata.metadata![key] = 'false';

      await octokit.paginate(
        'GET /repos/{owner}/{repo}/contributors',
        { owner: repo.owner, repo: repo.repo },
        (response, done) => {
          if (
            response.data.find((contributor) =>
              githubUserIds.includes(contributor.id!.toString())
            )
          ) {
            done();
            metadata.metadata![key] = 'true';
          }
          return response.data;
        }
      );
    }

    // potentially add platform_name and platform_username

    const discordResponse = await rest.put(
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
