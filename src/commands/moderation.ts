import {
  EmbedBuilder,
  ChannelType,
  type CacheType,
  type ChatInputCommandInteraction,
} from 'discord.js';
import parseDuration from 'parse-duration';

export const banCommand = async (i: ChatInputCommandInteraction<CacheType>) => {
  await i.deferReply({ ephemeral: true });

  const user = i.options.getUser('user', true);
  const reason = i.options.getString('reason', true);
  const deleteMessageDays = i.options.getInteger('delete-message-days', true);
  const silent = i.options.getBoolean('silent') ?? false;

  const member = await i.guild?.members.fetch(user);

  if (!member) {
    await i.editReply('User is not member of this guild!');
    return;
  }
  if (!member.moderatable) {
    await i.editReply('Member is not moderatable!');
    return;
  }

  if (!silent) {
    const dm = await user.createDM();
    await dm.send({
      embeds: [
        new EmbedBuilder()
          .setTitle(`You have been banned from ${i.guild!.name}!`)
          .setColor(0xef4444)
          .addFields({ name: 'Reason', value: reason })
          .addFields({
            name: 'Appeal',
            value: 'https://discord.gg/aZFj7sZPkU',
          }),
      ],
    });
  }

  const { MOD_LOGS_CHANNEL } = process.env;
  if (MOD_LOGS_CHANNEL !== undefined) {
    const channel = i.guild?.channels?.cache.get(MOD_LOGS_CHANNEL);

    if (channel && channel.type === ChannelType.GuildText) {
      await channel.send({
        embeds: [
          new EmbedBuilder()
            .setTitle(`Banned a user!`)
            .setColor(0xef4444)
            .setDescription(`${user} has been banned!`)
            .addFields({ name: 'User ID', value: user.id })
            .addFields({ name: 'Reason', value: reason })
            .addFields({ name: 'Silent', value: `${silent}` })
            .addFields({
              name: 'Days of messages deleted',
              value: `${deleteMessageDays}`,
            })
            .setFooter({
              text: i.user.username,
              iconURL: i.user.displayAvatarURL(),
            })
            .setTimestamp(Date.now()),
        ],
      });
    }
  }

  await member.ban({ deleteMessageSeconds: deleteMessageDays * 3600, reason });
  await i.editReply(`Banned ${user}!`);
};

export const timeoutCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply({ ephemeral: true });

  const user = i.options.getUser('user', true);
  const reason = i.options.getString('reason', true);
  const durationString = i.options.getString('duration', true);
  const duration = parseDuration(durationString);
  const silent = i.options.getBoolean('silent') ?? false;

  const member = await i.guild?.members.fetch(user);

  if (!member) {
    await i.editReply('User is not member of this guild!');
    return;
  }
  if (!member.moderatable) {
    await i.editReply('Member is not moderatable!');
    return;
  }
  if (!duration) {
    await i.editReply('Invalid duration provided!');
    return;
  }

  const startS = Math.floor(Date.now() / 1000);
  const endS = Math.floor(startS + duration / 1000);

  if (!silent) {
    const dm = await user.createDM();
    await dm.send({
      embeds: [
        new EmbedBuilder()
          .setTitle(`You have been timed out in ${i.guild!.name}!`)
          .setColor(0xef4444)
          .addFields({ name: 'Reason', value: reason })
          .addFields({ name: 'Start', value: `<t:${startS}:F>` })
          .addFields({ name: 'End', value: `<t:${endS}:F> (<t:${endS}:R>)` }),
      ],
    });
  }

  const { MOD_LOGS_CHANNEL } = process.env;
  if (MOD_LOGS_CHANNEL !== undefined) {
    const channel = i.guild?.channels?.cache.get(MOD_LOGS_CHANNEL);

    if (channel && channel.type === ChannelType.GuildText) {
      await channel.send({
        embeds: [
          new EmbedBuilder()
            .setTitle(`Timed a user out!`)
            .setColor(0xef4444)
            .setDescription(`${user} has been timed out!`)
            .addFields({ name: 'User ID', value: user.id })
            .addFields({ name: 'Duration', value: durationString })
            .addFields({ name: 'Start', value: `<t:${startS}:F>` })
            .addFields({ name: 'End', value: `<t:${endS}:F> (<t:${endS}:R>)` })
            .addFields({ name: 'Silent', value: `${silent}` })
            .setFooter({
              text: i.user.username,
              iconURL: i.user.displayAvatarURL(),
            })
            .setTimestamp(Date.now()),
        ],
      });
    }
  }

  await member.timeout(duration, reason);
  await i.editReply(`Timed ${user} out!`);
};

export const kickCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply({ ephemeral: true });

  const user = i.options.getUser('user', true);
  const reason = i.options.getString('reason', true);
  const silent = i.options.getBoolean('silent') ?? false;

  const member = await i.guild?.members.fetch(user);

  if (!member) {
    await i.editReply('User is not member of this guild!');
    return;
  }
  if (!member.moderatable) {
    await i.editReply('Member is not moderatable!');
    return;
  }

  if (!silent) {
    const dm = await user.createDM();
    await dm.send({
      embeds: [
        new EmbedBuilder()
          .setTitle(`You have been kicked from ${i.guild!.name}!`)
          .setColor(0xef4444)
          .addFields({ name: 'Reason', value: reason }),
      ],
    });
  }

  const { MOD_LOGS_CHANNEL } = process.env;
  if (MOD_LOGS_CHANNEL !== undefined) {
    const channel = i.guild?.channels?.cache.get(MOD_LOGS_CHANNEL);

    if (channel && channel.type === ChannelType.GuildText) {
      await channel.send({
        embeds: [
          new EmbedBuilder()
            .setTitle(`Kicjed a user!`)
            .setColor(0xef4444)
            .setDescription(`${user} has been kicked!`)
            .addFields({ name: 'User ID', value: user.id })
            .addFields({ name: 'Reason', value: reason })
            .addFields({ name: 'Silent', value: `${silent}` })
            .setFooter({
              text: i.user.username,
              iconURL: i.user.displayAvatarURL(),
            })
            .setTimestamp(Date.now()),
        ],
      });
    }
  }

  await member.kick(reason);
  await i.editReply(`Kicked ${user}!`);
};
