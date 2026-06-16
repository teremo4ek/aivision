/** Mock of @tauri-apps/plugin-fs. */
export async function readFile(): Promise<Uint8Array> {
  return new Uint8Array();
}

export async function readTextFile(): Promise<string> {
  return "";
}

export async function exists(): Promise<boolean> {
  return false;
}
