/**
 * i18n module barrel file.
 * Exports all public APIs for the internationalization system.
 */

// Locale types and core functions
export {
  LOCALES,
  DEFAULT_LOCALE,
  LOCALE_SETTING_KEY,
  parseLocale,
  translate,
  formatMessage,
  type Locale,
  type MessageKey,
} from "./locale";

// Provider and hooks
export { LocaleProvider } from "./LocaleProvider";
export { useLocale } from "./useLocale";
export { LocaleContext, type LocaleContextValue } from "./locale-context";

// Formatters
export {
  formatDate,
  formatRelativeTime,
  formatNumber,
  formatPercent,
  formatCurrency,
  parseNumber,
  type DateFormatOptions,
  type NumberFormatOptions,
  type PercentFormatOptions,
  type CurrencyFormatOptions,
} from "./formatters";

// Catalogs - English (source)
export { common as enCommon, navigation as enNavigation, settings as enSettings, workspace as enWorkspace, research as enResearch } from "./catalogs/en";

// Catalogs - Simplified Chinese
export { common as zhCNCommon, navigation as zhCNNavigation, settings as zhCNSettings, workspace as zhCNWorkspace, research as zhCNResearch } from "./catalogs/zh-CN";