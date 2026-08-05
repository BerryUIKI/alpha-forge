/**
 * Layout Components Module
 *
 * Hybrid layout combining Codex-style workspace selection with route-based navigation.
 * - LeftSidebar: Workspace selector, project list, user operations
 * - MainContent: OperationBar, routed pages, StatusBar
 * - RightSidebar: Collapsible Agent panel
 */

// Main orchestrator component
export { MainLayout } from "./MainLayout";

// Left Sidebar components
export { LeftSidebar, WorkspaceSelector, ScrollableList, UserOperations } from "./LeftSidebar";

// Main Content components
export { MainContent, OperationBar, StatusBar } from "./MainContent";

// Right Sidebar components
export { RightSidebar, AgentPanel } from "./RightSidebar";

// Type definitions
export type {
  SidebarState,
  AppStatus,
  WorkspaceType,
  UserMenuItem,
  LeftSidebarProps,
  WorkspaceSelectorProps,
  ScrollableListProps,
  ScrollableListItem,
  UserOperationsProps,
  RightSidebarProps,
  AgentPanelProps,
  MainContentProps,
  OperationBarProps,
  StatusBarProps,
} from "./types";

// Constants
export { WORKSPACE_OPTIONS, DEFAULT_SIDEBAR_WIDTHS } from "./types";