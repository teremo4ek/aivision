/** Mock of @tauri-apps/plugin-dialog. */
export async function ask(): Promise<boolean> {
  return true;
}

export async function confirm(): Promise<boolean> {
  return true;
}

export async function message(): Promise<void> {}

export async function open(): Promise<string | string[] | null> {
  return null;
}

export async function save(): Promise<string | null> {
  return null;
}
