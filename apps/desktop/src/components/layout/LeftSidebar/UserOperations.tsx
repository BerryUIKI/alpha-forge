/**
 * UserOperations Component
 *
 * Bottom section of left sidebar for user operations.
 * Fixed position, always visible, does not scroll.
 * Integrates theme toggle functionality.
 *
 * @version GUI-M1-1
 */

import { useState } from "react";
import { User, Sun, Moon, Settings, ChevronUp } from "lucide-react";
import { useTheme } from "next-themes";
import type { UserOperationsProps, UserMenuItem } from "../types";

const MENU_ITEMS: Array<{ id: UserMenuItem; label: string; icon: typeof User }> = [
  { id: "profile", label: "User Profile", icon: User },
  { id: "theme-toggle", label: "Toggle Theme", icon: Sun },
  { id: "settings", label: "Settings", icon: Settings },
];

export function UserOperations({
  username = "User",
  isMenuOpen: externalIsMenuOpen,
  onMenuOpenChange,
  onMenuItemClick,
}: UserOperationsProps) {
  const [internalIsMenuOpen, setInternalIsMenuOpen] = useState(false);
  const isMenuOpen = externalIsMenuOpen ?? internalIsMenuOpen;
  
  // Get theme from context
  const { theme, setTheme } = useTheme();
  const currentTheme = theme === "dark" ? "dark" : "light";

  const handleToggle = () => {
    const newState = !isMenuOpen;
    setInternalIsMenuOpen(newState);
    onMenuOpenChange?.(newState);
  };

  const handleMenuItemClick = (item: UserMenuItem) => {
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
      // TODO: [GUI-M1-1] Navigate to settings page
      console.log("Settings clicked - implement navigation");
    }
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
                  {item.label}
                  {item.id === "theme-toggle" && ` (${currentTheme === "light" ? "Light" : "Dark"})`}
                </span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}