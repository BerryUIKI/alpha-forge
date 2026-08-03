/**
 * Layout component type definitions for GUI-M1-0
 *
 * This module defines the core layout structure following the Codex-style paradigm.
 * All components are UI-only with business logic reserved as placeholders.
 */

/**
 * Workspace types available in the application
 * Corresponds to Module A workspace selector dropdown
 */
export type WorkspaceType =
  | "analyze"
  | "quantification"
  | "comprehensive-market"
  | "options"
  | "futures"
  | "other-derivatives";

/**
 * Sidebar collapse state
 */
export type SidebarState = "expanded" | "collapsed";

/**
 * User menu options in the bottom user operations area
 */
export type UserMenuItem =
  | "profile"
  | "theme-toggle"
  | "language"
  | "settings";

/**
 * Menu bar item identifiers
 * Corresponds to Module B native menu structure
 */
export type MenuItemId =
  | "file"
  | "edit"
  | "view"
  | "help";

/**
 * File menu items
 */
export type FileMenuItem =
  | "new-workspace"
  | "open-workspace"
  | "save"
  | "export"
  | "exit";

/**
 * Edit menu items
 */
export type EditMenuItem =
  | "undo"
  | "redo"
  | "cut"
  | "copy"
  | "paste";

/**
 * View menu items
 */
export type ViewMenuItem =
  | "toggle-left-sidebar"
  | "toggle-right-sidebar"
  | "zoom-in"
  | "zoom-out"
  | "reset-zoom";

/**
 * Help menu items
 */
export type HelpMenuItem =
  | "documentation"
  | "keyboard-shortcuts"
  | "report-issue"
  | "about";

/**
 * Application status displayed in status bar
 */
export type AppStatus = "idle" | "running" | "error" | "syncing";

/**
 * Left sidebar props
 */
export interface LeftSidebarProps {
  /** Current expanded/collapsed state */
  state?: SidebarState;
  /** Callback when sidebar state changes */
  onStateChange?: (state: SidebarState) => void;
  /** Current selected workspace type */
  selectedWorkspace?: WorkspaceType;
  /** Callback when workspace selection changes */
  onWorkspaceChange?: (workspace: WorkspaceType) => void;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width when expanded */
  minWidth?: number;
  /** Maximum width when expanded */
  maxWidth?: number;
}

/**
 * Workspace selector dropdown props
 */
export interface WorkspaceSelectorProps {
  /** Currently selected workspace */
  selected?: WorkspaceType;
  /** Callback when selection changes */
  onSelect?: (workspace: WorkspaceType) => void;
  /** Whether the dropdown is open */
  isOpen?: boolean;
  /** Callback when dropdown state changes */
  onOpenChange?: (open: boolean) => void;
}

/**
 * Scrollable list props (middle section of left sidebar)
 */
export interface ScrollableListProps {
  /** List items to display (placeholder content) */
  items?: Array<{ id: string; label: string; icon?: unknown }>;
  /** Selected item ID */
  selectedId?: string;
  /** Callback when item is selected */
  onSelect?: (id: string) => void;
  /** Empty state message */
  emptyMessage?: string;
}

/**
 * User operations menu props (bottom section of left sidebar)
 */
export interface UserOperationsProps {
  /** Current username */
  username?: string;
  /** Whether the user menu is open */
  isMenuOpen?: boolean;
  /** Callback when menu state changes */
  onMenuOpenChange?: (open: boolean) => void;
  /** Callback when menu item is clicked */
  onMenuItemClick?: (item: UserMenuItem) => void;
  /** Current theme mode */
  theme?: "light" | "dark";
}

/**
 * Right sidebar (Agent panel) props
 */
export interface RightSidebarProps {
  /** Current expanded/collapsed state */
  state?: SidebarState;
  /** Callback when sidebar state changes */
  onStateChange?: (state: SidebarState) => void;
  /** Default width in pixels */
  defaultWidth?: number;
  /** Minimum width when expanded */
  minWidth?: number;
  /** Maximum width when expanded */
  maxWidth?: number;
}

/**
 * Agent panel props (content of right sidebar)
 */
export interface AgentPanelProps {
  /** Agent status message */
  status?: string;
  /** Placeholder content */
  placeholder?: string;
}

/**
 * Main content area props
 */
export interface MainContentProps {
  /** Current active workspace */
  activeWorkspace?: WorkspaceType;
  /** Whether right sidebar is visible */
  isRightSidebarVisible?: boolean;
  /** Callback to toggle right sidebar */
  onToggleRightSidebar?: () => void;
}

/**
 * Operation bar props (top of main content)
 */
export interface OperationBarProps {
  /** Current context title */
  title?: string;
  /** Whether right sidebar is expanded */
  isRightSidebarExpanded?: boolean;
  /** Callback to toggle right sidebar */
  onToggleRightSidebar?: () => void;
}

/**
 * Status bar props (bottom of main content)
 */
export interface StatusBarProps {
  /** Currently active workspace name */
  workspaceName?: string;
  /** Application running status */
  status?: AppStatus;
  /** Hint text to display */
  hint?: string;
}

/**
 * Workspace view props
 */
export interface WorkspaceViewProps {
  /** Workspace type */
  workspace: WorkspaceType;
  /** Whether this is the active view */
  isActive?: boolean;
}

/**
 * Main layout props (orchestrator component)
 */
export interface MainLayoutProps {
  /** Initial left sidebar state */
  leftSidebarState?: SidebarState;
  /** Initial right sidebar state */
  rightSidebarState?: SidebarState;
  /** Initial workspace type */
  initialWorkspace?: WorkspaceType;
}

/**
 * Menu bar configuration
 * This is used to configure the Tauri native menu
 */
export interface MenuBarConfig {
  /** File menu items */
  file: FileMenuItem[];
  /** Edit menu items */
  edit: EditMenuItem[];
  /** View menu items */
  view: ViewMenuItem[];
  /** Help menu items */
  help: HelpMenuItem[];
}

/**
 * Default workspace options for the selector dropdown
 */
export const WORKSPACE_OPTIONS: Array<{ value: WorkspaceType; label: string }> = [
  { value: "analyze", label: "Analyze" },
  { value: "quantification", label: "Quantification" },
  { value: "comprehensive-market", label: "Comprehensive Market" },
  { value: "options", label: "Options" },
  { value: "futures", label: "Futures" },
  { value: "other-derivatives", label: "Other Derivatives" },
];

/**
 * Default sidebar widths
 */
export const DEFAULT_SIDEBAR_WIDTHS = {
  left: {
    default: 240,
    min: 180,
    max: 400,
  },
  right: {
    default: 280,
    min: 200,
    max: 400,
  },
} as const;