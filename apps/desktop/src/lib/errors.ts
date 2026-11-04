// Shared error helpers.

/**
 * Extracts a user-facing message from an unknown error, including the
 * `{ code, message, recoverable }` payloads returned by Tauri commands.
 */
export function getErrorMessage(error: unknown, fallback = "Unexpected error"): string {
  if (error instanceof Error) {
    return error.message;
  }
  if (typeof error === "object" && error !== null && "message" in error) {
    const message = (error as { message?: unknown }).message;
    if (typeof message === "string" && message.trim()) {
      return message;
    }
  }
  if (typeof error === "string" && error.trim()) {
    return error;
  }
  return fallback;
}
