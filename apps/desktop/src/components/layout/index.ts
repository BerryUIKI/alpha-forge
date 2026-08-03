/**
 * Layout Components Module
 *
 * Simplified layout for the unified routing-based architecture.
 * - MainLayout: Orchestrator with Sidebar + Outlet + RightSidebar
 * - RightSidebar: Collapsible Agent panel
 */

// Main orchestrator component
export { MainLayout } from "./MainLayout";

// Right Sidebar components
export { RightSidebar, AgentPanel } from "./RightSidebar";

// Type definitions
export type {
  SidebarState,
  AppStatus,
  RightSidebarProps,
  AgentPanelProps,
} from "./types";

// Constants
export { DEFAULT_SIDEBAR_WIDTHS } from "./types";