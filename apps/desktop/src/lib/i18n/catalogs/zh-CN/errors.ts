/**
 * Simplified Chinese error messages catalog.
 * Maps stable error codes from Rust to user-friendly messages.
 */

export const errors = {
  // Generic error fallback
  unknownError: "发生了意外错误。请重试。",
  unknownErrorDescription: "如果问题持续存在，请检查应用程序日志。",

  // Internal error
  errorInternal: "内部错误",
  errorInternalDescription: "发生了应用程序内部错误。请重启应用程序。",

  // Not found
  errorNotFound: "未找到",
  errorNotFoundDescription: "请求的资源未找到。",

  // Validation error
  errorValidation: "验证错误",
  errorValidationDescription: "提供的数据无效。请检查您的输入后重试。",

  // Permission denied
  errorPermissionDenied: "权限拒绝",
  errorPermissionDeniedDescription: "您没有权限执行此操作。",

  // Timeout
  errorTimeout: "操作超时",
  errorTimeoutDescription: "操作耗时过长。请重试。",

  // Recoverable error hint
  errorRecoverableHint: "此错误可恢复。请重试。",

  // Non-recoverable error hint
  errorNonRecoverableHint: "此错误需要重启应用程序或管理员介入。",
} as const;

export type ErrorKey = keyof typeof errors;