import { useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import { LocaleContext, type LocaleContextValue } from "./locale-context";
import { LOCALE_SETTING_KEY, parseLocale, detectSystemLocale, translate, type Locale } from "./locale";

const DEFAULT_LOCALE: Locale = "en";

export function LocaleProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<Locale>(() => {
    // On first render, use system locale as default
    // This will be overridden if a saved preference exists
    return detectSystemLocale() ?? DEFAULT_LOCALE;
  });
  const hasUserSelectedLocale = useRef(false);

  useEffect(() => {
    let isMounted = true;
    void desktopApi.settings.getSetting(LOCALE_SETTING_KEY).then(
      (storedLocale) => {
        if (isMounted && !hasUserSelectedLocale.current) {
          // If user has a saved preference, use it
          // Otherwise, keep the system-detected locale
          if (storedLocale) {
            setLocaleState(parseLocale(storedLocale));
          }
        }
      },
      () => {
        // If settings read fails, keep system locale
        // This ensures the app works even if settings are unavailable
      },
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
