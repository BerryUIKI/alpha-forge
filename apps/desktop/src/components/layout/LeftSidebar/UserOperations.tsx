/**
 * UserOperations Component
 *
 * Bottom section of left sidebar for user operations.
 * Fixed position, always visible, does not scroll.
 * Integrates theme toggle and language selection functionality.
 *
 * @version GUI-M1-1
 */

import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { User, Sun, Moon, Settings, ChevronUp, Languages, Check } from "lucide-react";
import { useTheme } from "next-themes";
import { useLocale } from "@/lib/i18n/useLocale";
import { LOCALES, detectSystemLocale, type Locale } from "@/lib/i18n/locale";
import type { UserOperationsProps, UserMenuItem } from "../types";

type MenuView = "main" | "language";

const MENU_ITEMS: Array<{ id: UserMenuItem; label: string; icon: typeof User }> = [
  { id: "profile", label: "User Profile", icon: User },
  { id: "theme-toggle", label: "Toggle Theme", icon: Sun },
  { id: "language", label: "Language", icon: Languages },
  { id: "settings", label: "Settings", icon: Settings },
];

export function UserOperations({
  username = "User",
  isMenuOpen: externalIsMenuOpen,
  onMenuOpenChange,
  onMenuItemClick,
}: UserOperationsProps) {
  const [internalIsMenuOpen, setInternalIsMenuOpen] = useState(false);
  const [menuView, setMenuView] = useState<MenuView>("main");
  const isMenuOpen = externalIsMenuOpen ?? internalIsMenuOpen;
  const navigate = useNavigate();

  // Get theme from context
  const { theme, setTheme } = useTheme();
  const currentTheme = theme === "dark" ? "dark" : "light";

  // Get locale from context
  const { locale, setLocale, t } = useLocale();

  const handleToggle = () => {
    const newState = !isMenuOpen;
    setInternalIsMenuOpen(newState);
    setMenuView("main");
    onMenuOpenChange?.(newState);
  };

  const handleMenuItemClick = (item: UserMenuItem) => {
    if (item === "language") {
      // Switch to language sub-menu
      setMenuView("language");
      return;
    }

    onMenuItemClick?.(item);
    setInternalIsMenuOpen(false);
    onMenuOpenChange?.(false);

    // Handle theme toggle
    if (item === "theme-toggle") {
      setTheme(currentTheme === "light" ? "dark" : "light");
    } else if (item === "profile") {
      // TODO: [GUI-M1-1] Navigate to user profile page
      console.log("Profile clicked - implement navigation");
    } else if (item === "settings") {
      // Navigate to settings page
      navigate("/settings");
    }
  };

  const handleLanguageSelect = (selectedLocale: Locale | "system") => {
    if (selectedLocale === "system") {
      const systemLocale = detectSystemLocale();
      setLocale(systemLocale);
    } else {
      setLocale(selectedLocale);
    }
    setMenuView("main");
    setInternalIsMenuOpen(false);
    onMenuOpenChange?.(false);
  };

  const handleBackToMain = () => {
    setMenuView("main");
  };

  return (
    <div className="border-t border-border">
      {/* User Button */}
      <button
        onClick={handleToggle}
        className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-sm transition-colors hover:bg-accent"
        aria-expanded={isMenuOpen}
        aria-haspopup="menu"
        aria-label={`User menu: ${username}`}
      >
        <div className="flex items-center gap-2">
          <div className="flex h-7 w-7 items-center justify-center rounded-full bg-primary/10">
            <User className="h-4 w-4 text-primary" />
          </div>
          <span className="truncate font-medium">{username}</span>
        </div>
        <ChevronUp
          className={`h-4 w-4 transition-transform duration-200 ${
            isMenuOpen ? "" : "rotate-180"
          }`}
        />
      </button>

      {/* Dropdown Menu */}
      {isMenuOpen && (
        <div
          className="mb-1 mt-1 rounded-lg border border-border bg-popover shadow-lg"
          role="menu"
          aria-label="User operations menu"
        >
          {menuView === "main" ? (
            // Main menu
            <>
              {MENU_ITEMS.map((item) => {
                const Icon = item.id === "theme-toggle"
                  ? (currentTheme === "light" ? Sun : Moon)
                  : item.icon;
                return (
                  <button
                    key={item.id}
                    onClick={() => handleMenuItemClick(item.id)}
                    className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors first:rounded-t-lg last:rounded-b-lg hover:bg-accent"
                    role="menuitem"
                  >
                    <Icon className="h-4 w-4" />
                    <span>
                      {t(`menu${item.id.charAt(0).toUpperCase() + item.id.slice(1).replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())}` as any) || item.label}
                      {item.id === "theme-toggle" && ` (${currentTheme === "light" ? "Light" : "Dark"})`}
                      {item.id === "language" && ` (${locale === "zh-CN" ? "中文" : "EN"})`}
                    </span>
                  </button>
                );
              })}
            </>
          ) : (
            // Language sub-menu
            <>
              <button
                onClick={handleBackToMain}
                className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors rounded-t-lg hover:bg-accent"
                role="menuitem"
              >
                <ChevronUp className="h-4 w-4 rotate-[-90deg]" />
                <span className="font-medium">{t("language")}</span>
              </button>
              <div className="border-t border-border" />
              <button
                onClick={() => handleLanguageSelect("system")}
                className="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-accent"
                role="menuitem"
              >
                <span>{t("followSystem")}</span>
                {!LOCALES.includes(locale) && <Check className="h-4 w-4 text-primary" />}
              </button>
              <button
                onClick={() => handleLanguageSelect("en")}
                className="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-accent"
                role="menuitem"
              >
                <span>{t("english")}</span>
                {locale === "en" && <Check className="h-4 w-4 text-primary" />}
              </button>
              <button
                onClick={() => handleLanguageSelect("zh-CN")}
                className="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-sm transition-colors last:rounded-b-lg hover:bg-accent"
                role="menuitem"
              >
                <span>{t("simplifiedChinese")}</span>
                {locale === "zh-CN" && <Check className="h-4 w-4 text-primary" />}
              </button>
            </>
          )}
        </div>
      )}
    </div>
  );
}