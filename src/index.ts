import {
  Client,
  EmbedBuilder,
  GatewayIntentBits,
  Partials,
  ChannelType,
  OAuth2Scopes,
} from 'discord.js';

import * as BuildConfig from './constants';
import { parseLog } from './logs';
import { getLatestMinecraftVersion } from './utils/remoteVersions';

import { membersCommand } from './commands/members';
import { starsCommand } from './commands/stars';

import random from 'just-random';
import { green, bold, yellow } from 'kleur/colors';
import 'dotenv/config';
import { getTags } from './tagsTags';
import { modrinthCommand } from './commands/modrinth';

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
    activities: [
      {
        name: `Minecraft ${mcVersion}${
          mcVersion === '1.19.1' || mcVersion === '1.19.2'
            ? ' w/ No Chat Reports'
            : ''
        }`,
      },
    ],
    status: 'online',
  });

  client.on('messageCreate', async (e) => {
    if (!e.content) return;
    if (!e.channel.isTextBased()) return;

    if (e.author === client.user) return;

    if (
      process.env.NODE_ENV === 'development' &&
      e.channelId !== BuildConfig.DEBUG_CHANNEL_ID &&
      e.channel.type === ChannelType.DM &&
      !BuildConfig.DM_TESTERS.includes(e.author.id)
    ) {
      return;
    } else if (
      process.env.NODE_ENV !== 'development' &&
      e.channelId === BuildConfig.DEBUG_CHANNEL_ID
    ) {
      return;
    }

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

  if (
    process.env.NODE_ENV === 'development' &&
    interaction.channelId !== BuildConfig.DEBUG_CHANNEL_ID
  ) {
    return;
  }
  if (
    process.env.NODE_ENV !== 'development' &&
    interaction.channelId === BuildConfig.DEBUG_CHANNEL_ID
  ) {
    return;
  }

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
  } else if (commandName === 'tag') {
    const tags = await getTags();
    const tagName = interaction.options.getString('name', true);
    const tag = tags.find(
      (tag) => tag.name === tagName || tag.aliases?.includes(tagName)
    );

    if (!tag) {
      await interaction.reply({
        content: `Tag \`${tagName}\` does not exist.`,
        ephemeral: true,
      });
      return;
    }

    await interaction.reply({
      content: tag.text ? `**${tag.name}**\n\n` + tag.text : tag.text,
      embeds: tag.embed
        ? [
            new EmbedBuilder(tag.embed).setFooter({
              text: `Requested by ${interaction.user.tag}`,
              iconURL: interaction.user.avatarURL() ?? undefined,
            }),
          ]
        : [],
    });
  }
});

client.login(process.env.DISCORD_TOKEN);
