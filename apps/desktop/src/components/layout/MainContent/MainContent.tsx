/**
 * MainContent Component
 *
 * Main content area container combining routed page content and StatusBar.
 * Renders routed pages via children (Outlet).
 *
 * @version GUI-M0
 */

import type { MainContentProps } from "../types";
import { StatusBar } from "./StatusBar";

export function MainContent({ children }: MainContentProps) {
  return (
    <div className="flex min-w-0 flex-1 flex-col overflow-hidden">
      {/* Middle: Page Content (Outlet) */}
      <div className="min-w-0 flex-1 overflow-auto">{children}</div>

      {/* Bottom: Status Bar */}
      <StatusBar />
    </div>
  );
}
