/** Mock of @tauri-apps/api/webviewWindow (only used as a type in bindings.ts). */
export class WebviewWindow {
  constructor(public label: string) {}
}

export function getCurrentWebviewWindow(): WebviewWindow {
  return new WebviewWindow("main");
}
