import {
  SlashCommandBuilder,
  Routes,
  PermissionFlagsBits,
  type RESTGetAPIOAuth2CurrentApplicationResult,
} from 'discord.js';
import { REST } from '@discordjs/rest';
import { getTags } from './tags';

import config from './config';

export const reuploadCommands = async () => {
  const tags = await getTags();

  const commands = [
    new SlashCommandBuilder()
      .setName('ping')
      .setDescription('Replies with pong!'),
    new SlashCommandBuilder()
      .setName('stars')
      .setDescription('Returns GitHub stargazer count'),
    new SlashCommandBuilder()
      .setName('members')
      .setDescription('Returns the number of members in the server'),
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
  ].map((command) => command.toJSON());

  const rest = new REST({ version: '10' }).setToken(config.discord.botToken);

  const { id: appId } = (await rest.get(
    Routes.oauth2CurrentApplication()
  )) as RESTGetAPIOAuth2CurrentApplicationResult;

  await rest.put(Routes.applicationCommands(appId), {
    body: commands,
  });

  console.log('Successfully registered application commands.');
};
