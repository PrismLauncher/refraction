import { ChannelType, Message, PermissionFlagsBits } from 'discord.js';

import { getTimeIntervalID } from './datetime';
import { incr } from './redis';

const processMember = async (msg: Message<boolean>) => {
  const id = msg.author.id;

  await incr(`member:${id}:${getTimeIntervalID()}`, true);
};

const processChannel = async (msg: Message<boolean>) => {
  const guildChannel = await msg.guild!.channels.fetch(msg.channelId);
  if (!guildChannel) return;
  if (
    guildChannel.type === ChannelType.GuildStageVoice ||
    guildChannel.type === ChannelType.GuildVoice
  )
    return;

  let channelId = guildChannel.id;

  // this is a text thread or forum post
  if (
    guildChannel.parent &&
    (guildChannel.parent.type === ChannelType.GuildText ||
      guildChannel.parent.type === ChannelType.GuildForum)
  ) {
    channelId = guildChannel.parent.id;

    // if this is a thread in a text channel, track the thread as well
    if (guildChannel.parent.type === ChannelType.GuildText) {
      await incr(`thread:${guildChannel.id}:${getTimeIntervalID()}`, true);
    }
  }

  await incr(`channel:${channelId}:${getTimeIntervalID()}`, true);
};

export const collectStats = async (msg: Message<boolean>) => {
  // reject bots (and webhooks)
  if (msg.author.bot) return;

  // only collect stats on publicly accessible channels
  if (!msg.guild) return;
  const guildChannel = await msg.guild.channels.fetch(msg.channelId);
  if (!guildChannel) return;

  const everyonesPermsInChannel = guildChannel.permissionsFor(
    msg.guild.roles.everyone,
    false
  );

  if (!everyonesPermsInChannel.has(PermissionFlagsBits.ViewChannel, false))
    return;

  await incr(`total:forever`);
  await incr(`total:${getTimeIntervalID()}`, true);
  await processMember(msg);
  await processChannel(msg);
};
