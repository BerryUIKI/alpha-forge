import { useEffect, useRef, useState } from "react";
import { Minus, PanelLeftClose, PanelLeftOpen, Square, X } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { desktopApi } from "@/lib/desktop-api";

type MenuName = "File" | "Edit" | "View" | "Help";

interface MenuAction {
  label: string;
  action?: () => void;
  disabled?: boolean;
  separatorBefore?: boolean;
}

export interface WindowTitleBarProps {
  isLeftSidebarExpanded?: boolean;
  onToggleLeftSidebar?: () => void;
  onToggleRightSidebar?: () => void;
  onOpenSearch?: () => void;
}

function executeEditCommand(command: string) {
  if (typeof document.execCommand === "function") {
    document.execCommand(command);
  }
}

function runWindowAction(label: string, action: () => Promise<void>) {
  void action().catch(() => {
    console.error(`Window action failed: ${label}`);
  });
}

export function WindowTitleBar({
  isLeftSidebarExpanded = true,
  onToggleLeftSidebar,
  onToggleRightSidebar,
  onOpenSearch,
}: WindowTitleBarProps) {
  const navigate = useNavigate();
  const rootRef = useRef<HTMLElement>(null);
  const [openMenu, setOpenMenu] = useState<MenuName | null>(null);

  useEffect(() => {
    if (!openMenu) return;

    const closeOnOutsideClick = (event: MouseEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) setOpenMenu(null);
    };
    const closeOnEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") setOpenMenu(null);
    };

    document.addEventListener("mousedown", closeOnOutsideClick);
    document.addEventListener("keydown", closeOnEscape);
    return () => {
      document.removeEventListener("mousedown", closeOnOutsideClick);
      document.removeEventListener("keydown", closeOnEscape);
    };
  }, [openMenu]);

  const selectAction = (action?: () => void) => {
    setOpenMenu(null);
    action?.();
  };

  const menus: Record<MenuName, MenuAction[]> = {
    File: [
      { label: "New Research", action: () => navigate("/research") },
      { label: "Settings", action: () => navigate("/settings") },
      {
        label: "Exit AlphaForge",
        separatorBefore: true,
        action: () => runWindowAction("close", desktopApi.window.close),
      },
    ],
    Edit: [
      { label: "Undo", action: () => executeEditCommand("undo") },
      { label: "Redo", action: () => executeEditCommand("redo") },
      { label: "Cut", separatorBefore: true, action: () => executeEditCommand("cut") },
      { label: "Copy", action: () => executeEditCommand("copy") },
      { label: "Paste", action: () => executeEditCommand("paste") },
    ],
    View: [
      {
        label: isLeftSidebarExpanded ? "Collapse Navigation" : "Expand Navigation",
        action: onToggleLeftSidebar,
        disabled: !onToggleLeftSidebar,
      },
      {
        label: "Toggle Agent Panel",
        action: onToggleRightSidebar,
        disabled: !onToggleRightSidebar,
      },
      {
        label: "Open Search",
        separatorBefore: true,
        action: onOpenSearch,
        disabled: !onOpenSearch,
      },
    ],
    Help: [
      { label: "Keyboard Shortcuts", action: onOpenSearch, disabled: !onOpenSearch },
      {
        label: "About AlphaForge",
        separatorBefore: true,
        action: () => navigate("/settings#about"),
      },
    ],
  };

  return (
    <header
      ref={rootRef}
      className="relative z-50 flex h-9 shrink-0 select-none items-stretch border-b border-border bg-background/95 text-foreground"
      aria-label="Application title bar"
    >
      {onToggleLeftSidebar ? (
        <button
          type="button"
          onClick={onToggleLeftSidebar}
          className="flex w-11 items-center justify-center text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label={isLeftSidebarExpanded ? "Collapse sidebar" : "Expand sidebar"}
          title={isLeftSidebarExpanded ? "Collapse sidebar (Ctrl+1)" : "Expand sidebar (Ctrl+1)"}
          data-window-interactive
        >
          {isLeftSidebarExpanded ? (
            <PanelLeftClose className="h-4 w-4" />
          ) : (
            <PanelLeftOpen className="h-4 w-4" />
          )}
        </button>
      ) : (
        <div className="w-2" aria-hidden="true" />
      )}

      <div
        className="flex items-center px-2 text-xs font-semibold tracking-wide"
        data-tauri-drag-region
      >
        AlphaForge
      </div>

      <nav className="flex items-stretch" aria-label="Application menu" role="menubar">
        {(Object.keys(menus) as MenuName[]).map((menuName) => (
          <div key={menuName} className="relative flex items-stretch">
            <button
              type="button"
              className="px-2.5 text-xs text-muted-foreground transition-colors hover:bg-accent hover:text-foreground data-[open=true]:bg-accent data-[open=true]:text-foreground"
              aria-haspopup="menu"
              aria-expanded={openMenu === menuName}
              data-open={openMenu === menuName}
              data-window-interactive
              onClick={() => setOpenMenu((current) => (current === menuName ? null : menuName))}
            >
              {menuName}
            </button>

            {openMenu === menuName && (
              <div
                className="absolute left-0 top-full z-50 min-w-52 rounded-b-lg border border-t-0 border-border bg-popover p-1.5 shadow-xl"
                role="menu"
                aria-label={`${menuName} menu`}
              >
                {menus[menuName].map((item) => (
                  <div key={item.label}>
                    {item.separatorBefore && <div className="my-1 h-px bg-border" />}
                    <button
                      type="button"
                      className="flex w-full items-center rounded-md px-2.5 py-1.5 text-left text-xs transition-colors hover:bg-accent disabled:cursor-not-allowed disabled:opacity-40"
                      role="menuitem"
                      disabled={item.disabled}
                      onClick={() => selectAction(item.action)}
                    >
                      {item.label}
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </nav>

      <div
        className="min-w-12 flex-1"
        data-tauri-drag-region
        aria-label="Window drag region"
        onDoubleClick={() => runWindowAction("toggle maximize", desktopApi.window.toggleMaximize)}
      />

      <div className="flex items-stretch" aria-label="Window controls" data-window-interactive>
        <button
          type="button"
          onClick={() => runWindowAction("minimize", desktopApi.window.minimize)}
          className="flex w-11 items-center justify-center text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label="Minimize window"
          title="Minimize"
        >
          <Minus className="h-4 w-4" />
        </button>
        <button
          type="button"
          onClick={() => runWindowAction("toggle maximize", desktopApi.window.toggleMaximize)}
          className="flex w-11 items-center justify-center text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
          aria-label="Maximize or restore window"
          title="Maximize or restore"
        >
          <Square className="h-3.5 w-3.5" />
        </button>
        <button
          type="button"
          onClick={() => runWindowAction("close", desktopApi.window.close)}
          className="flex w-11 items-center justify-center text-muted-foreground transition-colors hover:bg-red-600 hover:text-white"
          aria-label="Close window"
          title="Close"
        >
          <X className="h-4 w-4" />
        </button>
      </div>
    </header>
  );
}
