import {
  Colors,
  EmbedBuilder,
  type Message,
  ThreadChannel,
  ReactionCollector,
} from 'discord.js';

function findFirstImage(message: Message): string | undefined {
  const result = message.attachments.find((attach) => {
    return attach.contentType?.startsWith('image/');
  });

  if (result === undefined) {
    return undefined;
  } else {
    return result.url;
  }
}

export async function expandDiscordLink(message: Message): Promise<void> {
  if (message.author.bot && !message.webhookId) return;

  const re =
    /(https?:\/\/)?(?:canary\.|ptb\.)?discord(?:app)?\.com\/channels\/(?<serverId>\d+)\/(?<channelId>\d+)\/(?<messageId>\d+)/g;

  const results = message.content.matchAll(re);
  const resultEmbeds: EmbedBuilder[] = [];

  for (const r of results) {
    if (resultEmbeds.length >= 3) break; // only process three previews

    if (r.groups == undefined || r.groups.serverId != message.guildId) continue; // do not let the bot leak messages from one server to another

    try {
      const channel = await message.guild?.channels.fetch(r.groups.channelId);

      if (!channel || !channel.isTextBased()) continue;

      if (channel instanceof ThreadChannel) {
        if (
          !channel.parent?.members?.some((user) => user.id == message.author.id)
        )
          continue; // do not reveal a message to a user who can't see it
      } else {
        if (!channel.members?.some((user) => user.id == message.author.id))
          continue; // do not reveal a message to a user who can't see it
      }

      const originalMessage = await channel.messages.fetch(r.groups.messageId);

      const embed = new EmbedBuilder()
        .setAuthor({
          name: originalMessage.author.tag,
          iconURL: originalMessage.author.displayAvatarURL(),
        })
        .setColor(Colors.Aqua)
        .setTimestamp(originalMessage.createdTimestamp)
        .setFooter({ text: `#${originalMessage.channel.name}` });

      embed.setDescription(
        (originalMessage.content ? originalMessage.content + '\n\n' : '') +
          `[Jump to original message](${originalMessage.url})`
      );

      if (originalMessage.attachments.size > 0) {
        embed.addFields({
          name: 'Attachments',
          value: originalMessage.attachments
            .map((att) => `[${att.name}](${att.url})`)
            .join('\n'),
        });

        const firstImage = findFirstImage(originalMessage);
        if (firstImage) {
          embed.setImage(firstImage);
        }
      }

      resultEmbeds.push(embed);
    } catch (ignored) {
      /* */
    }
  }

  if (resultEmbeds.length > 0) {
    const reply = await message.reply({
      embeds: resultEmbeds,
      allowedMentions: { repliedUser: false },
    });

    const collector = new ReactionCollector(reply, {
      filter: (reaction) => {
        return reaction.emoji.name === '❌';
      },
      time: 5 * 60 * 1000,
    });

    collector.on('collect', async (_, user) => {
      if (user === message.author) {
        await reply.delete();
        collector.stop();
      }
    });
  }
}
