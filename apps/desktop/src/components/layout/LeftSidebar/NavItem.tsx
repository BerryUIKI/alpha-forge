/**
 * NavItem Component
 *
 * A single navigation item in the sidebar.
 * Highlights active state based on current route.
 *
 * @version GUI-M0
 */

import { Link, useLocation } from "react-router-dom";
import { cn } from "@/lib/utils";
import type { NavItem as NavItemType } from "../types";

interface NavItemProps {
  /** Navigation item configuration */
  item: NavItemType;
  /** Whether the sidebar is collapsed */
  collapsed: boolean;
}

export function NavItem({ item, collapsed }: NavItemProps) {
  const location = useLocation();
  const isActive = location.pathname === item.route ||
    (item.route !== "/" && location.pathname.startsWith(item.route));

  return (
    <Link
      to={item.route}
      className={cn(
        "flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
        "hover:bg-accent hover:text-accent-foreground",
        isActive
          ? "bg-accent text-accent-foreground"
          : "text-muted-foreground",
        collapsed && "justify-center px-0",
      )}
      aria-current={isActive ? "page" : undefined}
      title={collapsed ? item.label : undefined}
    >
      <span className="flex h-5 w-5 shrink-0 items-center justify-center">
        <item.icon className="h-5 w-5" />
      </span>
      {!collapsed && <span>{item.label}</span>}
    </Link>
  );
}