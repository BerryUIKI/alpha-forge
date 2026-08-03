/**
 * Shared Intl formatters for locale-aware presentation.
 *
 * These wrappers ensure consistent formatting across the application
 * and prevent locale-specific errors from affecting persisted values.
 */

import type { Locale } from "./locale";

/**
 * Format options for date/time presentation.
 */
export interface DateFormatOptions {
  /** Include time in the output */
  includeTime?: boolean;
  /** Date style override */
  dateStyle?: "full" | "long" | "medium" | "short";
  /** Time style override (only used when includeTime is true) */
  timeStyle?: "full" | "long" | "medium" | "short";
}

/**
 * Format options for number presentation.
 */
export interface NumberFormatOptions {
  /** Minimum fraction digits */
  minimumFractionDigits?: number;
  /** Maximum fraction digits */
  maximumFractionDigits?: number;
  /** Use grouping separators (e.g., thousands) */
  useGrouping?: boolean;
}

/**
 * Format options for percent presentation.
 */
export interface PercentFormatOptions {
  /** Minimum fraction digits */
  minimumFractionDigits?: number;
  /** Maximum fraction digits */
  maximumFractionDigits?: number;
}

/**
 * Format options for currency presentation.
 */
export interface CurrencyFormatOptions {
  /** Currency code (ISO 4217) */
  currency: string;
  /** Minimum fraction digits (defaults to currency standard) */
  minimumFractionDigits?: number;
  /** Maximum fraction digits (defaults to currency standard) */
  maximumFractionDigits?: number;
}

/**
 * Format a date/time value for the given locale.
 *
 * @param locale - The target locale
 * @param value - The date to format (Date, ISO string, or timestamp)
 * @param options - Formatting options
 * @returns Formatted date string
 */
export function formatDate(
  locale: Locale,
  value: Date | string | number,
  options: DateFormatOptions = {},
): string {
  const date = typeof value === "string" ? new Date(value) : new Date(value);

  if (isNaN(date.getTime())) {
    return "";
  }

  const formatterOptions: Intl.DateTimeFormatOptions = {};

  if (options.dateStyle) {
    formatterOptions.dateStyle = options.dateStyle;
  } else {
    formatterOptions.year = "numeric";
    formatterOptions.month = "short";
    formatterOptions.day = "numeric";
  }

  if (options.includeTime) {
    if (options.timeStyle) {
      formatterOptions.timeStyle = options.timeStyle;
    } else {
      formatterOptions.hour = "2-digit";
      formatterOptions.minute = "2-digit";
    }
  }

  const formatter = new Intl.DateTimeFormat(locale, formatterOptions);
  return formatter.format(date);
}

/**
 * Format a relative time value (e.g., "2 days ago").
 *
 * @param locale - The target locale
 * @param value - The value and unit
 * @param unit - The time unit
 * @returns Formatted relative time string
 */
export function formatRelativeTime(
  locale: Locale,
  value: number,
  unit: Intl.RelativeTimeFormatUnit,
): string {
  const formatter = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
  return formatter.format(value, unit);
}

/**
 * Format a number for the given locale.
 *
 * @param locale - The target locale
 * @param value - The number to format
 * @param options - Formatting options
 * @returns Formatted number string
 */
export function formatNumber(
  locale: Locale,
  value: number,
  options: NumberFormatOptions = {},
): string {
  if (!Number.isFinite(value)) {
    return "";
  }

  const formatterOptions: Intl.NumberFormatOptions = {
    useGrouping: options.useGrouping ?? true,
  };

  if (options.minimumFractionDigits !== undefined) {
    formatterOptions.minimumFractionDigits = options.minimumFractionDigits;
  }
  if (options.maximumFractionDigits !== undefined) {
    formatterOptions.maximumFractionDigits = options.maximumFractionDigits;
  }

  const formatter = new Intl.NumberFormat(locale, formatterOptions);
  return formatter.format(value);
}

/**
 * Format a percentage for the given locale.
 *
 * IMPORTANT: The input value is expected to be a ratio (e.g., 0.25 for 25%).
 * DO NOT pass pre-multiplied values (e.g., 25 for 25%).
 *
 * @param locale - The target locale
 * @param value - The ratio value (0-1 scale)
 * @param options - Formatting options
 * @returns Formatted percent string
 */
export function formatPercent(
  locale: Locale,
  value: number,
  options: PercentFormatOptions = {},
): string {
  if (!Number.isFinite(value)) {
    return "";
  }

  const formatterOptions: Intl.NumberFormatOptions = {
    style: "percent",
    minimumFractionDigits: options.minimumFractionDigits ?? 0,
    maximumFractionDigits: options.maximumFractionDigits ?? 2,
  };

  const formatter = new Intl.NumberFormat(locale, formatterOptions);
  return formatter.format(value);
}

/**
 * Format a currency amount for the given locale.
 *
 * IMPORTANT: This function formats only; it does NOT convert currencies.
 * The caller must ensure the amount matches the provided currency code.
 *
 * @param locale - The target locale
 * @param value - The amount to format
 * @param options - Formatting options including currency code
 * @returns Formatted currency string
 */
export function formatCurrency(
  locale: Locale,
  value: number,
  options: CurrencyFormatOptions,
): string {
  if (!Number.isFinite(value)) {
    return "";
  }

  const formatterOptions: Intl.NumberFormatOptions = {
    style: "currency",
    currency: options.currency,
  };

  if (options.minimumFractionDigits !== undefined) {
    formatterOptions.minimumFractionDigits = options.minimumFractionDigits;
  }
  if (options.maximumFractionDigits !== undefined) {
    formatterOptions.maximumFractionDigits = options.maximumFractionDigits;
  }

  const formatter = new Intl.NumberFormat(locale, formatterOptions);
  return formatter.format(value);
}

/**
 * Parse a locale-aware number string back to a number.
 * Returns NaN if the string cannot be parsed.
 *
 * @param locale - The source locale
 * @param value - The formatted number string
 * @returns Parsed number or NaN
 */
export function parseNumber(locale: Locale, value: string): number {
  const formatter = new Intl.NumberFormat(locale);
  const parts = formatter.formatToParts(1234.5);

  let groupSeparator = ",";
  let decimalSeparator = ".";

  for (const part of parts) {
    if (part.type === "group") {
      groupSeparator = part.value;
    } else if (part.type === "decimal") {
      decimalSeparator = part.value;
    }
  }

  // Remove group separators and normalize decimal separator
  const normalized = value
    .replace(new RegExp(`\\${groupSeparator}`, "g"), "")
    .replace(decimalSeparator, ".");

  return parseFloat(normalized);
}