/**
 * Layout Components Module
 *
 * This module provides the Codex-style layout components for the desktop application.
 * All components are UI-only with business logic reserved as placeholders.
 *
 * Architecture Overview:
 * - MainLayout: Orchestrator component that combines all modules
 * - LeftSidebar (Module A): Collapsible sidebar with workspace selector
 * - RightSidebar (Module C): Collapsible sidebar for Agent panel
 * - MainContent (Module D): Main content area with operation bar, views, and status bar
 *
 * Missing: Module B (Native Menu Bar) - Will be implemented via Tauri API
 */

// Main orchestrator component
export { MainLayout } from "./MainLayout";

// Left Sidebar components (Module A)
export { LeftSidebar, WorkspaceSelector, ScrollableList, UserOperations } from "./LeftSidebar";

// Right Sidebar components (Module C)
export { RightSidebar, AgentPanel } from "./RightSidebar";

// Main Content components (Module D)
export { MainContent, OperationBar, StatusBar } from "./MainContent";
export {
  AnalyzeView,
  QuantificationView,
  ComprehensiveMarketView,
  OptionsView,
  FuturesView,
  OtherDerivativesView,
} from "./MainContent";

// Type definitions
export type {
  WorkspaceType,
  SidebarState,
  UserMenuItem,
  MenuItemId,
  FileMenuItem,
  EditMenuItem,
  ViewMenuItem,
  HelpMenuItem,
  AppStatus,
  LeftSidebarProps,
  WorkspaceSelectorProps,
  ScrollableListProps,
  UserOperationsProps,
  RightSidebarProps,
  AgentPanelProps,
  MainContentProps,
  OperationBarProps,
  StatusBarProps,
  WorkspaceViewProps,
  MainLayoutProps,
  MenuBarConfig,
} from "./types";

// Constants
export { WORKSPACE_OPTIONS, DEFAULT_SIDEBAR_WIDTHS } from "./types";