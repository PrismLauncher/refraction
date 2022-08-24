import type { CacheType, CommandInteraction } from 'discord.js';
import { COLORS } from '../constants';

export const starsCommand = async (i: CommandInteraction<CacheType>) => {
  const count = await fetch('https://api.github.com/repos/PolyMC/PolyMC')
    .then((r) => r.json() as Promise<{ stargazers_count: number }>)
    .then((j) => j.stargazers_count);

  await i.reply({
    embeds: [
      {
        title: `⭐ ${count} total stars!`,
        color: COLORS.yellow,
      },
    ],
  });
};
