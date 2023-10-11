import {
  CacheType,
  ChatInputCommandInteraction,
  EmbedBuilder,
} from 'discord.js';

import config from '../config';

export const sayCommand = async (
  interaction: ChatInputCommandInteraction<CacheType>
) => {
  if (!interaction.guild || !interaction.channel) return;

  const content = interaction.options.getString('content', true);
  await interaction.deferReply({ ephemeral: true });
  const message = await interaction.channel.send(content);
  await interaction.editReply('I said what you said!');

  if (config.discord.channels.sayLogChannelId) {
    const logsChannel = await interaction.guild.channels.fetch(
      config.discord.channels.sayLogChannelId
    );

    if (!logsChannel?.isTextBased()) return;

    await logsChannel.send({
      embeds: [
        new EmbedBuilder()
          .setTitle('Say command used')
          .setDescription(content)
          .setAuthor({
            name: interaction.user.tag,
            iconURL: interaction.user.avatarURL() ?? undefined,
          })
          .setURL(message.url),
      ],
    });
  }
};
