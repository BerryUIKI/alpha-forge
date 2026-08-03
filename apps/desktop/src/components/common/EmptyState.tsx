// Empty state component.

import { FileX } from "lucide-react";

interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: React.ReactNode;
  /** Accessible label for the empty state container */
  ariaLabel?: string;
}

export function EmptyState({ icon, title, description, action, ariaLabel }: EmptyStateProps) {
  return (
    <div
      className="flex flex-col items-center justify-center p-8 text-center"
      role="status"
      aria-label={ariaLabel || title}
    >
      <div className="mb-4 rounded-full bg-muted p-4">
        {icon || <FileX className="h-8 w-8 text-muted-foreground" aria-hidden="true" />}
      </div>
      <h3 className="mb-2 text-lg font-semibold">{title}</h3>
      {description && (
        <p className="mb-4 max-w-md text-sm text-muted-foreground">{description}</p>
      )}
      {action && <div className="mt-2">{action}</div>}
    </div>
  );
}