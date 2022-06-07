import { getLatest } from './version';
import { MessageEmbed } from 'discord.js';
// log providers
import { readMcLogs } from './logproviders/mclogs';
import { read0x0 } from './logproviders/0x0';
import { readPasteGG } from './logproviders/pastegg';
import { readHastebin } from './logproviders/haste';

const reg = /https\:\/\/mclo.gs\/[^ ]*/g;

type analyzer = (text: string) => Promise<Array<string> | null>;
type logProvider = (text: string) => Promise<null | string>;

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
      'Wrong Java Version',
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
        'Outdated PolyMC',
        `Your installed version is ${current}, while the newest version is ${latest}.\nPlease update, for more info see https://polymc.org/download/`,
      ];
    }
  }
  return null;
};

const flatpakNvidiaAnalyzer: analyzer = async (text) => {
  if (
    text.includes('org.lwjgl.LWJGLException: Could not choose GLX13 config')
  ) {
    return [
      'Outdated Nvidia Flatpak Driver',
      `The Nvidia driver for flatpak is outdated.\nPlease run \`flatpak update\` to fix this issue. If that does not solve it, please wait until the driver is added to Flathub and run it again.`,
    ];
  }
  return null;
};

const forgeJavaAnalyzer: analyzer = async (text) => {
  if (
    text.includes(
      'java.lang.NoSuchMethodError: sun.security.util.ManifestEntryVerifier.<init>(Ljava/util/jar/Manifest;)V'
    )
  ) {
    return [
      'Forge Java Bug',
      'Old versions of Forge crash with Java 8u321+.\nTo fix this, update forge to the latest version via the Versions tab \n(right click on Forge, click Change Version, and choose the latest one)\nAlternatively, you can download 8u312 or lower. See [archive](https://github.com/adoptium/temurin8-binaries/releases/tag/jdk8u312-b07)',
    ];
  }
  return null;
};

const intelHDAnalyzer: analyzer = async (text) => {
  if (text.includes('org.lwjgl.LWJGLException: Pixel format not accelerated')) {
    return [
      'Intel HD Windows 10',
      "Your drivers don't support windows 10 officially\nSee https://polymc.org/wiki/getting-started/installing-java/#a-note-about-intel-hd-20003000-on-windows-10 for more info",
    ];
  }
  return null;
};

const macOSNSWindowAnalyzer: analyzer = async (text) => {
  if (
    text.includes(
      "Terminating app due to uncaught exception 'NSInternalInconsistencyException'"
    )
  ) {
    return [
      'MacOS NSInternalInconsistencyException',
      'You need to downgrade your Java 8 version. See https://polymc.org/wiki/getting-started/installing-java/#older-minecraft-on-macos',
    ];
  }
  return null;
};

const quiltFabricInternalsAnalyzer: analyzer = async (text) => {
  const base = 'Caused by: java.lang.ClassNotFoundException: ';
  if (
    text.includes(base + 'net.fabricmc.fabric.impl') ||
    text.includes(base + 'net.fabricmc.fabric.mixin') ||
    text.includes(base + 'net.fabricmc.loader.impl') ||
    text.includes(base + 'net.fabricmc.loader.mixin') ||
    text.includes(
      'org.quiltmc.loader.impl.FormattedException: java.lang.NoSuchMethodError:'
    )
  ) {
    return [
      'Fabric Internal Access',
      `The mod you are using is using fabric internals that are not meant to be used by anything but the loader itself.
      Those mods break both on Quilt and with fabric updates.
      If you're using fabric, downgrade your fabric loader could work, on Quilt you can try updating to the latest beta version, but there's nothing much to do unless the mod author stops using them.`,
    ];
  }
  return null;
};

const oomAnalyzer: analyzer = async (text) => {
  if (text.includes('java.lang.OutOfMemoryError: Java heap space')) {
    return [
      'Out of Memory',
      'Allocate more RAM to your instance to prevent this crash.',
    ];
  }
  return null;
};

const analyzers: analyzer[] = [
  javaAnalyzer,
  versionAnalyzer,
  flatpakNvidiaAnalyzer,
  forgeJavaAnalyzer,
  intelHDAnalyzer,
  macOSNSWindowAnalyzer,
  quiltFabricInternalsAnalyzer,
  oomAnalyzer,
];

const providers: logProvider[] = [
  readMcLogs,
  read0x0,
  readPasteGG,
  readHastebin,
];

export async function parseLog(s: string): Promise<MessageEmbed | null> {
  if (s.includes('https://pastebin.com/')) {
    const embed = new MessageEmbed()
      .setTitle('pastebin.com detected')
      .setDescription(
        'Please use https://mclo.gs or another paste provider and send logs using the Log Upload feature in PolyMC. (See !log)'
      )
      .setColor('DARK_RED');
    return embed;
  }
  let log: string = '';
  for (let i in providers) {
    const provider = providers[i];
    const res = await provider(s);
    if (res) {
      log = res;
      break;
    } else {
      continue;
    }
  }
  if (!log) return null;

  const embed = new MessageEmbed()
    .setTitle('Log analysis')
    .setColor('DARK_GREEN');
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
