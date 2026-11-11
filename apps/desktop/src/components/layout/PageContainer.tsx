import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface PageContainerProps {
  children: ReactNode;
  className?: string;
}

/**
 * Shared page gutter and layout rhythm for desktop pages.
 *
 * The container is also the query container for content-width-aware layouts,
 * so dashboard grids respond to the actual space left by the sidebars rather
 * than the window width.
 */
export function PageContainer({ children, className }: PageContainerProps) {
  return (
    <div data-page-container="" className={cn("@container flex flex-col gap-6 p-6", className)}>
      {children}
    </div>
  );
}
