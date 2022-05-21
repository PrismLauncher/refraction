import { Client, Intents } from 'discord.js';
import { commands, aliases } from './commands';

import * as BuildConfig from './constants';
import Filter from 'bad-words';

import { green, bold, blue, underline } from 'kleur/colors';

const client = new Client({
  intents: [
    Intents.FLAGS.GUILDS,
    Intents.FLAGS.GUILD_MESSAGES,
    Intents.FLAGS.DIRECT_MESSAGES,
    Intents.FLAGS.GUILD_MEMBERS,
    Intents.FLAGS.GUILD_MESSAGE_REACTIONS,
    Intents.FLAGS.GUILD_BANS,
  ],
});

client.login(process.env.DISCORD_TOKEN);

client.once('ready', async () => {
  console.log(green(bold('Discord bot ready!')));
  console.log(
    'Invite link:',
    blue(
      underline(
        client.generateInvite({
          scopes: ['bot'],
          permissions: ['ADMINISTRATOR'],
        })
      )
    )
  );

  const POLYMC_GUILD = await client.guilds.fetch(BuildConfig.GUILD_ID);
  const DEBUG_CHANNEL = POLYMC_GUILD.channels.cache.get(
    BuildConfig.DEBUG_CHANNEL_ID
  );

  if (!DEBUG_CHANNEL.isText()) throw new Error();
  DEBUG_CHANNEL.send({
    embeds: [
      {
        title: 'Started!',
        description: new Date().toISOString(),
        color: 'AQUA',
      },
    ],
  });

  client.on('messageCreate', async (e) => {
    if (!e.content) return;
    if (!e.channel.isText()) return;
    if (e.author.bot) return;
    if (e.author === client.user) return;

    const profane = new Filter().isProfane(e.content);

    if (profane) {
      e.reply({
        embeds: [
          {
            title: 'Profanity detected!',
            description: 'Please try not to use these words 😄',
            color: 'FUCHSIA',
          },
        ],
      });
    }

    const cmd = e.content.split(' ')[0];
    if (!cmd.startsWith('!')) return;
    let func = commands[cmd];
    func = func ?? commands[aliases[cmd]];

    if (func !== undefined) {
      await func(client, e);
    }
  });
});
