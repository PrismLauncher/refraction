type Side = 'required' | 'optional' | 'unsupported';

export interface ModrinthProject {
  slug: string;
  title: string;
  description: string;
  categories: string[];
  client_side: Side;
  server_side: Side;
  project_type: 'mod' | 'modpack';
  downloads: number;
  icon_url: string | null;
  id: string;
  team: string;
}

import {
  EmbedBuilder,
  type CacheType,
  type ChatInputCommandInteraction,
} from 'discord.js';

import { COLORS } from '../constants';

export const modrinthCommand = async (
  i: ChatInputCommandInteraction<CacheType>
) => {
  await i.deferReply();

  const { value: id } = i.options.get('id') ?? { value: null };

  if (!id || typeof id !== 'string') {
    await i.editReply({
      embeds: [
        new EmbedBuilder()
          .setTitle('Error!')
          .setDescription('You need to provide a valid mod ID!')
          .setColor(COLORS.red),
      ],
    });

    return;
  }

  const res = await fetch('https://api.modrinth.com/v2/project/' + id);

  if (!res.ok) {
    await i.editReply({
      embeds: [
        new EmbedBuilder()
          .setTitle('Error!')
          .setDescription('Not found!')
          .setColor(COLORS.red),
      ],
    });

    setTimeout(() => {
      i.deleteReply();
    }, 3000);

    return;
  }

  const data = (await res.json()) as
    | ModrinthProject
    | { error: string; description: string };

  if ('error' in data) {
    console.error(data);

    await i.editReply({
      embeds: [
        new EmbedBuilder()
          .setTitle('Error!')
          .setDescription(`\`${data.error}\` ${data.description}`)
          .setColor(COLORS.red),
      ],
    });

    setTimeout(() => {
      i.deleteReply();
    }, 3000);

    return;
  }

  await i.editReply({
    embeds: [
      new EmbedBuilder()
        .setTitle(data.title)
        .setDescription(data.description)
        .setThumbnail(data.icon_url)
        .setURL(`https://modrinth.com/project/${data.slug}`)
        .setFields([
          {
            name: 'Categories',
            value: data.categories.join(', '),
            inline: true,
          },
          {
            name: 'Project type',
            value: data.project_type,
            inline: true,
          },
          {
            name: 'Downloads',
            value: data.downloads.toString(),
            inline: true,
          },
          {
            name: 'Client',
            value: data.client_side,
            inline: true,
          },
          {
            name: 'Server',
            value: data.server_side,
            inline: true,
          },
        ])
        .setColor(COLORS.green),
    ],
  });
};
