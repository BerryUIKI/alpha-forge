/**
 * Central formatting utilities module.
 *
 * Re-exports locale-aware formatting functions for dates, numbers, currencies,
 * and percentages.
 *
 * @module lib/formatting
 */

export {
  formatDate,
  formatNumber,
  formatCurrency,
  formatPercent,
  parseNumber,
  type DateFormatOptions,
  type NumberFormatOptions,
  type CurrencyFormatOptions,
  type PercentFormatOptions,
} from "../i18n/formatters";
