/**
 * UserOperations Component
 *
 * Bottom section of left sidebar for user operations.
 * Fixed position, always visible, does not scroll.
 * UI-only implementation with placeholder menu items.
 *
 * TODO: [GUI-M1-1] Implement user profile navigation
 * TODO: [GUI-M1-1] Implement theme toggle business logic
 * TODO: [GUI-M1-1] Implement settings navigation
 * TODO: [GUI-M1-4] Add i18n for menu labels
 */

import { useState } from "react";
import { User, Sun, Moon, Settings, ChevronUp } from "lucide-react";
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
  theme = "light",
}: UserOperationsProps) {
  const [internalIsMenuOpen, setInternalIsMenuOpen] = useState(false);
  const isMenuOpen = externalIsMenuOpen ?? internalIsMenuOpen;

  const handleToggle = () => {
    const newState = !isMenuOpen;
    setInternalIsMenuOpen(newState);
    onMenuOpenChange?.(newState);
  };

  const handleMenuItemClick = (item: UserMenuItem) => {
    onMenuItemClick?.(item);
    setInternalIsMenuOpen(false);
    onMenuOpenChange?.(false);

    // TODO: [GUI-M1-1] Implement actual menu actions
    // Placeholder for business logic
    if (item === "theme-toggle") {
      // TODO: [GUI-M1-1] Integrate with theme provider
      console.log("Theme toggle clicked - implement business logic");
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
            const Icon = item.id === "theme-toggle" ? (theme === "light" ? Sun : Moon) : item.icon;
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
                  {item.id === "theme-toggle" && ` (${theme === "light" ? "Light" : "Dark"})`}
                </span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}