/**
 * HoldingsList Component
 *
 * Displays top holdings with ticker, name, sector, value, and change.
 *
 * @version GUI-M3
 */

import { cn } from "@/lib/utils";

export interface Holding {
  id: string;
  ticker: string;
  name: string;
  sector: string;
  allocation: string;
  value: string;
  change: string;
  changePositive: boolean;
}

interface HoldingsListProps {
  holdings: Holding[];
  className?: string;
}

export function HoldingsList({ holdings, className }: HoldingsListProps) {
  return (
    <ul className={cn("divide-y divide-border/60", className)}>
      {holdings.map((holding) => (
        <li key={holding.id} className="flex items-center justify-between gap-4 py-2.5">
          <div className="flex min-w-0 items-center gap-3">
            <span className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-muted text-[10px] font-bold text-muted-foreground">
              {holding.ticker}
            </span>
            <div className="min-w-0">
              <div className="truncate text-sm font-medium">{holding.name}</div>
              <div className="truncate text-xs text-muted-foreground">
                {holding.sector} · {holding.allocation}
              </div>
            </div>
          </div>
          <div className="text-right">
            <div className="text-sm font-semibold">{holding.value}</div>
            <div
              className={cn(
                "text-xs font-medium",
                holding.changePositive ? "text-green-500" : "text-red-500",
              )}
            >
              {holding.change}
            </div>
          </div>
        </li>
      ))}
    </ul>
  );
}