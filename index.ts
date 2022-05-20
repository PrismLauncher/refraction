import { Client, Intents } from 'discord.js';
import Koa from 'koa';

const MUTED_ROLE = '976055433431240734';

// Health check server

{
  const app = new Koa();

  app.use(async (ctx) => {
    ctx.body = 'Hello there';
    ctx.res.setHeader('content-type', 'text/plain');
  });

  app.listen(3000, () => {
    console.log('Started server!');
  });
}

const client = new Client({
  intents: [Intents.FLAGS.GUILDS, Intents.FLAGS.GUILD_MESSAGES],
});

client.once('ready', async () => {
  console.log('Discord bot ready!');
});

client.login(process.env.DISCORD_TOKEN);

client.on('messageCreate', async (e) => {
  if (e.author === client.user) return;

  if ([...e.mentions.users.values()].includes(client.user)) {
    e.reply({
      content: `What\'s up <@${e.author.id}>`,
      allowedMentions: {
        parse: ['users'],
        repliedUser: true,
      },
    });

    return;
  }

  if (e.author.id === '360401361856364544') {
    e.react('975940717622984724');
    return;
  }

  if (
    e.member.roles.cache.has('975975986250256424') &&
    e.content.startsWith('!!mute')
  ) {
    const [, , time, ...more] = e.content.split(' ');
    if (more.length) {
      e.reply('Too many arguments!');
      return;
    }

    const parsedTime = parseInt(time);
    if (isNaN(parsedTime)) {
      e.reply('Not a number (seconds)!');
      return;
    }

    const member = e.mentions.members.at(0);
    await member.roles.add(MUTED_ROLE);

    setTimeout(() => {
      member.roles.remove(MUTED_ROLE);
    }, parsedTime * 1000);

    e.reply('Done.');
  }
});
