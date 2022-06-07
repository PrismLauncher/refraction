const reg = /https\:\/\/mclo.gs\/[^ ]*/;

export async function readMcLogs(s: string): Promise<null | string> {
  const r = s.match(reg);
  if (r == null || !r[0]) return null;
  const link = r[0];
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
  return log;
}
