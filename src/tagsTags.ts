import type { EmbedData } from 'discord.js';

import { readFile } from 'fs/promises';
import { join } from 'path';
import { COLORS } from './constants';

interface Tag {
  name: string;
  aliases?: Array<string>;
  text?: string;
  embed?: EmbedData;
}

export const getTags = async (): Promise<Tag[]> => {
  const raw = JSON.parse(
    await readFile(join(__dirname, 'tags.json'), { encoding: 'utf8' })
  ) as Tag[];

  return raw.map((tag) => {
    if (tag.embed?.color) {
      // @ts-expect-error this doesn't work for TypeScript but it does for me
      tag.embed.color = COLORS[tag.embed.color];
    }

    return tag;
  });
};
