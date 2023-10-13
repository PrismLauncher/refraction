import {
  REST,
  Routes,
  OAuth2Routes,
  type RESTPutAPICurrentUserApplicationRoleConnectionJSONBody,
  type RESTGetAPICurrentUserConnectionsResult,
  type RESTGetAPIUserResult,
  ConnectionService,
  RESTPostOAuth2AccessTokenResult,
} from 'discord.js';

import Fastify, { RequestGenericInterface } from 'fastify';

import { areContributors, contributorsStored, storeToken } from './storage';
import config from './config';

interface OAuth2Callback extends RequestGenericInterface {
  Querystring: {
    code: string;
  };
}

const makeRestAPI = (accessToken: string | undefined) => {
  const rest = new REST({
    version: '10',
    authPrefix: 'Bearer',
  });
  if (accessToken) rest.setToken(accessToken);
  return rest;
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

const getTokensFromOAuth = async (rest: REST, code: string) => {
  return (await rest.post(Routes.oauth2TokenExchange(), {
    auth: false,
    body: new URLSearchParams({
      client_id: config.discord.clientId,
      client_secret: config.discord.clientSecret,
      code,
      grant_type: 'authorization_code',
      redirect_uri: config.discord.oauth2.redirectUri,
      scope: config.discord.oauth2.scope,
    }),
    passThroughBody: true,
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
  })) as RESTPostOAuth2AccessTokenResult;
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

export const listen = () => {
  const fastify = Fastify({ logger: true });

  fastify.get('/oauth2/authorize', (_, reply) => {
    reply.redirect(generateAuthorizeUrl());
  });

  fastify.get<OAuth2Callback>('/oauth2/callback', async (request, reply) => {
    const { code } = request.query;

    if (!code) {
      return reply.code(400);
    }

    const anonRest = makeRestAPI(undefined);

    const tokenResponse = await getTokensFromOAuth(anonRest, code.toString());

    if (!tokenResponse) {
      return reply
        .code(400)
        .send(
          'The authorization code is invalid. Please restart the authorization process.'
        );
    }

    const userRest = makeRestAPI(tokenResponse.access_token);
    const discordUserId = (await getDiscordProfile(userRest)).id;

    storeToken(
      discordUserId,
      tokenResponse.access_token,
      tokenResponse.refresh_token
    );

    const githubUserIds = await getGitHubConnections(userRest);

    const metadata: RESTPutAPICurrentUserApplicationRoleConnectionJSONBody = {
      metadata: {},
    };

    for (const repo of config.github.repos) {
      const key = `contributed_${repo.key}`;
      metadata.metadata![key] = 'false';

      if (!(await contributorsStored(repo.owner, repo.repo))) {
        return reply
          .code(500)
          .send(
            "We don't have data about GitHub contributors right now. Yell at @scrumplex if you see this."
          );
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

    return reply
      .code(200)
      .send(
        'You should have your linked roles now! You can close this page now.'
      );
  });
  fastify.listen({ port: config.httpPort });
};
