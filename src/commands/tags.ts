import { EmbedBuilder } from 'discord.js';
import { getTags, type Command } from '..';
import { COLORS } from '../constants';

export const cmd: Command = {
  name: 'tags',
  desc: 'Lists the tags available',
  exec: async (e) => {
    const em = new EmbedBuilder().setTitle('tags').setColor(COLORS.green);

    const tags = await getTags();

    for (const i in tags) {
      const tag = tags[i];
      let text = '';

      if (tag.aliases && tag.aliases[0]) {
        text += '**Aliases**: ' + tag.aliases.join(', ') + '\n';
      }

      if (tag.text) {
        text += tag.text;
      } else if (tag.embed) {
        text += '\n[embedded message]';
      }

      em.addFields({ name: '?' + tag.name, value: text });
    }

    await e.reply({ embeds: [em] });
  },
};
