import { MessageEmbed } from 'discord.js';
import { getTags, type Command } from '..';

export const cmd: Command = {
  name: 'tags',
  desc: 'Lists the tags available',
  exec: async (e) => {
    const em = new MessageEmbed().setTitle('tags').setColor('DARK_GREEN');

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
      em.addField(tag.name, text);
    }

    await e.reply({ embeds: [em] });
  },
};
