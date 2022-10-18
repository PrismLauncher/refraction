interface MetaPackage {
  formatVersion: number;
  name: string;
  recommended: string[];
  uid: string;
}

interface SimplifiedGHReleases {
  tag_name: string;
}

// TODO: caching
export async function getLatestMinecraftVersion(): Promise<string> {
  const f = await fetch(
    'https://meta.PrismLauncher.org/v1/net.minecraft/package.json'
  );

  const minecraft = (await f.json()) as MetaPackage;
  return minecraft.recommended[0];
}

// TODO: caching
export async function getLatestPrismLauncherVersion(): Promise<string> {
  const f = await fetch('https://api.github.com/repos/PrismLauncher/PrismLauncher/releases');
  const versions = (await f.json()) as SimplifiedGHReleases[];

  return versions[0].tag_name;
}
