/** Mock of @tauri-apps/plugin-os. */
export type Platform = "macos" | "windows" | "linux";

export function platform(): Platform {
  return "macos";
}

export function type(): Platform {
  return "macos";
}

export async function locale(): Promise<string> {
  return "en";
}

export function version(): string {
  return "14.0.0";
}

export function family(): "unix" | "windows" {
  return "unix";
}

export function arch(): string {
  return "aarch64";
}
