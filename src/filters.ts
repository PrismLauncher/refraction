import { Message } from 'discord.js';
import { isBad } from './badLinks';
import urlRegex from 'url-regex';

// true if message is ok, false if filtered
export async function filterMessage(e: Message): Promise<boolean> {
  // url matcher
  const urlMatches = [...e.content.matchAll(urlRegex())];

  if (urlMatches.length) {
    console.log('Found links in message from', e.author.tag);

    for (const match of urlMatches) {
      console.log('[link]', match[0]);
      if (await isBad(match[0])) {
        await e.reply({
          embeds: [
            {
              title: 'Hold on!',
              description:
                'There seems to be a phishing / malware link in your message.',
              color: 'RED',
            },
          ],
        });

        return false;
      }
    }
  }

  return true;
}
