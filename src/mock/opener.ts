/** Mock of @tauri-apps/plugin-opener. */
export async function openUrl(url: string): Promise<void> {
  try {
    window.open(url, "_blank", "noopener");
  } catch {
    // Ignore — opening in a browser isn't essential for the UI demo.
  }
}

export async function openPath(): Promise<void> {}

export async function revealItemInDir(): Promise<void> {}
