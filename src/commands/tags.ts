import {
  type ChatInputCommandInteraction,
  type CacheType,
  EmbedBuilder,
} from 'discord.js';
import { getTags } from '../tagsTags';

export const tagsCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  const tags = await getTags();
  const tagName = i.options.getString('name', true);
  const tag = tags.find(
    (tag) => tag.name === tagName || tag.aliases?.includes(tagName)
  );

  if (!tag) {
    await i.reply({
      content: `Tag \`${tagName}\` does not exist.`,
      ephemeral: true,
    });
    return;
  }

  await i.reply({
    content: tag.text ? `**${tag.name}**\n\n` + tag.text : tag.text,
    embeds: tag.embed
      ? [
          new EmbedBuilder(tag.embed).setFooter({
            text: `Requested by ${i.user.tag}`,
            iconURL: i.user.avatarURL() ?? undefined,
          }),
        ]
      : [],
  });
};
