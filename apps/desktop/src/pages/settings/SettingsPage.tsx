import { useState, useEffect } from "react";
import { ExternalLink, HardDriveDownload, RefreshCw, ShieldCheck, Activity, Database, Bot } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { desktopApi } from "@/lib/desktop-api";
import { formatMessage, LOCALES, type Locale } from "@/lib/i18n/locale";
import { useLocale } from "@/lib/i18n/useLocale";
import { InternalPluginsPanel } from "@/features/plugins";
import { ApiUsageSettings, AppearanceSettings, ProfessionalTerminologySettings } from "./SettingsPreferences";

export function SettingsPage() {
  const { locale, setLocale, t } = useLocale();
  const [message, setMessage] = useState<string | null>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isChecking, setIsChecking] = useState(false);

  // System info state
  const [dbHealth, setDbHealth] = useState<string | null>(null);
  const [isCheckingHealth, setIsCheckingHealth] = useState(false);

  // Agent config state
  const [apiKey, setApiKey] = useState("");
  const [isSavingAgent, setIsSavingAgent] = useState(false);
  const [agentMessage, setAgentMessage] = useState<string | null>(null);
  const [hasExistingKey, setHasExistingKey] = useState(false);

  // Check if API key exists in secure storage on mount
  useEffect(() => {
    desktopApi.credentials
      .hasOpenAiApiKey()
      .then(setHasExistingKey)
      .catch(() => setAgentMessage(t("agentConfigError")));
  }, [t]);

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

  const checkDatabaseHealth = async () => {
    setIsCheckingHealth(true);
    setMessage(null);
    try {
      const result = await desktopApi.system.checkDatabaseHealth();
      setDbHealth(result);
      setMessage(t("databaseHealthy"));
    } catch {
      setDbHealth("error");
      setMessage(t("databaseCheckFailed"));
    } finally {
      setIsCheckingHealth(false);
    }
  };

  const saveAgentConfig = async () => {
    setIsSavingAgent(true);
    setAgentMessage(null);
    try {
      // ✅ SECURE: Save API key to OS keychain instead of plaintext database
      await desktopApi.credentials.saveOpenAiApiKey(apiKey);
      setHasExistingKey(true);
      setApiKey("");
      setAgentMessage(t("agentConfigSaved"));
    } catch {
      setAgentMessage(t("agentConfigError"));
    } finally {
      setIsSavingAgent(false);
    }
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div id="general" className="scroll-mt-4">
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

      <AppearanceSettings />
      <ProfessionalTerminologySettings />
      <ApiUsageSettings />

      {/* Agent Configuration Section */}
      <section id="agent" className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <Bot className="mt-0.5 h-5 w-5" />
          <div className="flex-1">
            <h2 className="font-semibold">{t("agentConfig")}</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              {t("agentConfigDescription")}
            </p>
          </div>
        </div>
        <div className="mt-4 space-y-4">
          <div>
            <label htmlFor="api-key" className="block text-sm font-medium">
              API Key
            </label>
            <input
              id="api-key"
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder={hasExistingKey ? t("apiKeyPlaceholder") : "sk-..."}
              className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
            />
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={saveAgentConfig}
              disabled={isSavingAgent || !apiKey.trim()}
              className="rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
            >
              {isSavingAgent ? t("saving") : t("save")}
            </button>
            {agentMessage && (
              <span className="text-sm text-muted-foreground">{agentMessage}</span>
            )}
          </div>
        </div>
      </section>

      <div id="internal-plugins" className="scroll-mt-4"><InternalPluginsPanel /></div>

      <section id="data" className="scroll-mt-4 rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <Database className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">{t("databaseHealth")}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t("databaseHealthDescription")}</p>
          </div>
        </div>
        <div className="mt-4 flex items-center gap-4">
          <button
            onClick={checkDatabaseHealth}
            disabled={isCheckingHealth}
            className="rounded-md border border-input px-3 py-2 text-sm font-medium hover:bg-accent disabled:opacity-50"
          >
            {isCheckingHealth ? t("checking") : t("checkDatabaseHealth")}
          </button>
          {dbHealth && (
            <span className={`flex items-center gap-1.5 text-sm ${dbHealth === "healthy" ? "text-green-600" : "text-destructive"}`}>
              <Activity className="h-4 w-4" />
              {dbHealth === "healthy" ? t("databaseStatusHealthy") : t("databaseStatusError")}
            </span>
          )}
        </div>
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
      <section id="about" className="scroll-mt-4 rounded-lg border border-border bg-card p-5">
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
