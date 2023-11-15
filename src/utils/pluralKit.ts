import { Message } from "discord.js";

export async function proxied(message: Message): Promise<boolean> {
    if (message.webhookId !== null)
        return false;

    await new Promise(resolve => setTimeout(resolve, 300));
    const response = await fetch(`https://api.pluralkit.me/v2/messages/${message.id}`);
    return response.ok;
}
