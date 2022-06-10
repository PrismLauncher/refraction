import fetch from 'node-fetch';

interface SimplifiedMetaVersion {
  recommended: boolean;
  type: 'release' | 'snapshot';
  version: string;
}

export const getLatestMinecraft = async () => {
  const meta = (await fetch(
    'https://meta.polymc.org/v1/net.minecraft/index.json'
  ).then((r) => r.json())) as { versions: SimplifiedMetaVersion[] };

  return meta.versions.filter((v) => v.recommended)[0].version;
};
