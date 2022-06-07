const reg = /https\:\/\/hst.sh\/[\w]*/;

export async function readHastebin(s: string): Promise<string | null> {
  const r = s.match(reg);
  if (r == null || !r[0]) return null;
  const link = r[0];
  const id = link.replace('https://hst.sh/', '');
  if (!id) return null;
  let log: string;
  try {
    const f = await fetch(`https://hst.sh/raw/${id}`);
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
