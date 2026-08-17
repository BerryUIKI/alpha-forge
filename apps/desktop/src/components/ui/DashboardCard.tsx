/**
 * DashboardCard Component
 *
 * Reusable glass-morphism card with header and body.
 * Matches the Wealthfolio-inspired design language.
 *
 * @version GUI-M2
 */

import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface DashboardCardProps {
  /** Card title */
  title: string;
  /** Inline subtitle next to the title */
  subtitle?: ReactNode;
  /** Right-aligned slot — a toggle, link, etc. Takes precedence over `meta`. */
  action?: ReactNode;
  /** Convenience: right-aligned uppercase meta text */
  meta?: string;
  /** Whether the body has padding. Set false for full-bleed content. */
  padded?: boolean;
  /** Higher opacity + subtle shadow, for cards on low-contrast backgrounds. */
  elevated?: boolean;
  /** Extra classes for the card body. */
  className?: string;
  children: ReactNode;
}

export function DashboardCard({
  title,
  subtitle,
  action,
  meta,
  padded = true,
  elevated = false,
  className,
  children,
}: DashboardCardProps) {
  return (
    <div className="w-full">
      <div className="flex items-baseline justify-between pb-2">
        <div className="flex items-baseline gap-2">
          <h2 className="text-sm font-semibold tracking-tight">{title}</h2>
          {subtitle != null && subtitle !== "" && (
            <span className="text-muted-foreground/60 text-xs">{subtitle}</span>
          )}
        </div>
        {action ??
          (meta ? (
            <span className="text-muted-foreground/70 text-xs font-medium uppercase tracking-wide">
              {meta}
            </span>
          ) : null)}
      </div>
      <div
        className={cn(
          "border-border/40 rounded-xl border backdrop-blur-xl",
          elevated ? "bg-card/90 shadow-xs" : "bg-card/70",
          padded ? "p-3 md:p-4" : "",
          className,
        )}
      >
        {children}
      </div>
    </div>
  );
}