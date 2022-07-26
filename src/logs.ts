import { getLatestPolyMCVersion } from './utils/remoteVersions';
import { EmbedBuilder } from 'discord.js';

// log providers
import { readMcLogs } from './logproviders/mclogs';
import { read0x0 } from './logproviders/0x0';
import { readPasteGG } from './logproviders/pastegg';
import { readHastebin } from './logproviders/haste';
import { COLORS } from './constants';

type Analyzer = (text: string) => Promise<Array<string> | null>;
type LogProvider = (text: string) => Promise<null | string>;

const javaAnalyzer: Analyzer = async (text) => {
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

const versionAnalyzer: Analyzer = async (text) => {
  const vers = text.match(/PolyMC version: [0-9].[0-9].[0-9]/g);
  if (vers && vers[0]) {
    const latest = await getLatestPolyMCVersion();
    const current = vers[0].replace('PolyMC version: ', '');
    if (current < latest) {
      return [
        'Outdated PolyMC',
        `Your installed version is ${current}, while the newest version is ${latest}.\nPlease update, for more info see https://polymc.org/download/`,
      ];
    }
  }
  return null;
};

const flatpakNvidiaAnalyzer: Analyzer = async (text) => {
  if (
    text.includes('org.lwjgl.LWJGLException: Could not choose GLX13 config') ||
    text.includes(
      'GLFW error 65545: GLX: Failed to find a suitable GLXFBConfig'
    )
  ) {
    return [
      'Outdated Nvidia Flatpak Driver',
      `The Nvidia driver for flatpak is outdated.\nPlease run \`flatpak update\` to fix this issue. If that does not solve it, please wait until the driver is added to Flathub and run it again.`,
    ];
  }
  return null;
};

const forgeJavaAnalyzer: Analyzer = async (text) => {
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

const intelHDAnalyzer: Analyzer = async (text) => {
  if (text.includes('org.lwjgl.LWJGLException: Pixel format not accelerated')) {
    return [
      'Intel HD Windows 10',
      "Your drivers don't support windows 10 officially\nSee https://polymc.org/wiki/getting-started/installing-java/#a-note-about-intel-hd-20003000-on-windows-10 for more info",
    ];
  }
  return null;
};

const macOSNSWindowAnalyzer: Analyzer = async (text) => {
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

const quiltFabricInternalsAnalyzer: Analyzer = async (text) => {
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

const oomAnalyzer: Analyzer = async (text) => {
  if (text.includes('java.lang.OutOfMemoryError')) {
    return [
      'Out of Memory',
      'Allocating more RAM to your instance could help prevent this crash.',
    ];
  }
  return null;
};

const javaOptionsAnalyzer: Analyzer = async (text) => {
  const r1 = /Unrecognized VM option '(\w*)'/;
  const r2 = /Unrecognized option: \w*/;
  const m1 = text.match(r1);
  const m2 = text.match(r2);
  if (m1) {
    return [
      'Wrong Java Arguments',
      `Remove \`-XX:${m1[1]}\` from your Java arguments.`,
    ];
  }
  if (m2) {
    return [
      'Wrong Java Arguments',
      `Remove \`${m2[1]}\` from your Java arguments.`,
    ];
  }
  return null;
};

const shenadoahGCAnalyzer: Analyzer = async (text) => {
  if (text.includes("Unrecognized VM option 'UseShenandoahGC'")) {
    return [
      "Java 8 doesn't support ShenandoahGC",
      'Remove `UseShenandoahGC` from your Java Arguments',
    ];
  }
  return null;
};

const optifineAnalyzer: Analyzer = async (text) => {
  const matchesOpti = text.match(/\[✔️\] OptiFine_[\w,.]*/);
  const matchesOptiFabric = text.match(/\[✔️\] optifabric-[\w,.]*/);
  if (matchesOpti || matchesOptiFabric) {
    return [
      'Possible Optifine Problems',
      'OptiFine is known to cause problems when paired with other mods. Try to disable OptiFine and see if the issue persists.\nCheck `!optifine` for more info & alternatives you can use.',
    ];
  }
  return null;
};

const forge119IssueAnalyzer: Analyzer = async (text) => {
  const matches = text.match(
    /Caused by: java.lang.RuntimeException: java.lang.reflect.InvocationTargetException\n.at MC-BOOTSTRAP\/cpw.mods.modlauncher@[0-9]\.[0-9]\.[0-9]\/cpw.mods.modlauncher.LaunchServiceHandlerDecorator.launch\(LaunchServiceHandlerDecorator.java:[0-9]*\)/
  );
  if (matches) {
    return [
      'Update your Forge 1.19',
      'Update to the latest version of Forge 1.19, older ones have issues with the new split natives system, causing this crash.',
    ];
  }
  return null;
};

const tempM1Analyzer: Analyzer = async (text) => {
  const lwjglFail = text.includes('[LWJGL] Failed to load a library');
  const m1 =
    (text.includes('natives-macos') || text.includes('natives-osx')) &&
    (text.includes('aarch64') || text.includes('arm64'));

  if (lwjglFail && m1) {
    return [
      'M1 issues',
      "PolyMC doesn't support Apple M1 for sub-1.19 versions yet. Use ManyMC https://github.com/MinecraftMachina/ManyMC.",
    ];
  }

  return null;
};

const analyzers: Analyzer[] = [
  javaAnalyzer,
  versionAnalyzer,
  flatpakNvidiaAnalyzer,
  forgeJavaAnalyzer,
  intelHDAnalyzer,
  macOSNSWindowAnalyzer,
  quiltFabricInternalsAnalyzer,
  oomAnalyzer,
  shenadoahGCAnalyzer,
  optifineAnalyzer,
  forge119IssueAnalyzer,
  tempM1Analyzer,
  javaOptionsAnalyzer,
];

const providers: LogProvider[] = [
  readMcLogs,
  read0x0,
  readPasteGG,
  readHastebin,
];

export async function parseLog(s: string): Promise<EmbedBuilder | null> {
  if (s.includes('https://pastebin.com/')) {
    const embed = new EmbedBuilder()
      .setTitle('pastebin.com detected')
      .setDescription(
        'Please use https://mclo.gs or another paste provider and send logs using the Log Upload feature in PolyMC. (See ?log)'
      )
      .setColor(COLORS.red);
    return embed;
  }

  let log = '';
  for (const i in providers) {
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
  const embed = new EmbedBuilder().setTitle('Log analysis');

  let thereWasAnIssue = false;
  for (const i in analyzers) {
    const Analyzer = analyzers[i];
    const out = await Analyzer(log);
    if (out) {
      embed.addFields({ name: out[0], value: out[1] });
      thereWasAnIssue = true;
    }
  }

  if (thereWasAnIssue) {
    embed.setColor(COLORS.red);
    return embed;
  } else {
    embed.setColor(COLORS.green);
    embed.addFields({
      name: 'Analyze failed',
      value: 'No issues found automatically',
    });

    return embed;
  }
}
