import { describe, it, expect } from "vitest";
import { getAppStartupInfo, verifyStartupEnvironment } from "./startup";

describe("startup coordinator", () => {
  it("collects runtime startup diagnostics", () => {
    const info = getAppStartupInfo();
    expect(info).toHaveProperty("isTauri");
    expect(info).toHaveProperty("platform");
    expect(info).toHaveProperty("userAgent");
    expect(typeof info.startedAt).toBe("string");
  });

  it("verifies healthy startup environment", () => {
    const check = verifyStartupEnvironment();
    expect(check.ok).toBe(true);
    expect(check.errors).toEqual([]);
  });
});
