import { describe, expect, it, vi } from "vitest";
import { DEFAULT_LOCALE, formatMessage, parseLocale, translate, detectSystemLocale } from "./locale";

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

describe("detectSystemLocale", () => {
  it("detects exact locale match", () => {
    const originalLanguages = navigator.languages;
    
    vi.spyOn(navigator, "languages", "get").mockReturnValue(["zh-CN", "zh"]);
    expect(detectSystemLocale()).toBe("zh-CN");

    vi.spyOn(navigator, "languages", "get").mockReturnValue(["en-US", "en"]);
    expect(detectSystemLocale()).toBe("en");

    vi.spyOn(navigator, "languages", "get").mockReturnValue(originalLanguages as any);
  });

  it("matches language-only prefix", () => {
    const originalLanguages = navigator.languages;
    
    vi.spyOn(navigator, "languages", "get").mockReturnValue(["zh-TW"]);
    expect(detectSystemLocale()).toBe("zh-CN"); // Matches "zh" prefix

    vi.spyOn(navigator, "languages", "get").mockReturnValue(["en-GB"]);
    expect(detectSystemLocale()).toBe("en"); // Matches "en" prefix

    vi.spyOn(navigator, "languages", "get").mockReturnValue(originalLanguages as any);
  });

  it("falls back to DEFAULT_LOCALE for unsupported languages", () => {
    const originalLanguages = navigator.languages;
    
    vi.spyOn(navigator, "languages", "get").mockReturnValue(["fr", "de", "es"]);
    expect(detectSystemLocale()).toBe(DEFAULT_LOCALE);

    vi.spyOn(navigator, "languages", "get").mockReturnValue(originalLanguages as any);
  });

  it("uses first matching locale in preference order", () => {
    const originalLanguages = navigator.languages;
    
    vi.spyOn(navigator, "languages", "get").mockReturnValue(["fr", "zh-CN", "en"]);
    expect(detectSystemLocale()).toBe("zh-CN"); // First match

    vi.spyOn(navigator, "languages", "get").mockReturnValue(originalLanguages as any);
  });
});
