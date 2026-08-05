/**
 * UserOperations Component
 *
 * Bottom section of left sidebar for user operations.
 * Fixed position, always visible, does not scroll.
 * Integrates theme toggle and language selection.
 *
 * @version GUI-M1-1
 */

import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useTheme } from "next-themes";
import { User, Sun, Moon, Settings, ChevronUp, Globe } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type { UserOperationsProps, UserMenuItem } from "../types";

const SUPPORTED_LOCALES = [
  { code: "en" as const, label: "English" },
  { code: "zh-CN" as const, label: "简体中文" },
];

export function UserOperations({
  username = "User",
  isMenuOpen: externalIsMenuOpen,
  onMenuOpenChange,
  onMenuItemClick,
}: UserOperationsProps) {
  const navigate = useNavigate();
  const { t, locale, setLocale } = useLocale();
  const { theme, setTheme } = useTheme();
  const [internalIsMenuOpen, setInternalIsMenuOpen] = useState(false);
  const [isLanguageMenuOpen, setIsLanguageMenuOpen] = useState(false);

  const isMenuOpen = externalIsMenuOpen ?? internalIsMenuOpen;
  const currentTheme = theme === "dark" ? "dark" : "light";

  const handleToggle = () => {
    const newState = !isMenuOpen;
    setInternalIsMenuOpen(newState);
    onMenuOpenChange?.(newState);
    // Close language menu when main menu closes
    if (!newState) {
      setIsLanguageMenuOpen(false);
    }
  };

  const handleMenuItemClick = (item: UserMenuItem) => {
    onMenuItemClick?.(item);
    setInternalIsMenuOpen(false);
    onMenuOpenChange?.(false);
    setIsLanguageMenuOpen(false);

    switch (item) {
      case "theme-toggle":
        setTheme(currentTheme === "light" ? "dark" : "light");
        break;
      case "settings":
        navigate("/settings");
        break;
      case "language":
        // Toggle language submenu
        setIsLanguageMenuOpen(!isLanguageMenuOpen);
        setInternalIsMenuOpen(true);
        onMenuOpenChange?.(true);
        break;
      case "profile":
        // TODO: Navigate to profile page when implemented
        break;
    }
  };

  const handleLanguageSelect = (newLocale: "en" | "zh-CN") => {
    setLocale(newLocale);
    setIsLanguageMenuOpen(false);
    setInternalIsMenuOpen(false);
    onMenuOpenChange?.(false);
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
          {/* Language Submenu */}
          {isLanguageMenuOpen ? (
            <>
              <button
                onClick={() => setIsLanguageMenuOpen(false)}
                className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors first:rounded-t-lg hover:bg-accent text-muted-foreground"
                role="menuitem"
              >
                <ChevronUp className="h-4 w-4 rotate-[-90deg]" />
                <span>{t("back" as any) || "Back"}</span>
              </button>
              <div className="border-t border-border" />
              {SUPPORTED_LOCALES.map((loc) => (
                <button
                  key={loc.code}
                  onClick={() => handleLanguageSelect(loc.code)}
                  className={`flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-accent ${
                    locale === loc.code ? "bg-primary/10 font-medium" : ""
                  }`}
                  role="menuitem"
                >
                  <Globe className="h-4 w-4" />
                  <span>{loc.label}</span>
                </button>
              ))}
            </>
          ) : (
            <>
              {/* Theme Toggle */}
              <button
                onClick={() => handleMenuItemClick("theme-toggle")}
                className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors first:rounded-t-lg hover:bg-accent"
                role="menuitem"
              >
                {currentTheme === "light" ? (
                  <Sun className="h-4 w-4" />
                ) : (
                  <Moon className="h-4 w-4" />
                )}
                <span>
                  {currentTheme === "light" 
                    ? (t("lightMode" as any) || "Light Mode")
                    : (t("darkMode" as any) || "Dark Mode")
                  }
                </span>
              </button>

              {/* Language Selection */}
              <button
                onClick={() => handleMenuItemClick("language")}
                className="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-sm transition-colors hover:bg-accent"
                role="menuitem"
              >
                <div className="flex items-center gap-2">
                  <Globe className="h-4 w-4" />
                  <span>{t("language" as any) || "Language"}</span>
                </div>
                <ChevronUp className="h-4 w-4 rotate-90" />
              </button>

              {/* Settings */}
              <button
                onClick={() => handleMenuItemClick("settings")}
                className="flex w-full items-center gap-2 px-3 py-2 text-left text-sm transition-colors last:rounded-b-lg hover:bg-accent"
                role="menuitem"
              >
                <Settings className="h-4 w-4" />
                <span>{t("settings" as any) || "Settings"}</span>
              </button>
            </>
          )}
        </div>
      )}
    </div>
  );
}