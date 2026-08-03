import { describe, expect, it } from "vitest";
import {
  formatCurrency,
  formatDate,
  formatNumber,
  formatPercent,
  formatRelativeTime,
  parseNumber,
} from "./formatters";

describe("formatDate", () => {
  it("formats dates in Chinese locale", () => {
    const date = new Date("2026-08-03T12:00:00Z");
    const result = formatDate("zh-CN", date);
    expect(result).toMatch(/2026/);
  });

  it("formats dates in English locale", () => {
    const date = new Date("2026-08-03T12:00:00Z");
    const result = formatDate("en", date);
    expect(result).toMatch(/2026/);
  });

  it("includes time when requested", () => {
    const date = new Date("2026-08-03T12:00:00Z");
    const result = formatDate("en", date, { includeTime: true });
    expect(result.length).toBeGreaterThan(10);
  });

  it("handles ISO string input", () => {
    const result = formatDate("en", "2026-08-03T12:00:00Z");
    expect(result).toMatch(/2026/);
  });

  it("handles timestamp input", () => {
    const timestamp = 1722686400000; // 2024-08-03
    const result = formatDate("en", timestamp);
    expect(result).toMatch(/2024/);
  });

  it("returns empty string for invalid date", () => {
    const result = formatDate("en", new Date("invalid"));
    expect(result).toBe("");
  });
});

describe("formatRelativeTime", () => {
  it("formats past relative time in Chinese", () => {
    // Chinese uses "前天" (day before yesterday) for -2 days with numeric: "auto"
    const result = formatRelativeTime("zh-CN", -2, "day");
    expect(result).toBeDefined();
    expect(result.length).toBeGreaterThan(0);
  });

  it("formats future relative time in English", () => {
    const result = formatRelativeTime("en", 3, "month");
    expect(result).toContain("3");
  });

  it("handles 'now' as 0", () => {
    const result = formatRelativeTime("en", 0, "day");
    expect(result).toBeDefined();
  });

  it("formats relative time with numeric values", () => {
    // Test with a value that doesn't have a special word
    const result = formatRelativeTime("zh-CN", -5, "day");
    expect(result).toContain("5");
  });
});

describe("formatNumber", () => {
  it("formats integers with grouping", () => {
    const result = formatNumber("en", 1234567);
    expect(result).toBe("1,234,567");
  });

  it("formats decimals", () => {
    const result = formatNumber("en", 1234.56, { minimumFractionDigits: 2 });
    expect(result).toBe("1,234.56");
  });

  it("respects maximumFractionDigits", () => {
    const result = formatNumber("en", 1234.56789, { maximumFractionDigits: 2 });
    expect(result).toBe("1,234.57");
  });

  it("uses locale-appropriate grouping for zh-CN", () => {
    // zh-CN typically uses the same grouping style as en
    const result = formatNumber("zh-CN", 1234567);
    expect(result).toContain("1");
    expect(result).toContain("234");
    expect(result).toContain("567");
  });

  it("can disable grouping", () => {
    const result = formatNumber("en", 1234567, { useGrouping: false });
    expect(result).toBe("1234567");
  });

  it("returns empty string for non-finite values", () => {
    expect(formatNumber("en", NaN)).toBe("");
    expect(formatNumber("en", Infinity)).toBe("");
    expect(formatNumber("en", -Infinity)).toBe("");
  });
});

describe("formatPercent", () => {
  it("formats ratio as percentage", () => {
    const result = formatPercent("en", 0.25);
    expect(result).toBe("25%");
  });

  it("formats with fraction digits", () => {
    const result = formatPercent("en", 0.2567, { minimumFractionDigits: 2 });
    expect(result).toBe("25.67%");
  });

  it("respects maximumFractionDigits", () => {
    const result = formatPercent("en", 0.25678, { maximumFractionDigits: 1 });
    expect(result).toBe("25.7%");
  });

  it("formats small percentages correctly", () => {
    const result = formatPercent("en", 0.001);
    expect(result).toBe("0.1%");
  });

  it("handles 0 and 1 correctly", () => {
    expect(formatPercent("en", 0)).toBe("0%");
    expect(formatPercent("en", 1)).toBe("100%");
  });

  it("formats in Chinese locale", () => {
    const result = formatPercent("zh-CN", 0.25);
    expect(result).toContain("25");
  });

  it("returns empty string for non-finite values", () => {
    expect(formatPercent("en", NaN)).toBe("");
    expect(formatPercent("en", Infinity)).toBe("");
  });

  // Critical test: prevent 100x errors
  it("does NOT multiply input by 100 (input is already a ratio)", () => {
    // 0.25 as input should give 25%, not 2500%
    const result = formatPercent("en", 0.25);
    expect(result).toBe("25%");
    // 25 as input would be wrong - it would give 2500%
    const wrongInput = formatPercent("en", 25);
    expect(wrongInput).toBe("2,500%");
  });
});

describe("formatCurrency", () => {
  it("formats USD in English locale", () => {
    const result = formatCurrency("en", 1234.56, { currency: "USD" });
    expect(result).toContain("$");
    expect(result).toContain("1,234.56");
  });

  it("formats CNY in Chinese locale", () => {
    const result = formatCurrency("zh-CN", 1234.56, { currency: "CNY" });
    expect(result).toContain("1,234.56");
  });

  it("preserves currency code without conversion", () => {
    // Formatting USD amount with zh-CN locale should still be USD
    const result = formatCurrency("zh-CN", 100, { currency: "USD" });
    expect(result).toContain("100");
    // Currency code should appear
    expect(result).toMatch(/USD|US\s*\$/i);
  });

  it("respects fraction digit limits", () => {
    const result = formatCurrency("en", 1234.567, {
      currency: "USD",
      maximumFractionDigits: 2,
    });
    expect(result).toContain("1,234.57");
  });

  it("handles zero amounts", () => {
    const result = formatCurrency("en", 0, { currency: "USD" });
    expect(result).toContain("0");
  });

  it("handles negative amounts", () => {
    const result = formatCurrency("en", -50, { currency: "USD" });
    expect(result).toContain("50");
    expect(result).toContain("-");
  });

  it("returns empty string for non-finite values", () => {
    expect(formatCurrency("en", NaN, { currency: "USD" })).toBe("");
    expect(formatCurrency("en", Infinity, { currency: "USD" })).toBe("");
  });
});

describe("parseNumber", () => {
  it("parses English formatted numbers", () => {
    expect(parseNumber("en", "1,234.56")).toBe(1234.56);
    expect(parseNumber("en", "1000")).toBe(1000);
  });

  it("parses Chinese formatted numbers", () => {
    // Chinese uses same format as English for numbers
    const result = parseNumber("zh-CN", "1234.56");
    expect(result).toBe(1234.56);
  });

  it("returns NaN for unparseable strings", () => {
    expect(parseNumber("en", "not a number")).toBeNaN();
  });

  it("handles numbers without grouping", () => {
    expect(parseNumber("en", "1234567.89")).toBe(1234567.89);
  });
});
