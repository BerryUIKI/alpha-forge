/**
 * English error messages catalog.
 * Maps stable error codes from Rust to user-friendly messages.
 */

export const errors = {
  // Generic error fallback
  unknownError: "An unexpected error occurred. Please try again.",
  unknownErrorDescription: "If the problem persists, please check the application logs.",

  // Internal error
  errorInternal: "Internal error",
  errorInternalDescription: "An internal application error occurred. Please restart the application.",

  // Not found
  errorNotFound: "Not found",
  errorNotFoundDescription: "The requested resource was not found.",

  // Validation error
  errorValidation: "Validation error",
  errorValidationDescription: "The provided data is invalid. Please check your input and try again.",

  // Permission denied
  errorPermissionDenied: "Permission denied",
  errorPermissionDeniedDescription: "You do not have permission to perform this action.",

  // Timeout
  errorTimeout: "Operation timed out",
  errorTimeoutDescription: "The operation took too long to complete. Please try again.",

  // Recoverable error hint
  errorRecoverableHint: "This error can be recovered. Please try again.",

  // Non-recoverable error hint
  errorNonRecoverableHint: "This error requires application restart or administrator intervention.",
} as const;

export type ErrorKey = keyof typeof errors;