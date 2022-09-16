import {
  ActionRowBuilder,
  ButtonBuilder,
  ButtonStyle,
  CacheType,
  ChatInputCommandInteraction,
} from 'discord.js';

export const roleMenuCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  const row = new ActionRowBuilder<ButtonBuilder>().addComponents(
    new ButtonBuilder()
      .setCustomId('showRoleMenu')
      .setLabel('Show role menu')
      .setStyle(ButtonStyle.Primary)
  );

  await i.channel?.send({ content: '**Role menu**', components: [row] });

  await i.reply({ content: 'Done!', ephemeral: true });
};
