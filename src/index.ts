import {
  Client,
  GatewayIntentBits,
  Partials,
  OAuth2Scopes,
  InteractionType,
} from 'discord.js';
import { reuploadCommands } from './_reupload';

import * as BuildConfig from './constants';
import { parseLog } from './logs';
import { getLatestMinecraftVersion } from './utils/remoteVersions';

import { membersCommand } from './commands/members';
import { starsCommand } from './commands/stars';
import { modrinthCommand } from './commands/modrinth';
import { tagsCommand } from './commands/tags';
import { jokeCommand } from './commands/joke';

import random from 'just-random';
import { green, bold, yellow } from 'kleur/colors';
import 'dotenv/config';

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
    GatewayIntentBits.DirectMessages,
    GatewayIntentBits.GuildMembers,
    GatewayIntentBits.GuildPresences,
    GatewayIntentBits.GuildMessageReactions,
    GatewayIntentBits.GuildBans,
  ],
  partials: [Partials.Channel],
});

client.once('ready', async () => {
  console.log(green('Discord bot ready!'));

  console.log(
    client.generateInvite({
      scopes: [OAuth2Scopes.Bot],
      permissions: [
        'AddReactions',
        'ViewChannel',
        'BanMembers',
        'KickMembers',
        'CreatePublicThreads',
        'CreatePrivateThreads',
        'EmbedLinks',
        'ManageChannels',
        'ManageRoles',
        'ModerateMembers',
        'MentionEveryone',
        'MuteMembers',
        'SendMessages',
        'SendMessagesInThreads',
        'ReadMessageHistory',
      ],
    })
  );

  if (process.env.NODE_ENV !== 'development')
    console.warn(yellow(bold('Running in production mode!')));

  const mcVersion = await getLatestMinecraftVersion();
  client.user?.presence.set({
    activities: [{ name: `Minecraft ${mcVersion}` }],
    status: 'online',
  });

  client.on('messageCreate', async (e) => {
    if (!e.content) return;
    if (!e.channel.isTextBased()) return;

    if (e.author === client.user) return;

    if (e.cleanContent.match(BuildConfig.ETA_REGEX)) {
      await e.reply(
        `${random(BuildConfig.ETA_MESSAGES)} <:pofat:964546613194420294>`
      );
    }

    const log = await parseLog(e.content);
    if (log != null) {
      e.reply({ embeds: [log] });
      return;
    }
  });
});

client.on('interactionCreate', async (interaction) => {
  if (!interaction.isChatInputCommand()) return;

  const { commandName } = interaction;

  if (commandName === 'ping') {
    await interaction.reply({
      content: `Pong! \`${client.ws.ping}ms\``,
      ephemeral: true,
    });
  } else if (commandName === 'members') {
    await membersCommand(interaction);
  } else if (commandName === 'stars') {
    await starsCommand(interaction);
  } else if (commandName === 'modrinth') {
    await modrinthCommand(interaction);
  } else if (commandName === 'rolypoly') {
    await interaction.reply(
      'https://media.discordapp.net/attachments/985048903126769764/985051373886382100/rollin-time.gif?width=324&height=216'
    );
  } else if (commandName === 'say') {
    if (!interaction.channel) return;

    await interaction.deferReply({ ephemeral: true });
    await interaction.channel.send(interaction.options.getString('content')!);
    await interaction.editReply('I said what you said!');
  } else if (commandName === 'tag') {
    await tagsCommand(interaction);
  } else if (commandName === 'joke') {
    await jokeCommand(interaction);
  }
});

client.on('messageReactionAdd', async (reaction, user) => {
  if (reaction.partial) {
    try {
      await reaction.fetch();
    } catch (error) {
      console.error('Something went wrong when fetching the message:', error);
      return;
    }
  }

  if (
    reaction.message.interaction &&
    reaction.message.interaction?.type === InteractionType.ApplicationCommand &&
    reaction.message.interaction?.user === user &&
    reaction.emoji.name === '❌'
  ) {
    await reaction.message.delete();
  }
});

reuploadCommands()
  .then(() => {
    client.login(process.env.DISCORD_TOKEN);
  })
  .catch((e) => {
    console.error(e);
    process.exit(1);
  });
