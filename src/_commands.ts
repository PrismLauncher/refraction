import {
  SlashCommandBuilder,
  Routes,
  PermissionFlagsBits,
  type RESTGetAPIOAuth2CurrentApplicationResult,
} from 'discord.js';
import { REST } from '@discordjs/rest';
import { getTags } from './tags';

export const reuploadCommands = async () => {
  const tags = await getTags();

  const commands = [
    new SlashCommandBuilder()
      .setName('ping')
      .setDescription('Replies with pong!'),
    new SlashCommandBuilder()
      .setName('tag')
      .setDescription('Send a tag')
      .addStringOption((option) =>
        option
          .setName('name')
          .setDescription('The tag name')
          .setRequired(true)
          .addChoices(...tags.map((b) => ({ name: b.name, value: b.name })))
      )
      .addUserOption((option) =>
        option
          .setName('user')
          .setDescription('The user to mention')
          .setRequired(false)
      ),
    new SlashCommandBuilder()
      .setName('modrinth')
      .setDescription('Get info on a Modrinth project')
      .addStringOption((option) =>
        option.setName('id').setDescription('The ID or slug').setRequired(true)
      ),
    new SlashCommandBuilder()
      .setName('say')
      .setDescription('Say something through the bot')
      .addStringOption((option) =>
        option
          .setName('content')
          .setDescription('Just content?')
          .setRequired(true)
      )
      .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers)
      .setDMPermission(false),
    new SlashCommandBuilder().setName('joke').setDescription("it's a joke"),
    new SlashCommandBuilder()
      .setName('rory')
      .setDescription('Gets a Rory photo!')
      .addStringOption((option) =>
        option
          .setName('id')
          .setDescription('specify a Rory ID')
          .setRequired(false)
      ),
    new SlashCommandBuilder()
      .setName('ban')
      .setDescription('Ban a member')
      .addUserOption((option) =>
        option.setName('user').setDescription('Member to ban').setRequired(true)
      )
      .addStringOption((option) =>
        option
          .setName('reason')
          .setDescription('Reason of ban')
          .setRequired(true)
      )
      .addIntegerOption((option) =>
        option
          .setName('delete-message-days')
          .setDescription('Days of messages to delete')
          .setRequired(true)
      )
      .addBooleanOption((option) =>
        option
          .setName('silent')
          .setDescription('Disable DM to banned user (default false)')
          .setRequired(false)
      )
      .setDefaultMemberPermissions(PermissionFlagsBits.BanMembers),
    new SlashCommandBuilder()
      .setName('timeout')
      .setDescription('Time a member out')
      .addUserOption((option) =>
        option
          .setName('user')
          .setDescription('Member to timeout')
          .setRequired(true)
      )
      .addStringOption((option) =>
        option
          .setName('duration')
          .setDescription('Duration of timeout')
          .setRequired(true)
      )
      .addStringOption((option) =>
        option
          .setName('reason')
          .setDescription('Reason of timeout')
          .setRequired(true)
      )
      .addBooleanOption((option) =>
        option
          .setName('silent')
          .setDescription('Disable DM to banned user (default false)')
          .setRequired(false)
      )
      .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers),
    new SlashCommandBuilder()
      .setName('kick')
      .setDescription('Kick a member')
      .addUserOption((option) =>
        option
          .setName('user')
          .setDescription('Member to kick')
          .setRequired(true)
      )
      .addStringOption((option) =>
        option
          .setName('reason')
          .setDescription('Reason of kick')
          .setRequired(true)
      )
      .addBooleanOption((option) =>
        option
          .setName('silent')
          .setDescription('Disable DM to banned user (default false)')
          .setRequired(false)
      )
      .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers),
  ].map((command) => command.toJSON());

  const rest = new REST({ version: '10' }).setToken(process.env.DISCORD_TOKEN!);

  const { id: appId } = (await rest.get(
    Routes.oauth2CurrentApplication()
  )) as RESTGetAPIOAuth2CurrentApplicationResult;

  await rest.put(Routes.applicationCommands(appId), {
    body: commands,
  });

  console.log('Successfully registered application commands.');
};
