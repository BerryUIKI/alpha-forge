/**
 * UserOperations Component
 *
 * Bottom section of left sidebar with theme toggle, language switch, and settings.
 * Fixed height, does not participate in scrolling.
 *
 * @version GUI-M2
 */

import { useNavigate } from "react-router-dom";
import { Moon, Sun, Globe, Settings } from "lucide-react";
import { LOCALES, type Locale } from "@/lib/i18n/locale";
import { useLocale } from "@/lib/i18n/useLocale";

interface UserOperationsProps {
  /** Current theme */
  theme?: "light" | "dark";
  /** Theme change callback */
  onThemeChange?: (theme: "light" | "dark") => void;
}

export function UserOperations({ theme, onThemeChange }: UserOperationsProps) {
  const { locale, setLocale, t } = useLocale();
  const navigate = useNavigate();

  const handleThemeToggle = () => {
    const newTheme = theme === "light" ? "dark" : "light";
    onThemeChange?.(newTheme);
  };

  const handleLanguageChange = (newLocale: Locale) => {
    void setLocale(newLocale);
  };

  const handleSettingsClick = () => {
    navigate("/settings");
  };

  return (
    <div className="border-t border-border p-3">
      <div className="flex items-center justify-between gap-2">
        {/* Theme Toggle */}
        <button
          onClick={handleThemeToggle}
          className="flex h-9 w-9 items-center justify-center rounded-lg transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-label={t("menuThemeToggle")}
          title={t("menuThemeToggle")}
        >
          {theme === "light" ? (
            <Moon className="h-4 w-4" />
          ) : (
            <Sun className="h-4 w-4" />
          )}
        </button>

        {/* Language Switcher */}
        <div className="relative">
          <select
            value={locale}
            onChange={(e) => handleLanguageChange(e.target.value as Locale)}
            className="flex h-9 items-center justify-center gap-1 rounded-lg border border-transparent bg-transparent px-2 pr-6 text-sm transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            aria-label={t("menuLanguage")}
            title={t("menuLanguage")}
            style={{
              appearance: "none",
              backgroundImage: "none",
            }}
          >
            {LOCALES.map((option) => (
              <option key={option} value={option}>
                {option === "zh-CN" ? "中文" : "EN"}
              </option>
            ))}
          </select>
          <Globe className="pointer-events-none absolute left-1.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
        </div>

        {/* Settings Button */}
        <button
          onClick={handleSettingsClick}
          className="flex h-9 w-9 items-center justify-center rounded-lg transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          aria-label={t("menuSettings")}
          title={t("menuSettings")}
        >
          <Settings className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}