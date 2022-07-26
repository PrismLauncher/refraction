import { Client, Message, EmbedBuilder, type EmbedData } from 'discord.js';

import * as BuildConfig from './constants';
import { commands } from './commands';
import { filterMessage } from './filters';
import { parseLog } from './logs';
import { getLatestMinecraftVersion } from './utils/remoteVersions';

import {
  parse as discordParse,
  type SuccessfulParsedMessage,
} from 'discord-command-parser';

import random from 'just-random';
import { readFile } from 'fs/promises';
import { join } from 'path';
import { green, bold, yellow } from 'kleur/colors';
import 'dotenv/config';

export interface Command {
  name: string;
  aliases?: string[];
  desc?: string;
  examples?: string[];
  exec(
    m: Message,
    p: SuccessfulParsedMessage<Message<boolean>>
  ): Promise<void> | void;
}

interface Tag {
  name: string;
  aliases?: Array<string>;
  text?: string;
  embed?: EmbedData;
}

export const getTags = async (): Promise<Tag[]> => {
  const raw = JSON.parse(
    await readFile(join(__dirname, 'tags.json'), { encoding: 'utf8' })
  ) as Tag[];

  return raw.map((tag) => {
    if (tag.embed?.color) {
      tag.embed.color = BuildConfig.COLORS[tag.embed.color];
    }

    return tag;
  });
};

const client = new Client({
  intents: [
    'Guilds',
    'GuildMessages',
    'MessageContent',
    'DirectMessages',
    'GuildMembers',
    'GuildPresences',
    'GuildMessageReactions',
    'GuildBans',
  ],
});

client.once('ready', async () => {
  console.log(green('Discord bot ready!'));

  if (process.env.NODE_ENV !== 'development')
    console.warn(yellow(bold('Running in production mode!')));

  client.user?.presence.set({
    activities: [{ name: `Minecraft ${await getLatestMinecraftVersion()}` }],
    status: 'online',
  });

  client.on('messageCreate', async (e) => {
    if (!e.content) return;
    if (!e.channel.isTextBased()) return;
    if (e.author === client.user) return;

    if (
      process.env.NODE_ENV === 'development' &&
      e.channelId !== BuildConfig.DEBUG_CHANNEL_ID
    ) {
      return;
    } else if (
      process.env.NODE_ENV !== 'development' &&
      e.channelId === BuildConfig.DEBUG_CHANNEL_ID
    ) {
      return;
    }

    const messageIsOK = await filterMessage(e);
    if (!messageIsOK) {
      return;
    }

    if (e.cleanContent.match(BuildConfig.ETA_REGEX)) {
      await e.reply(
        `${random(BuildConfig.ETA_MESSAGES)} <:pofat:964546613194420294>`
      );
    }

    const commanded = await parseMsgForCommands(e);
    if (commanded) return;
    const tagged = await parseMsgForTags(e);
    if (tagged) return;

    const log = await parseLog(e.content);
    if (log != null) {
      e.reply({ embeds: [log] });
      return;
    }
  });
});

async function parseMsgForCommands(e: Message) {
  const parsed = discordParse(e, '!', { allowBots: true });

  if (!parsed.success) return false;
  const cmd = commands.find(
    (c) => c.name == parsed.command || c.aliases?.includes(parsed.command)
  );

  if (!cmd) {
    return false;
  }

  try {
    await cmd.exec(e, parsed);
  } catch (err: unknown) {
    const em = new EmbedBuilder()
      .setTitle('Error')
      .setColor(BuildConfig.COLORS.red)
      // @ts-expect-error no why
      .setDescription(err['message'] as string);

    e.reply({ embeds: [em] });
  }

  return true;
}

async function parseMsgForTags(e: Message) {
  const parsed = discordParse(e, '?', { allowBots: true });
  if (!parsed.success) return false;

  const tag = await getTags().then((r) =>
    r.find(
      (t) => t.name == parsed.command || t.aliases?.includes(parsed.command)
    )
  );

  if (tag) {
    const requesterAvatarURL = e.author.avatar;
    const tagRequester = {
      text: `Requested by ${e.author.tag}`,
      ...(requesterAvatarURL ? { icon_url: requesterAvatarURL } : null),
    };

    if (tag.text) {
      e.reply({
        embeds: [
          new EmbedBuilder({
            title: tag.name,
            description: tag.text,
            footer: tagRequester,
          }),
        ],
      });
    } else if (tag.embed) {
      const em = new EmbedBuilder(tag.embed).setFooter(tagRequester);
      e.reply({ embeds: [em] });
    }

    return true;
  }
}

client.login(process.env.DISCORD_TOKEN);
