import {
  REST,
  Routes,
  OAuth2Routes,
  type RESTPutAPICurrentUserApplicationRoleConnectionJSONBody,
} from 'discord.js';

import axios from 'axios';
import express from 'express';

import config from './config';

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

    // Get GitHub connections
    // Ask GitHub for contributions

    // potentially add platform_name and platform_username
    const metadata: RESTPutAPICurrentUserApplicationRoleConnectionJSONBody = {
      metadata: {
        contributed_launcher: 'true',
        contributed_translations: 'true',
      },
    };

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
