import { MessageEmbed } from 'discord.js';
import type { Command } from '../index';
import { tags } from '../index';

export const cmd: Command = {
  name: 'tags',
  desc: 'Lists the tags available',
  exec: async (e) => {
    const em = new MessageEmbed().setTitle('tags').setColor('DARK_GREEN');

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
      em.addField(tag.name, text);
    }
    return e.reply({ embeds: [em] });
  },
};
