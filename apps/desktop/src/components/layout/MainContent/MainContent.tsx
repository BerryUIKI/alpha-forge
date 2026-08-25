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
    <div className="flex flex-1 flex-col overflow-hidden">
      {/* Middle: Page Content (Outlet) */}
      <div className="flex-1 overflow-auto">
        {children}
      </div>

      {/* Bottom: Status Bar */}
      <StatusBar />
    </div>
  );
}
