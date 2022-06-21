const reg = /https:\/\/0x0.st\/\w*.\w*/;

export async function read0x0(s: string): Promise<null | string> {
  const r = s.match(reg);
  if (r == null || !r[0]) return null;
  const link = r[0];
  let log: string;
  try {
    const f = await fetch(link);
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
