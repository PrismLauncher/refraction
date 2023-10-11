export default {
  discord: {
    clientId: process.env.DISCORD_CLIENT_ID || '',
    clientSecret: process.env.DISCORD_CLIENT_SECRET || '',
    botToken: process.env.DISCORD_BOT_TOKEN || '',
    oauth2: {
      baseUrl: 'https://discord.com/api/oauth2/',
      redirectUri: `${process.env.PUBLIC_URI}/oauth2/callback`,
      scope: 'connections role_connections.write',
    },
    channels: {
      sayLogChannelId: process.env.DISCORD_SAY_LOG_CHANNELID || '',
    },
  },
  github: {
    repoRoleMapping: {
      'PrismLauncher/PrismLauncher': {
        key: 'launcher',
        name: 'Launcher contributor',
      },
      'PrismLauncher/prismlauncher.org': {
        key: 'website',
        name: 'Web developer',
      },
      'PrismLauncher/Translations': {
        key: 'translations',
        name: 'Translator',
      },
    },
  },
  expressPort: Number(process.env.EXPRESS_PORT) || 3000,
};
