import { NavLink } from "react-router-dom";
import {
  Box,
  BookOpen,
  LayoutDashboard,
  Monitor,
  Moon,
  PieChart,
  Search,
  Settings,
  Sun,
} from "lucide-react";

import { useTheme } from "@/app/theme/useTheme";
import type { Theme } from "@/app/theme/theme-context";

const navItems = [
  { to: "/today", icon: LayoutDashboard, label: "Today" },
  { to: "/research", icon: Search, label: "Research" },
  { to: "/journal", icon: BookOpen, label: "Journal" },
  { to: "/portfolio", icon: PieChart, label: "Portfolio" },
  { to: "/artifacts", icon: Box, label: "Artifacts" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

const nextTheme: Record<Theme, Theme> = {
  system: "light",
  light: "dark",
  dark: "system",
};

export function Sidebar() {
  const { theme, setTheme } = useTheme();
  const ThemeIcon = theme === "system" ? Monitor : theme === "dark" ? Moon : Sun;

  return (
    <aside className="flex w-16 flex-col items-center gap-1 border-r border-border bg-card py-4">
      {navItems.map((item) => (
        <NavLink
          key={item.to}
          to={item.to}
          className={({ isActive }) =>
            `flex h-10 w-10 items-center justify-center rounded-lg transition-colors ${
              isActive
                ? "bg-primary/10 text-primary"
                : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
            }`
          }
          title={item.label}
        >
          <item.icon className="h-5 w-5" />
        </NavLink>
      ))}
      <div className="mt-auto">
        <button
          type="button"
          onClick={() => setTheme(nextTheme[theme])}
          title={`Theme: ${theme}`}
          aria-label={`Switch theme (current: ${theme})`}
          className="flex h-10 w-10 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
        >
          <ThemeIcon className="h-5 w-5" />
        </button>
      </div>
    </aside>
  );
}
