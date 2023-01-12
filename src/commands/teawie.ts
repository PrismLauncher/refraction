import type { CacheType, ChatInputCommandInteraction } from 'discord.js';
import { EmbedBuilder } from 'discord.js';

const teawieApiURL = 'https://guzzle.gay/api';

export interface TeawieResponse {
  /* The URL to the image of this teawie */
  url: string;
}

export const teawieCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply();

  const resp: Response = await fetch(teawieApiURL + `/get_random_teawie`, {
    headers: { Accept: 'application/json' },
  });

  if (!resp.ok) {
    await i.editReply({
      embeds: [
        new EmbedBuilder()
          .setTitle('Error!')
          .setDescription("Couldn't get a random teawie :("),
      ],
    });

    return;
  }

  const teawie: TeawieResponse = await resp.json();

  await i.editReply({
    embeds: [
      new EmbedBuilder()
        .setTitle('Teawie!')
        .setURL(teawie.url)
        .setImage(teawie.url),
    ],
  });
};
