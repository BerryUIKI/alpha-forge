import { createContext } from "react";
import { DEFAULT_LOCALE, translate, type Locale, type MessageKey } from "./locale";

export interface LocaleContextValue {
  locale: Locale;
  setLocale: (locale: Locale) => Promise<void>;
  t: (key: MessageKey) => string;
}

export const LocaleContext = createContext<LocaleContextValue>({
  locale: DEFAULT_LOCALE,
  setLocale: async () => undefined,
  t: (key) => translate(DEFAULT_LOCALE, key),
});
