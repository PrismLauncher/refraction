import { MessageEmbed } from 'discord.js';
import { commands } from '.';
import { Command } from '..';

export const cmd: Command = {
  name: 'help',
  desc: 'Shows this menu.',
  exec: async (e) => {
    const embed = new MessageEmbed()
      .setTitle('Help Menu')
      .setColor('DARK_GREEN');

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
      embed.addField('!' + cmd.name, resp.join('\n'));
    }

    await e.reply({ embeds: [embed] });
  },
};
