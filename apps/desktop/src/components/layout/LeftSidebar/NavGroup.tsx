/**
 * NavGroup Component
 *
 * A named group of navigation items in the sidebar.
 * Shows a section label when expanded, hides it when collapsed.
 *
 * @version GUI-M0
 */

import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface NavGroupProps {
  /** Section label text */
  label: string;
  /** Navigation items */
  children: ReactNode;
  /** Whether the sidebar is collapsed */
  collapsed: boolean;
}

export function NavGroup({ label, children, collapsed }: NavGroupProps) {
  return (
    <div className="mb-2" role="group" aria-label={label}>
      {!collapsed && (
        <div className="px-3 pb-1.5">
          <span className="text-xs font-semibold uppercase tracking-wider text-muted-foreground/60">
            {label}
          </span>
        </div>
      )}
      <div className={cn("flex flex-col gap-0.5", collapsed && "items-center")}>
        {children}
      </div>
    </div>
  );
}