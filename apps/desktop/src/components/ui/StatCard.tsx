/**
 * StatCard Component
 *
 * Metric display card with label, large value, and optional change indicator.
 *
 * @version GUI-M2
 */

import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface StatCardProps {
  /** Metric label */
  label: string;
  /** Metric value */
  value: string;
  /** Optional change text (e.g. "+$12,430 (1.49%)") */
  change?: string;
  /** Whether the change is positive */
  isPositive?: boolean;
  /** Optional icon */
  icon?: ReactNode;
  /** Extra classes */
  className?: string;
}

export function StatCard({
  label,
  value,
  change,
  isPositive = true,
  icon,
  className,
}: StatCardProps) {
  return (
    <div className={cn("rounded-xl border border-border bg-card p-5", className)}>
      <div className="flex items-center justify-between">
        <div className="text-xs font-medium uppercase tracking-wider text-muted-foreground">
          {label}
        </div>
        {icon && <div className="text-muted-foreground">{icon}</div>}
      </div>
      <div className="mt-1.5 text-2xl font-bold tracking-tight">{value}</div>
      {change && (
        <div
          className={cn(
            "mt-1 text-sm font-medium",
            isPositive ? "text-green-500" : "text-red-500",
          )}
        >
          {change}
        </div>
      )}
    </div>
  );
}