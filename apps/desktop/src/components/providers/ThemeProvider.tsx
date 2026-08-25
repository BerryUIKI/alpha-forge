// Theme provider for managing light/dark mode.

import { ThemeProvider as NextThemesProvider } from "next-themes";
import { useEffect, type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";

function VisualPreferencesBootstrap() {
  useEffect(() => {
    void Promise.all([
      desktopApi.settings.getSetting("app.theme.accent"),
      desktopApi.settings.getSetting("app.theme.marketColors"),
    ]).then(([accent, marketColors]) => {
      if (accent) document.documentElement.dataset.accent = accent;
      if (marketColors) document.documentElement.dataset.marketColors = marketColors;
    }).catch(() => undefined);
  }, []);
  return null;
}

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
    >
      <VisualPreferencesBootstrap />
      {children}
    </NextThemesProvider>
  );
}
