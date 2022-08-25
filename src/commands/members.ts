import type { CacheType, CommandInteraction } from 'discord.js';

import { COLORS } from '../constants';

export const membersCommand = async (i: CommandInteraction<CacheType>) => {
  await i.deferReply();

  const memes = await i.guild?.members.fetch().then((r) => r.toJSON());
  if (!memes) return;

  await i.editReply({
    embeds: [
      {
        title: `${memes.length} total members!`,
        description: `${
          memes.filter(
            (m) =>
              m.presence?.status === 'online' ||
              m.presence?.status === 'idle' ||
              m.presence?.status === 'dnd'
          ).length
        } online members`,
        color: COLORS.blue,
      },
    ],
  });
};
