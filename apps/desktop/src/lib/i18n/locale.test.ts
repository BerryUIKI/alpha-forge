import { describe, expect, it } from "vitest";
import { DEFAULT_LOCALE, formatMessage, parseLocale, translate } from "./locale";

describe("locale helpers", () => {
  it("uses Chinese as the launch default and accepts the supported locales", () => {
    expect(DEFAULT_LOCALE).toBe("zh-CN");
    expect(parseLocale("en")).toBe("en");
    expect(parseLocale("unsupported")).toBe("zh-CN");
  });

  it("returns localized messages and formats message values", () => {
    expect(translate("zh-CN", "settings")).toBe("设置");
    expect(formatMessage(translate("en", "backupCreated"), { path: "/tmp/backup.db" })).toBe(
      "Backup created at /tmp/backup.db",
    );
  });
});
