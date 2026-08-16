/**
 * ActivityFeed Component
 *
 * List of recent activity items with colored type indicators.
 *
 * @version GUI-M3
 */

import { cn } from "@/lib/utils";

export interface ActivityItem {
  id: string;
  type: "research" | "thesis" | "portfolio" | "options";
  title: string;
  description: string;
  timestamp: string;
}

interface ActivityFeedProps {
  items: ActivityItem[];
  className?: string;
}

const TYPE_DOT_CLASSES: Record<ActivityItem["type"], string> = {
  research: "bg-indigo-400",
  thesis: "bg-green-500",
  portfolio: "bg-amber-400",
  options: "bg-sky-400",
};

export function ActivityFeed({ items, className }: ActivityFeedProps) {
  return (
    <ul className={cn("divide-y divide-border/60", className)}>
      {items.map((item) => (
        <li key={item.id} className="flex items-start gap-3 py-2.5">
          <span
            className={cn("mt-1.5 h-2 w-2 shrink-0 rounded-full", TYPE_DOT_CLASSES[item.type])}
          />
          <div className="min-w-0 flex-1">
            <div className="flex items-baseline gap-2">
              <span className="text-sm font-semibold">{item.title}</span>
              <span className="text-xs text-muted-foreground/60">{item.timestamp}</span>
            </div>
            <p className="line-clamp-2 text-sm text-muted-foreground">{item.description}</p>
          </div>
        </li>
      ))}
    </ul>
  );
}