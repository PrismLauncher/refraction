import { CacheType, ChatInputCommandInteraction } from 'discord.js';
import { Uwurandom } from 'uwurandom-node';

const random = Uwurandom.new();

export const uwurandomCommand = async (
    interaction: ChatInputCommandInteraction<CacheType>
) => {
    const length = interaction.options.getInteger('n', true);
    let message = "";
    for (let i = 0; i < length; i++)
        message += random.generate();

    interaction.reply(message.replace('*', '\\*'));
};