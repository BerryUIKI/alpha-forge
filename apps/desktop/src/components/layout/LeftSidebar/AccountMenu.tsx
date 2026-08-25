import { useEffect, useRef, useState } from "react";
import { Activity, Check, Moon, Settings, Sun, UserRound } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useTheme } from "next-themes";
import { useActiveWorkspace } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { useLocale } from "@/lib/i18n/useLocale";

export function AccountMenu({ collapsed }: { collapsed: boolean }) {
  const { t } = useLocale();
  const { resolvedTheme, setTheme } = useTheme();
  const { workspaceId, workspaces, setActiveWorkspace } = useActiveWorkspace();
  const navigate = useNavigate();
  const rootRef = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    if (!open) return;
    const closeOnOutsideClick = (event: MouseEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", closeOnOutsideClick);
    return () => document.removeEventListener("mousedown", closeOnOutsideClick);
  }, [open]);

  const goTo = (path: string) => {
    navigate(path);
    setOpen(false);
  };

  return (
    <div ref={rootRef} className="relative">
      <button
        type="button"
        onClick={() => setOpen((value) => !value)}
        className="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
        aria-label={t("accountMenu")}
        aria-expanded={open}
        title={t("accountMenu")}
      >
        <UserRound className="h-4 w-4" />
      </button>

      {open && (
        <div
          className={`absolute bottom-11 z-50 w-64 rounded-xl border border-border bg-popover p-1.5 shadow-xl ${collapsed ? "left-0" : "right-0"}`}
          role="menu"
          aria-label={t("accountMenu")}
        >
          <div className="px-2.5 py-2 text-xs font-medium uppercase tracking-wide text-muted-foreground">
            {t("switchPortfolio")}
          </div>
          <div className="max-h-40 overflow-y-auto">
            {workspaces.map((workspace) => (
              <button
                key={workspace.id}
                type="button"
                onClick={() => {
                  setActiveWorkspace(workspace.id);
                  setOpen(false);
                }}
                className="flex w-full items-center justify-between rounded-lg px-2.5 py-2 text-left text-sm hover:bg-accent"
                role="menuitemradio"
                aria-checked={workspace.id === workspaceId}
              >
                <span className="truncate">{workspace.name}</span>
                {workspace.id === workspaceId && <Check className="h-4 w-4" />}
              </button>
            ))}
          </div>
          <div className="my-1 h-px bg-border" />
          <button
            type="button"
            onClick={() => goTo("/settings#usage")}
            className="flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-sm hover:bg-accent"
            role="menuitem"
          >
            <Activity className="h-4 w-4" />
            {t("apiUsage")}
          </button>
          <button
            type="button"
            onClick={() => {
              setTheme(resolvedTheme === "dark" ? "light" : "dark");
              setOpen(false);
            }}
            className="flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-sm hover:bg-accent"
            role="menuitem"
          >
            {resolvedTheme === "dark" ? <Sun className="h-4 w-4" /> : <Moon className="h-4 w-4" />}
            {resolvedTheme === "dark" ? t("lightMode") : t("darkMode")}
          </button>
          <button
            type="button"
            onClick={() => goTo("/settings")}
            className="flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-sm hover:bg-accent"
            role="menuitem"
          >
            <Settings className="h-4 w-4" />
            {t("settings")}
          </button>
        </div>
      )}
    </div>
  );
}
