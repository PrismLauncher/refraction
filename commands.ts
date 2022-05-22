import type { Client, Message } from 'discord.js';
import pLimit from 'p-limit';
import { POLYCAT_CHANNEL_ID } from './constants';

type Commands = {
  [cmd: string]: (c: Client, e: Message) => void | Promise<void>;
};

export const commands: Commands = {
  '!ping': async (c, e) => {
    await e.reply(`${c.ws.ping}ms`);
  },

  '!why': async (c, e) => {
    await e.reply({
      embeds: [
        {
          title: 'Why PolyMC exists',
          description:
            'https://polymc.org/wiki/overview/faq/#why-did-our-community-choose-to-fork\nhttps://polymc.org/news/moving-on/',
          color: 'GREYPLE',
        },
      ],
    });
  },

  '!paths': async (c, e) => {
    await e.reply({
      embeds: [
        {
          title: 'Data directories',
          description: 'Where PolyMC stores your data (e.g. instances)',
          color: 'AQUA',
          fields: [
            {
              name: 'Portable (Windows / Linux)',
              value: 'In the PolyMC folder',
            },
            {
              name: 'Windows',
              value: '`%APPDATA%/PolyMC`',
            },
            {
              name: 'macOS',
              value: '`~/Library/Application Support/PolyMC`',
            },
            { name: 'Linux', value: '`~/.local/share/PolyMC`' },
            {
              name: 'Flatpak',
              value: '`~/.var/app/org.polymc.PolyMC/data/PolyMC`',
            },
          ],
        },
      ],
    });
  },

  '!cursed': async (c, e) => {
    await e.reply({
      embeds: [
        {
          title: "What's wrong with CurseForge?",
          description: `
There is a new option to block third party clients from accessing mod files. CurseForge started to enforce the option for modders to disallow third-party applications like PolyMC and other launchers.

We probably can't fully fix this. If you find out which mod is causing this, tell the modder to toggle that option.
`.trim(),
          color: 'ORANGE',
        },
      ],
    });
  },

  '!migrate': async (c, e) => {
    await e.reply('https://polymc.org/wiki/getting-started/migrating-multimc/');
  },

  '!build': async (c, e) => {
    await e.reply('https://polymc.org/wiki/development/build-instructions/');
  },

  '!eta': async (c, e) => {
    await e.reply('Sometime');
  },

  '!members': async (c, e) => {
    const mems = await e.guild?.members.fetch().then((r) => r.toJSON());
    if (!mems) return;

    await e.reply({
      embeds: [
        {
          title: `${mems.length} total members!`,
          description: `${
            mems.filter((m) => m.presence?.status === 'online').length
          } online members`,
          color: 'GOLD',
        },
      ],
    });
  },

  '!polycatgen': async (c, e) => {
    if (!e.guild) return;
    if (e.channelId !== POLYCAT_CHANNEL_ID) return;

    await e.guild.emojis.fetch();
    const polycat = e.guild.emojis.cache.find(
      (emoji) => emoji.name?.toLowerCase() === 'polycat'
    );

    const lim = pLimit(2);
    const prom = [];
    for (let i = 0; i < 10; i++) {
      prom.push(
        lim(() =>
          e.channel.send(`${polycat}${polycat}${polycat}${polycat}${polycat}`)
        )
      );
    }
    await Promise.all(prom);
  },
};

export const aliases: { [a: string]: string } = {
  '!curse': '!cursed',
  '!curseforge': '!cursed',
  '!diff': '!why',
  '!migr': '!migrate',
  '!multimc': '!migrate',
};
