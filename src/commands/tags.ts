import {
  type ChatInputCommandInteraction,
  type CacheType,
  EmbedBuilder,
} from 'discord.js';
import { getTags } from '../tags';

export const tagsCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  const tags = await getTags();
  const tagName = i.options.getString('name', true);
  const mention = i.options.getUser('user', false);

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

  const embed = new EmbedBuilder();
  embed.setTitle(tag.title ?? tag.name);
  embed.setDescription(tag.content);
  if (tag.color) embed.setColor(tag.color);
  if (tag.image) embed.setImage(tag.image);
  if (tag.fields) embed.setFields(tag.fields);

  await i.reply({
    content: mention ? `<@${mention.id}> ` : undefined,
    embeds: [embed],
  });
};
