import {
  Client,
  GatewayIntentBits,
  Partials,
  OAuth2Scopes,
  InteractionType,
  SelectMenuBuilder,
  ActionRowBuilder,
  GuildMemberRoleManager,
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
import { roleMenuCommand } from './commands/rolemenu';

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
  if (interaction.isChatInputCommand()) {
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
    } else if (commandName === 'rolemenu') {
      await roleMenuCommand(interaction);
    }
  } else if (interaction.isButton()) {
    if (interaction.customId === 'showRoleMenu') {
      if (!interaction.guild || !interaction.member) return;

      const roles = await interaction.guild.roles
        .fetch()
        .then((a) =>
          a.filter((b) => BuildConfig.ALLOWED_ROLES.includes(b.name))
        );

      const row = new ActionRowBuilder<SelectMenuBuilder>().addComponents(
        new SelectMenuBuilder()
          .setCustomId('roleMenuSelect')
          .setPlaceholder('Nothing selected')
          .addOptions(
            roles.map((role) => ({
              label: role.name,
              value: role.id,
              default: (
                interaction.member!.roles as GuildMemberRoleManager
              ).cache.has(role.id),
            }))
          )
          .setMaxValues(roles.toJSON().length)
          .setMinValues(0)
      );

      await interaction.reply({
        content: 'Select your roles here.',
        components: [row],
        ephemeral: true,
      });
    }
  } else if (interaction.isSelectMenu()) {
    if (interaction.customId === 'roleMenuSelect') {
      if (!interaction.guild || !interaction.member) return;

      await interaction.deferReply({ ephemeral: true });

      const selectedRoles = interaction.values;

      const roleManager = interaction.member.roles as GuildMemberRoleManager;

      for (const role of BuildConfig.ALLOWED_ROLES) {
        const roleID = interaction.guild.roles.cache
          .find((a) => a.name === role)
          ?.id.toString();

        if (!roleID) continue;

        if (roleManager.cache.has(roleID) && !selectedRoles.includes(roleID)) {
          await roleManager.remove(roleID);
        }
        if (!roleManager.cache.has(roleID) && selectedRoles.includes(roleID)) {
          await roleManager.add(roleID);
        }
      }

      await interaction.editReply({
        content: 'Roles updated.',
      });

      setTimeout(() => {
        interaction.deleteReply();
      }, 3000);
    }
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
