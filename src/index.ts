import 'dotenv/config';

import {
  Client,
  GatewayIntentBits,
  Partials,
  OAuth2Scopes,
  InteractionType,
  PermissionFlagsBits,
  ChannelType,
  Events,
} from 'discord.js';

import config from './config';

import { reuploadCommands } from './_reupload';
import { listen as listenApp } from './server';
import {
  connect as connectStorage,
  isUserPlural,
  storeUserPlurality,
} from './storage';
import { scheduleJobs } from './tasks';

import * as BuildConfig from './constants';
import { parseLog } from './logs';
import { getLatestMinecraftVersion } from './utils/remoteVersions';
import { expandDiscordLink } from './utils/resolveMessage';

import { membersCommand } from './commands/members';
import { starsCommand } from './commands/stars';
import { modrinthCommand } from './commands/modrinth';
import { tagsCommand } from './commands/tags';
import { jokeCommand } from './commands/joke';
import { roryCommand } from './commands/rory';
import { sayCommand } from './commands/say';

import random from 'just-random';
import { green, bold, yellow, cyan } from 'kleur/colors';

import {
  fetchPluralKitMessage,
  isMessageProxied,
  pkDelay,
} from './utils/pluralKit';

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
    GatewayIntentBits.GuildModeration,
  ],
  partials: [Partials.Channel],
});

client.once('ready', async () => {
  console.log(green('Discord bot ready!'));

  console.log(
    cyan(
      client.generateInvite({
        scopes: [OAuth2Scopes.Bot],
        permissions: [
          PermissionFlagsBits.AddReactions,
          PermissionFlagsBits.ViewChannel,
          PermissionFlagsBits.BanMembers,
          PermissionFlagsBits.KickMembers,
          PermissionFlagsBits.CreatePublicThreads,
          PermissionFlagsBits.CreatePrivateThreads,
          PermissionFlagsBits.EmbedLinks,
          PermissionFlagsBits.ManageChannels,
          PermissionFlagsBits.ManageRoles,
          PermissionFlagsBits.ModerateMembers,
          PermissionFlagsBits.MentionEveryone,
          PermissionFlagsBits.MuteMembers,
          PermissionFlagsBits.SendMessages,
          PermissionFlagsBits.SendMessagesInThreads,
          PermissionFlagsBits.ReadMessageHistory,
        ],
      })
    )
  );

  if (process.env.NODE_ENV !== 'development')
    console.warn(yellow(bold('Running in production mode!')));

  const mcVersion = await getLatestMinecraftVersion();
  client.user?.presence.set({
    activities: [{ name: `Minecraft ${mcVersion}` }],
    status: 'online',
  });

  client.on(Events.MessageCreate, async (e) => {
    try {
      if (e.channel.partial) await e.channel.fetch();
      if (e.author.partial) await e.author.fetch();

      if (!e.content) return;
      if (!e.channel.isTextBased()) return;

      if (e.author === client.user) return;

      if (e.webhookId !== null) {
        // defer PK detection
        setTimeout(async () => {
          const pkMessage = await fetchPluralKitMessage(e);
          if (pkMessage !== null) storeUserPlurality(pkMessage.sender);
        }, pkDelay);
      }

      if ((await isUserPlural(e.author.id)) && (await isMessageProxied(e)))
        return;

      if (e.cleanContent.match(BuildConfig.ETA_REGEX)) {
        await e.reply(
          `${random(BuildConfig.ETA_MESSAGES)} <:pofat:1031701005559144458>`
        );
      }
      const log = await parseLog(e.content);
      if (log != null) {
        e.reply({ embeds: [log] });
        return;
      }
      await expandDiscordLink(e);
    } catch (error) {
      console.error('Unhandled exception on MessageCreate', error);
    }
  });
});

client.on(Events.InteractionCreate, async (interaction) => {
  try {
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
    } else if (commandName === 'say') {
      await sayCommand(interaction);
    } else if (commandName === 'tag') {
      await tagsCommand(interaction);
    } else if (commandName === 'joke') {
      await jokeCommand(interaction);
    } else if (commandName === 'rory') {
      await roryCommand(interaction);
    }
  } catch (error) {
    console.error('Unhandled exception on InteractionCreate', error);
  }
});

client.on(Events.MessageReactionAdd, async (reaction, user) => {
  try {
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
      reaction.message.interaction?.type ===
        InteractionType.ApplicationCommand &&
      reaction.message.interaction?.user === user &&
      reaction.emoji.name === '❌'
    ) {
      await reaction.message.delete();
    }
  } catch (error) {
    console.error('Unhandled exception on MessageReactionAdd', error);
  }
});

client.on(Events.ThreadCreate, async (channel) => {
  try {
    if (
      channel.type === ChannelType.PublicThread &&
      channel.parent &&
      channel.parent.name === 'support' &&
      channel.guild
    ) {
      const pingRole = channel.guild.roles.cache.find(
        (r) => r.name === 'Moderators'
      );

      if (!pingRole) return;

      await channel.send({
        content: `
    <@${channel.ownerId}> We've received your support ticket! Please upload your logs and post the link here if possible (run \`/tag log\` to find out how). Please don't ping people for support questions, unless you have their permission.
        `.trim(),
        allowedMentions: {
          repliedUser: true,
          roles: [],
          users: channel.ownerId ? [channel.ownerId] : [],
        },
      });
    }
  } catch (error) {
    console.error('Error handling ThreadCreate', error);
  }
});

reuploadCommands()
  .then(() => {
    client.login(config.discord.botToken);
    connectStorage();
    scheduleJobs();
    listenApp();
  })
  .catch((e) => {
    console.error(e);
    process.exit(1);
  });
