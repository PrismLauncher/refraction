import type { CacheType, ChatInputCommandInteraction } from 'discord.js';
import { EmbedBuilder } from 'discord.js';

export interface RoryResponse {
  /**
   * The ID of this Rory
   */
  id: number;
  /**
   * The URL to the image of this Rory
   */
  url: string;
  /**
   * When error :(
   */
  error: string | undefined;
}

export const roryCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply();

  const { value: id } = i.options.get('id') ?? { value: '' };

  const rory: RoryResponse = await fetch(`https://rory.cat/purr/${id}`, {
    headers: { Accept: 'application/json' },
  }).then((r) => r.json());

  if (rory.error) {
    await i.editReply({
      embeds: [
        new EmbedBuilder().setTitle('Error!').setDescription(rory.error),
      ],
    });

    return;
  }

  await i.editReply({
    embeds: [
      new EmbedBuilder()
        .setTitle('Rory :3')
        .setURL(`https://rory.cat/id/${rory.id}`)
        .setImage(rory.url)
        .setFooter({
          text: `ID ${rory.id}`,
        }),
    ],
  });
};
