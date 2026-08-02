import { useState } from "react";
import { ExternalLink, HardDriveDownload, RefreshCw, ShieldCheck } from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { desktopApi } from "@/lib/desktop-api";

export function SettingsPage() {
  const [message, setMessage] = useState<string | null>(null);
  const [isExporting, setIsExporting] = useState(false);
  const [isChecking, setIsChecking] = useState(false);

  const exportBackup = async () => {
    setIsExporting(true);
    setMessage(null);
    try {
      const path = await desktopApi.system.exportLocalBackup();
      setMessage(path ? `Backup created at ${path}` : "Backup export cancelled.");
    } catch {
      setMessage("Backup export failed. Choose a new writable filename and try again.");
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
        setMessage(`Version ${release.latestVersion} is available.`);
        await openUrl(release.releaseUrl);
      } else {
        setMessage(`You are up to date (${release.currentVersion}).`);
      }
    } catch {
      setMessage("Could not check GitHub Releases. Check your connection and try again.");
    } finally {
      setIsChecking(false);
    }
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div>
        <h1 className="text-2xl font-bold">Settings</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          Local-first controls, privacy information, and release updates.
        </p>
      </div>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <HardDriveDownload className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">Local backup</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Export a consistent SQLite backup. Existing files are never overwritten.
            </p>
          </div>
        </div>
        <button
          onClick={exportBackup}
          disabled={isExporting}
          className="mt-4 rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
        >
          {isExporting ? "Exporting…" : "Export local backup"}
        </button>
      </section>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <RefreshCw className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">Updates</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Checks GitHub Releases only when you request it. Updates are downloaded manually;
              nothing is installed automatically.
            </p>
          </div>
        </div>
        <button
          onClick={checkForUpdate}
          disabled={isChecking}
          className="mt-4 rounded-md border border-input px-3 py-2 text-sm font-medium hover:bg-accent disabled:opacity-50"
        >
          {isChecking ? "Checking…" : "Check for updates"}
        </button>
      </section>
      <section className="rounded-lg border border-border bg-card p-5">
        <div className="flex items-start gap-3">
          <ShieldCheck className="mt-0.5 h-5 w-5" />
          <div>
            <h2 className="font-semibold">About and privacy</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              AlphaForge is a local-first, open-source MVP. It has no account requirement, no
              automatic cloud backup, and telemetry is disabled by default. Your local database
              remains the source of truth.
            </p>
          </div>
        </div>
        <button
          onClick={() => openUrl("https://github.com/BerryUIKI/alpha-forge/blob/dev/docs/PRIVACY.md")}
          className="mt-4 inline-flex items-center gap-2 text-sm font-medium text-primary hover:underline"
        >
          Open privacy notice <ExternalLink className="h-4 w-4" />
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
