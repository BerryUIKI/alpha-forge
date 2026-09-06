/**
 * Unified error handling utilities for React frontend.
 * Provides type-safe error parsing and processing.
 */

import { processErrorResponse, type ErrorCode, type ErrorResponse } from "@/lib/i18n/errorMessages";
import type { Locale } from "@/lib/i18n/locale";

/**
 * Application error types that can occur in the frontend.
 */
export type AppError =
  | { type: "backend"; response: ErrorResponse }
  | { type: "network"; message: string }
  | { type: "validation"; field?: string; message: string }
  | { type: "not_found"; resource?: string }
  | { type: "unknown"; cause: unknown };

/**
 * Parse an unknown error into a structured AppError type.
 * This function safely handles any error type and converts it to a known structure.
 *
 * @param error - Unknown error from try/catch or Promise rejection
 * @returns Structured AppError
 *
 * @example
 * try {
 *   await someAsyncOperation();
 * } catch (error) {
 *   const appError = parseError(error);
 *   if (appError.type === "backend") {
 *     console.log("Backend error code:", appError.response.code);
 *   }
 * }
 */
export function parseError(error: unknown): AppError {
  // Handle null/undefined
  if (error == null) {
    return { type: "unknown", cause: error };
  }

  // Handle ErrorResponse from backend
  if (isErrorResponse(error)) {
    return { type: "backend", response: error };
  }

  // Handle Error objects
  if (error instanceof Error) {
    // Check for network errors
    if (isNetworkError(error)) {
      return { type: "network", message: error.message };
    }

    // Check for validation errors
    if (isValidationError(error)) {
      return { type: "validation", message: error.message };
    }

    // Generic Error - treat as backend error with default code
    return {
      type: "backend",
      response: {
        code: "INTERNAL",
        message: error.message,
        recoverable: false,
      },
    };
  }

  // Handle string errors
  if (typeof error === "string") {
    return {
      type: "backend",
      response: {
        code: "INTERNAL",
        message: error,
        recoverable: false,
      },
    };
  }

  // Handle object with error properties
  if (hasErrorMessage(error)) {
    return {
      type: "backend",
      response: {
        code: typeof error.code === "string" ? error.code : "INTERNAL",
        message: error.message,
        recoverable: typeof error.recoverable === "boolean" ? error.recoverable : false,
      },
    };
  }

  // Fallback to unknown
  return { type: "unknown", cause: error };
}

/**
 * Check if a value is a non-null object record.
 */
function isRecord(val: unknown): val is Record<string, unknown> {
  return typeof val === "object" && val !== null;
}

/**
 * Type guard to check if an error is an ErrorResponse.
 */
function isErrorResponse(error: unknown): error is ErrorResponse {
  return (
    isRecord(error) &&
    typeof error.code === "string" &&
    typeof error.message === "string" &&
    typeof error.recoverable === "boolean"
  );
}

/**
 * Type guard to check if an error is a network error.
 */
function isNetworkError(error: Error): boolean {
  const networkErrorMessages = [
    "Failed to fetch",
    "Network request failed",
    "Network error",
    "fetch failed",
    "ECONNREFUSED",
    "ENOTFOUND",
  ];

  return networkErrorMessages.some((msg) =>
    error.message.toLowerCase().includes(msg.toLowerCase())
  );
}

/**
 * Type guard to check if an error is a validation error.
 */
function isValidationError(error: Error): boolean {
  return (
    error.name === "ValidationError" ||
    error.message.toLowerCase().includes("validation") ||
    error.message.toLowerCase().includes("invalid")
  );
}

/**
 * Type guard to check if an object has error message.
 */
function hasErrorMessage(error: unknown): error is Record<string, unknown> & { message: string } {
  return isRecord(error) && typeof error.message === "string";
}

/**
 * Process an error for display with localization.
 * This is a type-safe wrapper around processErrorResponse.
 *
 * @param locale - Current locale
 * @param error - Unknown error from mutation or async operation
 * @returns Localized error messages
 *
 * @example
 * // In a hook
 * export function useWorkspace() {
 *   const mutation = useMutation({
 *     onError: (error) => {
 *       const messages = processAppError("zh-CN", error);
 *       setError(messages.title);
 *     },
 *   });
 * }
 */
export function processAppError(locale: Locale, error: unknown) {
  const appError = parseError(error);

  if (appError.type === "backend") {
    return processErrorResponse(locale, appError.response);
  }

  // Handle non-backend errors
  const fallbackResponse: ErrorResponse = {
    code: appError.type.toUpperCase().replace("_", "_") as ErrorCode,
    message: appError.type === "network"
      ? appError.message
      : appError.type === "validation"
      ? appError.message
      : appError.type === "not_found"
      ? appError.resource || "Resource not found"
      : "Unknown error",
    recoverable: appError.type !== "unknown",
  };

  return processErrorResponse(locale, fallbackResponse);
}

/**
 * Get error code from an unknown error.
 *
 * @param error - Unknown error
 * @returns Error code string or "UNKNOWN"
 */
export function getErrorCode(error: unknown): string {
  const appError = parseError(error);

  if (appError.type === "backend") {
    return appError.response.code;
  }

  return appError.type.toUpperCase();
}

/**
 * Check if an error is recoverable.
 *
 * @param error - Unknown error
 * @returns True if the error can be recovered by retrying
 */
export function isRecoverable(error: unknown): boolean {
  const appError = parseError(error);

  if (appError.type === "backend") {
    return appError.response.recoverable;
  }

  // Network errors are usually recoverable
  if (appError.type === "network") {
    return true;
  }

  // Unknown errors are not recoverable
  return false;
}

