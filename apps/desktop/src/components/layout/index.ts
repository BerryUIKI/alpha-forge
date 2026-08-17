/**
 * Layout Components Module
 *
 * Three-zone layout:
 * - LeftSidebar: navigation with NavItem/NavGroup
 * - MainContent: TopBar, routed pages, StatusBar
 * - RightSidebar: Collapsible Agent panel
 */

// Main orchestrator component
export { MainLayout } from "./MainLayout";

// Left Sidebar components
export { LeftSidebar, NavItem, NavGroup } from "./LeftSidebar";

// Main Content components
export { MainContent, StatusBar } from "./MainContent";

// Top Bar components
export { TopBar } from "./TopBar";

// Right Sidebar components
export { RightSidebar, AgentPanel } from "./RightSidebar";

// Type definitions
export type {
  SidebarState,
  AppStatus,
  NavItem as NavItemType,
  NavGroup as NavGroupType,
  LeftSidebarProps,
  TopBarProps,
  MainContentProps,
  StatusBarProps,
  RightSidebarProps,
  AgentPanelProps,
} from "./types";

// Constants
export { DEFAULT_SIDEBAR_WIDTHS } from "./types";