import type { Client, Message } from 'discord.js';
import fetch from 'node-fetch';
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
CurseForge added a new option to block third party clients like PolyMC from accessing mod files, and they started to enforce this option lately.

We can't allow you to download those mods directly from PolyMC, but PolyMC 1.3.1 and higher have a workaround to let modpacks work: letting you to download those opted out mods manually.
We highly encourage asking authors that opted out to stop doing so.
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
            mems.filter((m) => m.presence?.status !== 'offline').length
          } online members`,
          color: 'GOLD',
        },
      ],
    });
  },

  '!stars': async (c, e) => {
    const count = await fetch('https://api.github.com/repos/PolyMC/PolyMC')
      .then((r) => r.json() as Promise<{ stargazers_count: number }>)
      .then((j) => j.stargazers_count);

    await e.reply({
      embeds: [
        {
          title: `⭐ ${count} total stars!`,
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

  '!piracy': async (c, e) => {
    await e.reply({
      embeds: [
        {
          title: "We don't tolerate piracy!",
          description:
            "PolyMC has always been legal, legitimate & appropriate. We don't and never will have features such as offline login without an official account.",
          color: 'DARK_RED',
        },
      ],
    });
  },
};

export const aliases: { [a: string]: string } = {
  '!curse': '!cursed',
  '!curseforge': '!cursed',
  '!cf': '!cursed',
  '!diff': '!why',
  '!migr': '!migrate',
  '!multimc': '!migrate',
};
