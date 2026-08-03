import { describe, expect, it } from "vitest";
import {
  getLocalizedErrorMessage,
  getLocalizedErrorDescription,
  processErrorResponse,
  isKnownErrorCode,
  type ErrorCode,
} from "./errorMessages";

describe("errorMessages", () => {
  describe("getLocalizedErrorMessage", () => {
    it("returns Chinese message for zh-CN locale", () => {
      expect(getLocalizedErrorMessage("zh-CN", "NOT_FOUND")).toBe("未找到");
      expect(getLocalizedErrorMessage("zh-CN", "INTERNAL")).toBe("内部错误");
      expect(getLocalizedErrorMessage("zh-CN", "VALIDATION")).toBe("验证错误");
    });

    it("returns English message for en locale", () => {
      expect(getLocalizedErrorMessage("en", "NOT_FOUND")).toBe("Not found");
      expect(getLocalizedErrorMessage("en", "INTERNAL")).toBe("Internal error");
      expect(getLocalizedErrorMessage("en", "VALIDATION")).toBe("Validation error");
    });
  });

  describe("getLocalizedErrorDescription", () => {
    it("returns Chinese description for zh-CN locale", () => {
      expect(getLocalizedErrorDescription("zh-CN", "NOT_FOUND")).toBe("请求的资源未找到。");
      expect(getLocalizedErrorDescription("zh-CN", "TIMEOUT")).toBe("操作耗时过长。请重试。");
    });

    it("returns English description for en locale", () => {
      expect(getLocalizedErrorDescription("en", "NOT_FOUND")).toBe("The requested resource was not found.");
      expect(getLocalizedErrorDescription("en", "TIMEOUT")).toBe("The operation took too long to complete. Please try again.");
    });
  });

  describe("processErrorResponse", () => {
    it("processes known error codes correctly", () => {
      const error = {
        code: "NOT_FOUND",
        message: "Workspace not found",
        recoverable: true,
      };

      const result = processErrorResponse("zh-CN", error);

      expect(result.title).toBe("未找到");
      expect(result.description).toBe("请求的资源未找到。");
      expect(result.hint).toBe("此错误可恢复。请重试。");
      expect(result.originalCode).toBe("NOT_FOUND");
      expect(result.originalMessage).toBe("Workspace not found");
    });

    it("returns generic message for unknown error codes", () => {
      const error = {
        code: "UNKNOWN_CODE",
        message: "Something went wrong",
        recoverable: false,
      };

      const result = processErrorResponse("zh-CN", error);

      expect(result.title).toBe("发生了意外错误。请重试。");
      expect(result.originalCode).toBe("UNKNOWN_CODE");
    });

    it("returns correct hint for recoverable errors", () => {
      const recoverableError = {
        code: "VALIDATION",
        message: "Invalid input",
        recoverable: true,
      };

      const result = processErrorResponse("en", recoverableError);
      expect(result.hint).toBe("This error can be recovered. Please try again.");
    });

    it("returns correct hint for non-recoverable errors", () => {
      const nonRecoverableError = {
        code: "INTERNAL",
        message: "Critical failure",
        recoverable: false,
      };

      const result = processErrorResponse("en", nonRecoverableError);
      expect(result.hint).toBe("This error requires application restart or administrator intervention.");
    });
  });

  describe("isKnownErrorCode", () => {
    it("returns true for known error codes", () => {
      expect(isKnownErrorCode("INTERNAL")).toBe(true);
      expect(isKnownErrorCode("NOT_FOUND")).toBe(true);
      expect(isKnownErrorCode("VALIDATION")).toBe(true);
      expect(isKnownErrorCode("PERMISSION_DENIED")).toBe(true);
      expect(isKnownErrorCode("TIMEOUT")).toBe(true);
    });

    it("returns false for unknown error codes", () => {
      expect(isKnownErrorCode("UNKNOWN")).toBe(false);
      expect(isKnownErrorCode("random")).toBe(false);
      expect(isKnownErrorCode("")).toBe(false);
    });
  });
});