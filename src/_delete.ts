import { REST } from '@discordjs/rest';
import { Routes } from 'discord.js';

import 'dotenv/config';

const rest = new REST({ version: '10' }).setToken(process.env.DISCORD_TOKEN!);

rest
  .put(Routes.applicationCommands('977174139297230888'), { body: [] })
  .then(() => console.log('Successfully deleted all application commands.'))
  .catch(console.error);
