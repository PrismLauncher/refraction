import { EmbedBuilder } from 'discord.js';
import { commands } from '.';
import type { Command } from '..';
import { COLORS } from '../constants';

export const cmd: Command = {
  name: 'help',
  desc: 'Shows this menu.',
  exec: async (e) => {
    const embed = new EmbedBuilder()
      .setTitle('Help Menu')
      .setColor(COLORS.green);

    const comman = commands;
    comman.sort((x, y) => {
      return x.name == 'help' ? -1 : y.name == 'help' ? 1 : 0;
    });

    for (const i in comman) {
      const cmd = comman[i];
      const resp = [];
      if (cmd.desc) {
        resp.push(cmd.desc);
      }
      if (cmd.aliases && cmd.aliases[0]) {
        resp.push(`**Aliases**: ${cmd.aliases.join(', ')}`);
      }
      if (cmd.examples && cmd.examples[0]) {
        resp.push(`**Examples**: \n${cmd.examples.join('\n> ')}`);
      }
      embed.addFields({ name: '!' + cmd.name, value: resp.join('\n') });
    }

    await e.reply({ embeds: [embed] });
  },
};
