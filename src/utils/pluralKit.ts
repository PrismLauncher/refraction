import { Message } from 'discord.js';

interface PkMessage {
  sender: string;
}

export const pkDelay = 1000;

export async function fetchPluralKitMessage(message: Message) {
  const response = await fetch(
    `https://api.pluralkit.me/v2/messages/${message.id}`
  );

  if (!response.ok) return null;

  return (await response.json()) as PkMessage;
}

export async function isMessageProxied(message: Message) {
  await new Promise((resolve) => setTimeout(resolve, pkDelay));
  return (await fetchPluralKitMessage(message)) !== null;
}
