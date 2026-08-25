import { ArrowLeft, Bot, Database, Info, Languages, Palette, Plug, Settings2 } from "lucide-react";
import { NavLink, Outlet, useNavigate } from "react-router-dom";
import { useLocale } from "@/lib/i18n/useLocale";
import { WindowTitleBar } from "./WindowTitleBar";

const SETTINGS_SECTIONS = [
  { id: "general", labelKey: "settingsGeneral", icon: Settings2 },
  { id: "appearance", labelKey: "settingsAppearance", icon: Palette },
  { id: "localization", labelKey: "settingsLocalization", icon: Languages },
  { id: "agent", labelKey: "settingsAgents", icon: Bot },
  { id: "data", labelKey: "settingsData", icon: Database },
  { id: "internal-plugins", labelKey: "settingsPlugins", icon: Plug },
  { id: "about", labelKey: "settingsAbout", icon: Info },
] as const;

export function SettingsLayout() {
  const { t } = useLocale();
  const navigate = useNavigate();

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background">
      <WindowTitleBar />
      <div className="flex min-h-0 flex-1 overflow-hidden">
        <aside className="flex w-64 shrink-0 flex-col border-r border-border bg-card" aria-label={t("settingsNavigation")}>
          <div className="flex h-14 items-center gap-2 border-b border-border px-3">
            <button
              type="button"
              onClick={() => navigate(-1)}
              className="flex h-8 w-8 items-center justify-center rounded-lg hover:bg-accent"
              aria-label={t("backToApplication")}
              title={t("backToApplication")}
            >
              <ArrowLeft className="h-4 w-4" />
            </button>
            <span className="font-semibold">{t("settings")}</span>
          </div>
          <nav className="flex-1 space-y-1 overflow-y-auto p-3">
            {SETTINGS_SECTIONS.map(({ id, labelKey, icon: Icon }) => (
              <NavLink
                key={id}
                to={`/settings#${id}`}
                className="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
              >
                <Icon className="h-4 w-4" />
                {t(labelKey)}
              </NavLink>
            ))}
          </nav>
        </aside>
        <main className="min-w-0 flex-1 overflow-auto">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
