import type { CacheType, ChatInputCommandInteraction } from 'discord.js';

export const jokeCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply();
  const joke = await fetch('https://icanhazdadjoke.com', {
    headers: { Accept: 'text/plain' },
  }).then((r) => r.text());
  await i.editReply(joke);
};
