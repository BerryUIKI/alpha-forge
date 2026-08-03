import { describe, expect, it } from "vitest";
import * as enCatalog from "./catalogs/en";
import * as zhCNCatalog from "./catalogs/zh-CN";

/**
 * Catalog parity test.
 * Ensures that both locales have matching keys for all namespaces.
 */

function getKeys(obj: Record<string, unknown>): Set<string> {
  return new Set(Object.keys(obj));
}

function assertCatalogParity(
  enModule: Record<string, unknown>,
  zhCNModule: Record<string, unknown>,
  namespace: string,
) {
  const enKeys = getKeys(enModule);
  const zhCNKeys = getKeys(zhCNModule);

  // Check for missing keys in zh-CN
  const missingInZhCN = [...enKeys].filter((key) => !zhCNKeys.has(key));
  if (missingInZhCN.length > 0) {
    console.error(
      `Missing keys in zh-CN ${namespace}:`,
      missingInZhCN,
    );
  }

  // Check for extra keys in zh-CN (shouldn't exist if source is en)
  const extraInZhCN = [...zhCNKeys].filter((key) => !enKeys.has(key));
  if (extraInZhCN.length > 0) {
    console.error(
      `Extra keys in zh-CN ${namespace}:`,
      extraInZhCN,
    );
  }

  return {
    enKeys,
    zhCNKeys,
    missingInZhCN,
    extraInZhCN,
    hasParity: missingInZhCN.length === 0 && extraInZhCN.length === 0,
  };
}

describe("Catalog parity", () => {
  it("common catalog has parity between en and zh-CN", () => {
    const result = assertCatalogParity(
      enCatalog.common as Record<string, unknown>,
      zhCNCatalog.common as Record<string, unknown>,
      "common",
    );
    expect(result.hasParity).toBe(true);
    expect(result.missingInZhCN).toHaveLength(0);
    expect(result.extraInZhCN).toHaveLength(0);
  });

  it("navigation catalog has parity between en and zh-CN", () => {
    const result = assertCatalogParity(
      enCatalog.navigation as Record<string, unknown>,
      zhCNCatalog.navigation as Record<string, unknown>,
      "navigation",
    );
    expect(result.hasParity).toBe(true);
    expect(result.missingInZhCN).toHaveLength(0);
    expect(result.extraInZhCN).toHaveLength(0);
  });

  it("settings catalog has parity between en and zh-CN", () => {
    const result = assertCatalogParity(
      enCatalog.settings as Record<string, unknown>,
      zhCNCatalog.settings as Record<string, unknown>,
      "settings",
    );
    expect(result.hasParity).toBe(true);
    expect(result.missingInZhCN).toHaveLength(0);
    expect(result.extraInZhCN).toHaveLength(0);
  });

  it("all catalogs have the same namespaces in both locales", () => {
    const enNamespaces = Object.keys(enCatalog);
    const zhCNNamespaces = Object.keys(zhCNCatalog);

    expect(enNamespaces.sort()).toEqual(zhCNNamespaces.sort());
  });
});