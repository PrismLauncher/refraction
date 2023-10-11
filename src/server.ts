import {
  REST,
  Routes,
  OAuth2Routes,
  type RESTPutAPICurrentUserApplicationRoleConnectionJSONBody,
  type RESTGetAPICurrentUserConnectionsResult,
  type RESTGetAPIUserResult,
  ConnectionService,
} from 'discord.js';

import { Octokit } from '@octokit/core';
import { paginateRest } from '@octokit/plugin-paginate-rest';
import { throttling } from '@octokit/plugin-throttling';
import { retry } from '@octokit/plugin-retry';

import axios, { AxiosError } from 'axios';
import Fastify, { RequestGenericInterface } from 'fastify';

import {
  areContributors,
  contributorsStored,
  storeGitHubContributors,
  storeToken,
} from './storage';
import config from './config';

interface OAuth2Callback extends RequestGenericInterface {
  Querystring: {
    code: string;
  };
}

const MyOctokit = Octokit.plugin(paginateRest, throttling, retry);

const octokit = new MyOctokit({
  auth: config.github.token,
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
  try {
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
  } catch (e) {
    if (!(e instanceof AxiosError) || e.status != 400) throw e;
    return null;
  }
};

const getDiscordProfile = async (rest: REST) => {
  return (await rest.get(Routes.user())) as RESTGetAPIUserResult;
};

const getGitHubConnections = async (rest: REST) => {
  const connections = (await rest.get(
    Routes.userConnections()
  )) as RESTGetAPICurrentUserConnectionsResult;

  return connections.reduce((result, connection) => {
    if (connection.type == ConnectionService.GitHub) result.push(connection.id);
    return result;
  }, [] as string[]);
};

const getGitHubContributorIds = async (owner: string, repo: string) => {
  return (
    await octokit.paginate('GET /repos/{owner}/{repo}/contributors', {
      owner,
      repo,
    })
  ).reduce((result, contributor) => {
    if (contributor.id) result.push(contributor.id.toString());
    return result;
  }, [] as string[]);
};

export const listen = () => {
  const fastify = Fastify({ logger: true });

  fastify.get('/oauth2/authorize', (_, reply) => {
    reply.redirect(generateAuthorizeUrl());
  });

  fastify.get<OAuth2Callback>('/oauth2/callback', async (request, reply) => {
    const { code } = request.query;

    if (!code) {
      reply.code(400);
      return;
    }

    const tokenResponse = await getTokensFromOAuth(code.toString());

    if (!tokenResponse) {
      reply
        .code(400)
        .send(
          'The authorization code is invalid. Please restart the authorization process.'
        );
      return;
    }

    const accessToken = tokenResponse.data.access_token;
    const refreshToken = tokenResponse.data.refresh_token;

    const userRest = makeRestAPI(accessToken);
    const discordUserId = (await getDiscordProfile(userRest)).id;

    storeToken(discordUserId, accessToken, refreshToken);

    const githubUserIds = await getGitHubConnections(userRest);

    const metadata: RESTPutAPICurrentUserApplicationRoleConnectionJSONBody = {
      metadata: {},
    };

    for (const repo of config.github.repos) {
      const key = `contributed_${repo.key}`;
      metadata.metadata![key] = 'false';

      if (!(await contributorsStored(repo.owner, repo.repo))) {
        request.log.debug(
          `Cache miss for ${repo.owner}/${repo.repo} contributors`
        );
        const contributors = await getGitHubContributorIds(
          repo.owner,
          repo.repo
        );
        await storeGitHubContributors(repo.owner, repo.repo, contributors);
      }

      if (await areContributors(repo.owner, repo.repo, githubUserIds)) {
        metadata.metadata![key] = 'true';
      }
    }

    // potentially add platform_name and platform_username

    await userRest.put(
      Routes.userApplicationRoleConnection(config.discord.clientId),
      {
        body: metadata,
      }
    );

    reply
      .code(200)
      .send(
        'You should have your linked roles now! You can close this page now.'
      );
  });
  fastify.listen({ port: config.expressPort });
};
