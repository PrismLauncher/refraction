let cachedVer: string;
let cachedTimestamp: number;

export async function getLatest(): Promise<string> {
  if (cachedVer && Date.now() - cachedTimestamp < 600000) return cachedVer; // 10min
  const f = await fetch('https://api.github.com/repos/PolyMC/PolyMC/releases');
  const versions = await f.json();
  cachedVer = versions[0].tag_name;
  cachedTimestamp = Date.now();
  return versions[0].tag_name;
}
