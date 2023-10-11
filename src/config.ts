export default {
  discord: {
    clientId: process.env.DISCORD_CLIENT_ID || '',
    clientSecret: process.env.DISCORD_CLIENT_SECRET || '',
    botToken: process.env.DISCORD_BOT_TOKEN || '',
    oauth2: {
      redirectUri: `${process.env.PUBLIC_URI}/oauth2/callback`,
      scope: 'connections role_connections.write',
    },
    channels: {
      sayLogChannelId: process.env.DISCORD_SAY_LOG_CHANNELID || '',
    },
  },
  github: {
    repos: [
      {
        owner: 'PrismLauncher',
        repo: 'PrismLauncher',

        key: 'launcher',
        name: 'Launcher contributor',
      },
      {
        owner: 'PrismLauncher',
        repo: 'prismlauncher.org',

        key: 'website',
        name: 'Web developer',
      },
      {
        owner: 'PrismLauncher',
        repo: 'Translations',

        key: 'translations',
        name: 'Translator',
      },
    ],
  },
  expressPort: Number(process.env.EXPRESS_PORT) || 3000,
};
