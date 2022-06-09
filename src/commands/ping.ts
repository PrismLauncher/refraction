import type { Command } from '../index';

export const cmd: Command = {
  name: 'ping',
  desc: 'Shows the ping of the bot',
  aliases: ['test'],
  exec: async (e) => {
    await e.reply(`${e.client.ws.ping}ms`);
  },
};
