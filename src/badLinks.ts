import { FiltersEngine, Request } from '@cliqz/adblocker';
import fetch from 'node-fetch';

let engine: FiltersEngine;

const init = async () => {
  if (engine) return;

  console.log('initializing FiltersEngine');

  engine = await FiltersEngine.fromLists(
    fetch,
    [
      'https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/badware.txt',
      'https://raw.githubusercontent.com/JonDoeBeep/Phishing.Database/master/phishing-domains/output/domains/ACTIVE/list.txt',
      'https://malware-filter.gitlab.io/malware-filter/phishing-filter.txt',
    ],
    {
      enableInMemoryCache: true,
      enableOptimizations: true,
      enableCompression: true,
    }
  );
};

export const isBad = async (url: string) => {
  await init();

  const { match } = engine.match(
    Request.fromRawDetails({
      type: 'mainFrame',
      url,
    })
  );

  console.log('Testing URL', url, match);

  return match;
};
