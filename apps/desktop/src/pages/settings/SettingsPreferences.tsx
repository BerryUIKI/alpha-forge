import { useEffect, useState } from "react";
import { useTheme } from "next-themes";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { DEFAULT_PROFESSIONAL_TERMS, type ProfessionalTerm, useProfessionalTerms } from "@/lib/i18n/professional-terms";

const ACCENT_KEY = "app.theme.accent";
const MARKET_COLORS_KEY = "app.theme.marketColors";
const ACCENTS = ["indigo", "blue", "emerald", "amber", "rose"] as const;

export function AppearanceSettings() {
  const { t } = useLocale();
  const { theme, setTheme } = useTheme();
  const [accent, setAccent] = useState("indigo");
  const [marketColors, setMarketColors] = useState("global");

  useEffect(() => {
    void Promise.all([
      desktopApi.settings.getSetting(ACCENT_KEY),
      desktopApi.settings.getSetting(MARKET_COLORS_KEY),
    ]).then(([storedAccent, storedMarketColors]) => {
      if (storedAccent && ACCENTS.includes(storedAccent as (typeof ACCENTS)[number])) setAccent(storedAccent);
      if (storedMarketColors === "china" || storedMarketColors === "global") setMarketColors(storedMarketColors);
    }).catch(() => undefined);
  }, []);

  const updateAccent = (value: string) => {
    setAccent(value);
    document.documentElement.dataset.accent = value;
    void desktopApi.settings.setSetting(ACCENT_KEY, value);
  };

  const updateMarketColors = (value: string) => {
    setMarketColors(value);
    document.documentElement.dataset.marketColors = value;
    void desktopApi.settings.setSetting(MARKET_COLORS_KEY, value);
  };

  return (
    <section id="appearance" className="scroll-mt-4 rounded-lg border border-border bg-card p-5">
      <h2 className="font-semibold">{t("settingsAppearance")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("appearanceDescription")}</p>
      <div className="mt-4 grid gap-5 sm:grid-cols-2">
        <label className="text-sm font-medium">
          {t("themeMode")}
          <select className="mt-2 w-full rounded-md border border-input bg-background px-3 py-2" value={theme} onChange={(event) => setTheme(event.target.value)}>
            <option value="system">{t("followSystem")}</option>
            <option value="light">{t("lightMode")}</option>
            <option value="dark">{t("darkMode")}</option>
          </select>
        </label>
        <label className="text-sm font-medium">
          {t("marketColorScheme")}
          <select className="mt-2 w-full rounded-md border border-input bg-background px-3 py-2" value={marketColors} onChange={(event) => updateMarketColors(event.target.value)}>
            <option value="global">{t("marketColorsGlobal")}</option>
            <option value="china">{t("marketColorsChina")}</option>
          </select>
        </label>
      </div>
      <fieldset className="mt-5">
        <legend className="text-sm font-medium">{t("accentColor")}</legend>
        <div className="mt-2 flex gap-2">
          {ACCENTS.map((value) => (
            <button key={value} type="button" onClick={() => updateAccent(value)} aria-label={`${t("accentColor")}: ${value}`} aria-pressed={accent === value} className={`h-8 w-8 rounded-full border-2 ${accent === value ? "border-foreground" : "border-transparent"}`}>
              <span className={`block h-full w-full rounded-full accent-swatch-${value}`} />
            </button>
          ))}
        </div>
      </fieldset>
    </section>
  );
}

export function ProfessionalTerminologySettings() {
  const { t } = useLocale();
  const { enabled, overrides, setEnabled, setOverride } = useProfessionalTerms();
  const terms = Object.keys(DEFAULT_PROFESSIONAL_TERMS) as ProfessionalTerm[];

  return (
    <section id="localization" className="scroll-mt-4 rounded-lg border border-border bg-card p-5">
      <h2 className="font-semibold">{t("professionalTerminology")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("professionalTerminologyDescription")}</p>
      <label className="mt-4 flex items-center gap-3 text-sm">
        <input type="checkbox" checked={enabled} onChange={(event) => void setEnabled(event.target.checked)} />
        {t("enableProfessionalTerminology")}
      </label>
      <details className="mt-4 rounded-lg border border-border p-3">
        <summary className="cursor-pointer text-sm font-medium">{t("editTerminology")}</summary>
        <div className="mt-3 grid gap-3 sm:grid-cols-2">
          {terms.map((term) => (
            <label key={term} className="text-xs font-medium text-muted-foreground">
              {term}
              <input
                defaultValue={overrides[term] ?? DEFAULT_PROFESSIONAL_TERMS[term]}
                onBlur={(event) => void setOverride(term, event.target.value)}
                className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground"
              />
            </label>
          ))}
        </div>
      </details>
    </section>
  );
}

export function ApiUsageSettings() {
  const { t } = useLocale();
  return (
    <section id="usage" className="scroll-mt-4 rounded-lg border border-border bg-card p-5">
      <h2 className="font-semibold">{t("apiUsage")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("apiUsageNotCollecting")}</p>
    </section>
  );
}
