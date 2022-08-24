import { SlashCommandBuilder, Routes } from 'discord.js';
import { REST } from '@discordjs/rest';
import { getTags } from './tagsTags';

import 'dotenv/config';

(async () => {
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
      .setName('rolypoly')
      .setDescription('Rooooooly Pooooooly'),
    new SlashCommandBuilder()
      .setName('tag')
      .setDescription('Send a tag')
      .addStringOption((option) =>
        option
          .setName('name')
          .setDescription('The tag name')
          .setRequired(true)
          .addChoices(...tags.map((b) => ({ name: b.name, value: b.name })))
      ),
  ].map((command) => command.toJSON());

  const rest = new REST({ version: '10' }).setToken(process.env.DISCORD_TOKEN!);

  await rest.put(Routes.applicationCommands('977174139297230888'), {
    body: [],
  });

  console.log('Successfully deleted all application commands.');

  await rest.put(Routes.applicationCommands('977174139297230888'), {
    body: commands,
  });

  console.log('Successfully registered application commands.');
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
