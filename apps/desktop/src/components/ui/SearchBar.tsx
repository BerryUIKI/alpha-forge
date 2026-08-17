/**
 * SearchBar Component
 *
 * Styled search input with icon and keyboard shortcut hint.
 *
 * @version GUI-M2
 */

import { Search } from "lucide-react";
import { cn } from "@/lib/utils";

interface SearchBarProps {
  /** Placeholder text */
  placeholder?: string;
  /** Current value */
  value?: string;
  /** Change handler */
  onChange?: (value: string) => void;
  /** Key down handler */
  onKeyDown?: (e: React.KeyboardEvent) => void;
  /** Extra classes */
  className?: string;
}

export function SearchBar({
  placeholder = "Search...",
  value,
  onChange,
  onKeyDown,
  className,
}: SearchBarProps) {
  return (
    <div
      className={cn(
        "flex items-center gap-2 rounded-lg border border-border bg-muted/50 px-3 py-1.5 text-sm text-muted-foreground transition-colors",
        "focus-within:border-primary focus-within:ring-1 focus-within:ring-primary",
        className,
      )}
    >
      <Search className="h-4 w-4 shrink-0" />
      <input
        type="text"
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        onKeyDown={onKeyDown}
        placeholder={placeholder}
        className="flex-1 bg-transparent outline-none placeholder:text-muted-foreground/40"
      />
      <kbd className="hidden rounded border border-border bg-background px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground/60 md:inline-flex">
        ⌘K
      </kbd>
    </div>
  );
}