import { NavLink } from "react-router-dom";
import { LayoutDashboard, Search, BookOpen, PieChart, Box, Settings } from "lucide-react";

const navItems = [
  { to: "/today", icon: LayoutDashboard, label: "Today" },
  { to: "/research", icon: Search, label: "Research" },
  { to: "/journal", icon: BookOpen, label: "Journal" },
  { to: "/portfolio", icon: PieChart, label: "Portfolio" },
  { to: "/artifacts", icon: Box, label: "Artifacts" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export function Sidebar() {
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
          title={item.label}
        >
          <item.icon className="h-5 w-5" />
        </NavLink>
      ))}
    </aside>
  );
}
