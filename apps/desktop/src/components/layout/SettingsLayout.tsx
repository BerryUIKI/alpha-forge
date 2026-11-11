import { useEffect, useRef } from "react";
import {
  ArrowLeft,
  Bot,
  Database,
  Info,
  Languages,
  Palette,
  Plug,
  Settings2,
  Key,
} from "lucide-react";
import { Link, Outlet, useLocation, useNavigate } from "react-router-dom";
import { useLocale } from "@/lib/i18n/useLocale";
import { readLastAppPath } from "@/lib/navigation/last-path";
import { WindowTitleBar } from "./WindowTitleBar";

const SETTINGS_SECTIONS = [
  { id: "general", labelKey: "settingsGeneral", icon: Settings2 },
  { id: "appearance", labelKey: "settingsAppearance", icon: Palette },
  { id: "localization", labelKey: "settingsLocalization", icon: Languages },
  { id: "usage", labelKey: "apiUsage", icon: Key },
  { id: "agent", labelKey: "settingsAgents", icon: Bot },
  { id: "data", labelKey: "settingsData", icon: Database },
  { id: "internal-plugins", labelKey: "settingsPlugins", icon: Plug },
  { id: "about", labelKey: "settingsAbout", icon: Info },
] as const;

function scrollContainerToSection(
  container: HTMLElement | null,
  sectionId: string,
  attempt = 0,
): void {
  if (!container) return;
  const target = document.getElementById(sectionId);
  if (!target) {
    // Sections can appear after asynchronous data resolves; retry a few frames.
    if (attempt < 20) {
      requestAnimationFrame(() => scrollContainerToSection(container, sectionId, attempt + 1));
    }
    return;
  }

  const top =
    target.getBoundingClientRect().top -
    container.getBoundingClientRect().top +
    container.scrollTop;
  container.scrollTo({ top });
}

export function SettingsLayout() {
  const { t } = useLocale();
  const navigate = useNavigate();
  const location = useLocation();
  const mainRef = useRef<HTMLElement>(null);

  const activeSection = location.hash.replace("#", "") || "general";

  // Reveal the selected section in the scrolling content container whenever
  // the hash changes (navigation, deep link, reload, or async section mount).
  useEffect(() => {
    if (!location.hash) return;
    const sectionId = location.hash.replace("#", "");
    requestAnimationFrame(() => scrollContainerToSection(mainRef.current, sectionId));
  }, [location.hash]);

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background">
      <WindowTitleBar />
      <div className="flex min-h-0 flex-1 overflow-hidden">
        <aside
          className="flex w-64 shrink-0 flex-col border-r border-border bg-card"
          aria-label={t("settingsNavigation")}
        >
          <div className="flex h-14 items-center gap-2 border-b border-border px-3">
            <button
              type="button"
              onClick={() => navigate(readLastAppPath(), { replace: true })}
              className="flex h-8 w-8 items-center justify-center rounded-lg hover:bg-accent"
              aria-label={t("backToApplication")}
              title={t("backToApplication")}
            >
              <ArrowLeft className="h-4 w-4" />
            </button>
            <span className="font-semibold">{t("settings")}</span>
          </div>
          <nav className="flex-1 space-y-1 overflow-y-auto p-3">
            {SETTINGS_SECTIONS.map(({ id, labelKey, icon: Icon }) => {
              const isActive = activeSection === id;
              return (
                <Link
                  key={id}
                  to={`/settings#${id}`}
                  replace
                  aria-current={isActive ? "page" : undefined}
                  className={
                    isActive
                      ? "flex items-center gap-2.5 rounded-lg bg-accent px-3 py-2 text-sm font-medium text-foreground"
                      : "flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
                  }
                >
                  <Icon className="h-4 w-4" aria-hidden="true" />
                  {t(labelKey)}
                </Link>
              );
            })}
          </nav>
        </aside>
        <main ref={mainRef} className="min-w-0 flex-1 overflow-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
