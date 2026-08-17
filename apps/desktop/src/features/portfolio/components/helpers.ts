/**
 * Utility helpers for Phase 3 portfolio dashboard.
 */

import { formatCurrency, formatNumber, formatPercent } from "@/lib/i18n/formatters";
import type { Locale } from "@/lib/i18n/locale";

/** Parse a decimal string (from Tauri) to a number for display. */
export function parseDecimal(value: string | null | undefined): number {
  if (value == null) return 0;
  const n = parseFloat(value);
  return isNaN(n) ? 0 : n;
}

/** Format a monetary value from a decimal string. */
export function fmtMoney(
  value: string | null | undefined,
  currency = "USD",
  locale: Locale = "en",
): string {
  return formatCurrency(locale, parseDecimal(value), { currency });
}

/** Format a number from a decimal string. */
export function fmtNumber(
  value: string | null | undefined,
  digits = 2,
  locale: Locale = "en",
): string {
  return formatNumber(locale, parseDecimal(value), {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

/** Format a percentage from a decimal string (e.g. "5.23" → "5.23%"). */
export function fmtPercent(
  value: string | null | undefined,
  digits = 2,
  locale: Locale = "en",
): string {
  if (value == null) return "—";
  // Rust computes weight_pct as (asset / total * 100), so store as a ratio
  // for formatPercent (which multiplies by 100 for display).
  const ratio = parseDecimal(value) / 100;
  return formatPercent(locale, ratio, {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

/** Format a gain/loss value with sign. */
export function fmtGainLoss(
  value: string | null | undefined,
  currency = "USD",
  locale: Locale = "en",
): string {
  const n = parseDecimal(value);
  const sign = n >= 0 ? "+" : "";
  return `${sign}${formatCurrency(locale, n, { currency })}`;
}

/** Determine CSS class for gain/loss (positive = green, negative = red). */
export function gainLossClass(value: string | null | undefined): string {
  const n = parseDecimal(value);
  if (n > 0) return "text-green-600 dark:text-green-400";
  if (n < 0) return "text-red-600 dark:text-red-400";
  return "text-muted-foreground";
}