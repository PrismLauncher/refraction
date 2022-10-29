import type { EmbedData } from 'discord.js';

import matter from 'gray-matter';
import { readdir, readFile } from 'fs/promises';
import { join } from 'path';
import { COLORS } from './constants';

interface Tag {
  name: string;
  aliases?: string[];
  text?: string;
  embed?: EmbedData;
}

const TAG_DIR = join(process.cwd(), 'tags');

export const getTags = async (): Promise<Tag[]> => {
  const filenames = await readdir(TAG_DIR);
  const tags: Tag[] = [];

  for (const _file of filenames) {
    const file = join(TAG_DIR, _file);
    const { data, content } = matter(await readFile(file));

    if (data.embed) {
      tags.push({
        ...data,
        name: _file.replace('.md', ''),
        embed: {
          ...data.embed,
          description: content,
          color: COLORS[data.embed.color],
        },
      });
    } else {
      tags.push({
        ...data,
        name: _file.replace('.md', ''),
        text: content,
      });
    }
  }

  return tags;
};
