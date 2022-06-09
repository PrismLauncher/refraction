import type { Command } from '../index';

export const cmd: Command = {
  name: 'members',
  desc: 'Shows the amount of online users in PolyMC Discord',
  aliases: ['mems', 'memcount'],
  exec: async (e) => {
    const memes = await e.guild?.members.fetch().then((r) => r.toJSON());
    if (!memes) return;

    await e.reply({
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
          color: 'GOLD',
        },
      ],
    });
  },
};
