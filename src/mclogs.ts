import { getLatest } from './version';
import { MessageEmbed } from 'discord.js';
const reg = /https\:\/\/mclo.gs\/[^ ]*/g;

type analyzer = (text: string) => Promise<Array<string> | null>;
const javaAnalyzer: analyzer = async (text) => {
  if (text.includes('This instance is not compatible with Java version')) {
    const xp =
      /Please switch to one of the following Java versions for this instance:[\r\n]+([^\r\n]+)/g;

    let ver: string;
    const m = text.match(xp);
    if (!m || !m[0]) {
      ver = '';
    } else {
      ver = m[0].split('\n')[1];
    }

    return [
      'WrongJavaVersion',
      `Please switch to the following: \`${ver}\`\nFor more information, type \`!java\``,
    ];
  }
  return null;
};

const versionAnalyzer: analyzer = async (text) => {
  const vers = text.match(/PolyMC version: [0-9].[0-9].[0-9]/g);
  if (vers && vers[0]) {
    const latest = await getLatest();
    const current = vers[0].replace('PolyMC version: ', '');
    if (latest != current) {
      return [
        'OutdatedPolyMC',
        `Your installed version is ${current}, while the newest version is ${latest}.\nPlease update, for more info see https://polymc.org/download/`,
      ];
    }
  }
  return null;
};

const analyzers: analyzer[] = [javaAnalyzer, versionAnalyzer];

export async function parseLog(s: string): Promise<MessageEmbed | null> {
  const r = s.match(reg);
  if (r == null || !r[0]) return null;
  const link = r[0]; // for now only first url
  const id = link.replace('https://mclo.gs/', '');
  if (!id) return null;
  const apiUrl = 'https://api.mclo.gs/1/raw/' + id;

  let log: string;
  try {
    const f = await fetch(apiUrl);
    if (f.status != 200) {
      throw 'nope';
    }
    log = await f.text();
  } catch (err) {
    console.log('Log analyze fail', err);
    return null;
  }
  console.log(apiUrl);
  const embed = new MessageEmbed()
    .setTitle('Log analyzer')
    .setColor('DARK_GREEN')
    .setDescription(`Analysis of ${link} [${apiUrl}] [ID: ${id}]`);
  for (let i in analyzers) {
    const analyzer = analyzers[i];
    const out = await analyzer(log);
    if (out) embed.addField(out[0], out[1]);
  }
  if (embed.fields[0]) return embed;
  else {
    embed.addField('Analyze failed', 'No issues found automatically');
    return embed;
  }
}
