/**
 * Error code to message key mapping.
 * Maps stable error codes from Rust backend to i18n message keys.
 */

import { translate, type Locale, type MessageKey } from "./locale";

/**
 * Stable error codes from Rust backend.
 * These codes must remain stable across versions.
 */
export type ErrorCode =
  | "INTERNAL"
  | "NOT_FOUND"
  | "VALIDATION"
  | "PERMISSION_DENIED"
  | "TIMEOUT";

/**
 * Error response structure from Rust backend.
 */
export interface ErrorResponse {
  code: string;
  message: string;
  recoverable: boolean;
}

/**
 * Map error codes to i18n message keys.
 */
function errorCodeToMessageKey(code: ErrorCode): MessageKey {
  // Convert NOT_FOUND -> errorNotFound (proper camelCase)
  const parts = code.split("_");
  const capitalized = parts.map((p, i) => 
    i === 0 ? p.toLowerCase() : p.charAt(0).toUpperCase() + p.slice(1).toLowerCase()
  ).join("");
  const key = `error${capitalized.charAt(0).toUpperCase() + capitalized.slice(1)}`;
  return key as MessageKey;
}

/**
 * Get localized error message for a given error code.
 * 
 * @param locale - Current locale
 * @param errorCode - Stable error code from backend
 * @returns Localized error message
 * 
 * @example
 * const message = getLocalizedErrorMessage("zh-CN", "NOT_FOUND");
 * // Returns: "未找到"
 */
export function getLocalizedErrorMessage(locale: Locale, errorCode: ErrorCode): string {
  const key = errorCodeToMessageKey(errorCode);
  return translate(locale, key);
}

/**
 * Get localized error description for a given error code.
 * 
 * @param locale - Current locale
 * @param errorCode - Stable error code from backend
 * @returns Localized error description
 */
export function getLocalizedErrorDescription(locale: Locale, errorCode: ErrorCode): string {
  const baseKey = errorCodeToMessageKey(errorCode);
  const key = `${baseKey}Description` as MessageKey;
  return translate(locale, key);
}

/**
 * Get localized error hint based on recoverable flag.
 * 
 * @param locale - Current locale
 * @param recoverable - Whether the error is recoverable
 * @returns Localized error hint
 */
export function getLocalizedErrorHint(locale: Locale, recoverable: boolean): string {
  const key = recoverable ? "errorRecoverableHint" : "errorNonRecoverableHint";
  return translate(locale, key);
}

/**
 * Process an error response from the backend and return localized messages.
 * 
 * @param locale - Current locale
 * @param error - Error response from backend
 * @returns Object with localized error messages
 * 
 * @example
 * const error = { code: "NOT_FOUND", message: "...", recoverable: true };
 * const localized = processErrorResponse("zh-CN", error);
 * // Returns: { title: "未找到", description: "请求的资源未找到。", hint: "此错误可恢复。请重试。" }
 */
export function processErrorResponse(locale: Locale, error: ErrorResponse) {
  // Check if error code is known
  const knownCodes: ErrorCode[] = ["INTERNAL", "NOT_FOUND", "VALIDATION", "PERMISSION_DENIED", "TIMEOUT"];
  const isKnownCode = knownCodes.includes(error.code as ErrorCode);

  if (!isKnownCode) {
    // Unknown error code - use generic message
    return {
      title: translate(locale, "unknownError"),
      description: translate(locale, "unknownErrorDescription"),
      hint: getLocalizedErrorHint(locale, error.recoverable),
      originalCode: error.code,
      originalMessage: error.message,
    };
  }

  const errorCode = error.code as ErrorCode;
  return {
    title: getLocalizedErrorMessage(locale, errorCode),
    description: getLocalizedErrorDescription(locale, errorCode),
    hint: getLocalizedErrorHint(locale, error.recoverable),
    originalCode: error.code,
    originalMessage: error.message,
  };
}

/**
 * Type guard to check if an error code is known.
 */
export function isKnownErrorCode(code: string): code is ErrorCode {
  const knownCodes: ErrorCode[] = ["INTERNAL", "NOT_FOUND", "VALIDATION", "PERMISSION_DENIED", "TIMEOUT"];
  return knownCodes.includes(code as ErrorCode);
}