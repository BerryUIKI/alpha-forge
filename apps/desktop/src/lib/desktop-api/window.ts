import { getCurrentWindow } from "@tauri-apps/api/window";

/**
 * Privileged main-window controls used by the custom title bar.
 * Keeping them here prevents layout components from reaching into Tauri APIs.
 */
export async function minimize(): Promise<void> {
  await getCurrentWindow().minimize();
}

export async function toggleMaximize(): Promise<void> {
  await getCurrentWindow().toggleMaximize();
}

export async function close(): Promise<void> {
  await getCurrentWindow().close();
}
