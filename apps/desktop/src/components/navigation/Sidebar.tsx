import { NavLink } from "react-router-dom";
import { LayoutDashboard, Search, BookOpen, PieChart, Box, Settings } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";

const navItems = [
  { to: "/today", icon: LayoutDashboard, label: "today" },
  { to: "/research", icon: Search, label: "research" },
  { to: "/journal", icon: BookOpen, label: "journal" },
  { to: "/portfolio", icon: PieChart, label: "portfolio" },
  { to: "/artifacts", icon: Box, label: "artifacts" },
  { to: "/settings", icon: Settings, label: "settings" },
] as const;

export function Sidebar() {
  const { t } = useLocale();

  return (
    <aside className="flex w-16 flex-col items-center border-r border-border bg-card py-4">
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
          title={t(item.label)}
          aria-label={t(item.label)}
        >
          <item.icon className="h-5 w-5" />
        </NavLink>
      ))}
    </aside>
  );
}
