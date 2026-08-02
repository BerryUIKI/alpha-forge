import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import { LocaleContext, type LocaleContextValue } from "./locale-context";
import { DEFAULT_LOCALE, LOCALE_SETTING_KEY, parseLocale, translate, type Locale } from "./locale";

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>(DEFAULT_LOCALE);
  const hasUserSelectedLocale = useRef(false);

  useEffect(() => {
    let isMounted = true;
    void desktopApi.settings.getSetting(LOCALE_SETTING_KEY).then(
      (storedLocale) => {
        if (isMounted && !hasUserSelectedLocale.current) {
          setLocaleState(parseLocale(storedLocale));
        }
      },
      () => undefined,
    );

    return () => {
      isMounted = false;
    };
  }, []);

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  const value = useMemo<LocaleContextValue>(
    () => ({
      locale,
      setLocale: async (nextLocale) => {
        hasUserSelectedLocale.current = true;
        setLocaleState(nextLocale);
        try {
          await desktopApi.settings.setSetting(LOCALE_SETTING_KEY, nextLocale);
        } catch {
          // Keep the active session language even if persistence is temporarily unavailable.
        }
      },
      t: (key) => translate(locale, key),
    }),
    [locale],
  );

  return <LocaleContext.Provider value={value}>{children}</LocaleContext.Provider>;
}
