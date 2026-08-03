import { useState } from "react";
import { ExternalLink, HardDriveDownload, RefreshCw, ShieldCheck } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { desktopApi } from "@/lib/desktop-api";
import { formatMessage, LOCALES, type Locale } from "@/lib/i18n/locale";
import { useLocale } from "@/lib/i18n/useLocale";

export function SettingsPage() {
  const { locale, setLocale, t } = useLocale();
  const [message, setMessage] = useState<string | null>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isChecking, setIsChecking] = useState(false);

  const exportBackup = async () => {
    setIsExporting(true);
    setMessage(null);
    try {
      const path = await desktopApi.system.exportLocalBackup();
      setMessage(path ? formatMessage(t("backupCreated"), { path }) : t("backupCancelled"));
    } catch {
      setMessage(t("backupFailed"));
    } finally {
      setIsExporting(false);
    }
  };

  const checkForUpdate = async () => {
    setIsChecking(true);
    setMessage(null);
    try {
      const release = await desktopApi.system.checkForUpdate();
      if (release.updateAvailable) {
        setMessage(formatMessage(t("updateAvailable"), { version: release.latestVersion }));
        await openUrl(release.releaseUrl);
      } else {
        setMessage(formatMessage(t("upToDate"), { version: release.currentVersion }));
      }
    } catch {
      setMessage(t("updateCheckFailed"));
    } finally {
      setIsChecking(false);
    }
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div>
        <h1 className="text-2xl font-bold">{t("settings")}</h1>
        <p className="mt-1 text-sm text-muted-foreground">{t("settingsDescription")}</p>
      </div>
      <section className="rounded-lg border border-border bg-card p-5">
        <h2 className="font-semibold">{t("language")}</h2>
        <p className="mt-1 text-sm text-muted-foreground">{t("languageDescription")}</p>
        <select
          aria-label={t("language")}
          className="mt-4 rounded-md border border-input bg-background px-3 py-2 text-sm"
          value={locale}
          onChange={(event) => {
            const newLocale = event.target.value as Locale;
            void setLocale(newLocale);
          }}
        >
          {LOCALES.map((option) => (
            <option key={option} value={option}>
              {option === "zh-CN" ? t("simplifiedChinese") : t("english")}
            </option>
          ))}
        </select>
      </section>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <HardDriveDownload className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">{t("localBackup")}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t("localBackupDescription")}</p>
          </div>
        </div>
        <button
          onClick={exportBackup}
          disabled={isExporting}
          className="mt-4 rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
        >
          {isExporting ? t("exporting") : t("exportLocalBackup")}
        </button>
      </section>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <RefreshCw className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">{t("updates")}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t("updatesDescription")}</p>
          </div>
        </div>
        <button
          onClick={checkForUpdate}
          disabled={isChecking}
          className="mt-4 rounded-md border border-input px-3 py-2 text-sm font-medium hover:bg-accent disabled:opacity-50"
        >
          {isChecking ? t("checking") : t("checkForUpdates")}
        </button>
      </section>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <ShieldCheck className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">{t("aboutAndPrivacy")}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t("aboutAndPrivacyDescription")}</p>
          </div>
        </div>
        <button
          onClick={() =>
            openUrl("https://github.com/BerryUIKI/alpha-forge/blob/dev/docs/PRIVACY.md")
          }
          className="mt-4 inline-flex items-center gap-2 text-sm font-medium text-primary hover:underline"
        >
          {t("openPrivacyNotice")} <ExternalLink className="h-4 w-4" />
        </button>
        <button
          onClick={() =>
            openUrl(
              "https://github.com/BerryUIKI/alpha-forge/blob/dev/docs/INVESTMENT_RESEARCH_DISCLAIMER.md",
            )
          }
          className="mt-4 ml-4 inline-flex items-center gap-2 text-sm font-medium text-primary hover:underline"
        >
          {t("openResearchDisclaimer")} <ExternalLink className="h-4 w-4" />
        </button>
      </section>
      {message && (
        <p role="status" className="rounded-md bg-muted p-3 text-sm">
          {message}
        </p>
      )}
    </div>
  );
}
