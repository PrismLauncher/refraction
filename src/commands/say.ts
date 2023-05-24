import {
  CacheType,
  ChatInputCommandInteraction,
  EmbedBuilder,
} from 'discord.js';

export const sayCommand = async (
  interaction: ChatInputCommandInteraction<CacheType>
) => {
  if (!interaction.guild || !interaction.channel) return;

  const content = interaction.options.getString('content', true);
  await interaction.deferReply({ ephemeral: true });
  const message = await interaction.channel.send(content);
  await interaction.editReply('I said what you said!');

  if (typeof process.env.SAY_LOGS_CHANNEL === 'string') {
    const logsChannel = await interaction.guild.channels.fetch(
      process.env.SAY_LOGS_CHANNEL
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
