/** Mock of @tauri-apps/plugin-updater. */
export async function check(): Promise<null> {
  // No update available in the mock.
  return null;
}

export type Update = {
  available: boolean;
  version: string;
  downloadAndInstall: () => Promise<void>;
};
