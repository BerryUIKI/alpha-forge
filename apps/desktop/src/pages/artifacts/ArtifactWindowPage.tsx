import { useEffect, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { X } from "lucide-react";
import { useParams } from "react-router-dom";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useArtifact } from "@/features/artifacts/hooks/useArtifacts";
import { artifactRegistry } from "@/features/artifacts/renderers";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";
import type { Artifact } from "@/lib/desktop-api/artifacts";

const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const ARTIFACT_TYPE_PATTERN = /^[a-z0-9_]{1,64}$/;

type ArtifactTheme = "light" | "dark" | "system";
interface ThemeClasses {
  dark: boolean;
  light: boolean;
}
interface LiveArtifactData {
  value: unknown;
}
interface ArtifactThemeEvent {
  theme?: unknown;
}

/**
 * The Rust window manager uses UUIDs and lowercase route segments. Keep the
 * same validation at the browser boundary so malformed URLs never trigger IPC.
 */
function isValidArtifactRoute(
  artifactId: string | undefined,
  artifactType: string | undefined,
): artifactId is string {
  return (
    artifactId !== undefined &&
    artifactType !== undefined &&
    UUID_PATTERN.test(artifactId) &&
    ARTIFACT_TYPE_PATTERN.test(artifactType)
  );
}

function parseTheme(payload: unknown): ArtifactTheme | null {
  if (!payload || typeof payload !== "object" || !("theme" in payload)) {
    return null;
  }

  const theme = (payload as ArtifactThemeEvent).theme;
  return theme === "light" || theme === "dark" || theme === "system" ? theme : null;
}

function artifactData(artifact: Artifact): unknown {
  return artifact.output ?? artifact.input;
}

export function ArtifactWindowPage() {
  const { artifactId, artifactType } = useParams<{
    artifactId: string;
    artifactType: string;
  }>();
  const { t } = useLocale();
  const routeIsValid = isValidArtifactRoute(artifactId, artifactType);
  // Passing an empty ID keeps the existing query disabled for unsafe routes.
  const artifactQuery = useArtifact(routeIsValid ? artifactId : "");
  const [liveData, setLiveData] = useState<LiveArtifactData | null>(null);
  const [theme, setTheme] = useState<ArtifactTheme>("system");
  const [closeError, setCloseError] = useState(false);
  const initialThemeClasses = useRef<ThemeClasses | null>(null);

  useEffect(() => {
    setLiveData(null);
    setTheme("system");
    setCloseError(false);
  }, [artifactId, artifactType]);

  useEffect(() => {
    if (!routeIsValid) {
      return undefined;
    }

    const root = document.documentElement;
    const initialClasses = {
      dark: root.classList.contains("dark"),
      light: root.classList.contains("light"),
    };
    initialThemeClasses.current = initialClasses;

    return () => {
      root.classList.toggle("dark", initialClasses.dark);
      root.classList.toggle("light", initialClasses.light);
      initialThemeClasses.current = null;
    };
  }, [routeIsValid, artifactId, artifactType]);

  useEffect(() => {
    if (!routeIsValid) {
      return;
    }

    const root = document.documentElement;
    const initialClasses = initialThemeClasses.current;
    if (!initialClasses) {
      return;
    }

    if (theme === "system") {
      root.classList.toggle("dark", initialClasses.dark);
      root.classList.toggle("light", initialClasses.light);
    } else {
      root.classList.toggle("dark", theme === "dark");
      root.classList.toggle("light", theme === "light");
    }
  }, [theme, routeIsValid, artifactId, artifactType]);

  useEffect(() => {
    if (!routeIsValid) {
      return undefined;
    }

    let disposed = false;
    const unlistenFns: UnlistenFn[] = [];

    const registerListeners = async () => {
      const results = await Promise.allSettled([
        listen<unknown>("artifact:update", (event) => {
          if (!disposed) {
            setLiveData({ value: event.payload });
          }
        }),
        listen<ArtifactThemeEvent>("artifact:theme", (event) => {
          if (!disposed) {
            const nextTheme = parseTheme(event.payload);
            if (nextTheme) {
              setTheme(nextTheme);
            }
          }
        }),
      ]);

      for (const result of results) {
        if (result.status === "fulfilled") {
          if (disposed) {
            result.value();
          } else {
            unlistenFns.push(result.value);
          }
        }
      }
    };

    void registerListeners();

    return () => {
      disposed = true;
      for (const unlisten of unlistenFns) {
        unlisten();
      }
    };
  }, [routeIsValid, artifactId, artifactType]);

  const handleClose = async () => {
    setCloseError(false);
    try {
      await getCurrentWindow().close();
    } catch {
      setCloseError(true);
    }
  };

  const shellProps = {
    theme,
    onClose: handleClose,
    closeLabel: t("closeArtifactWindow"),
    closeError,
    closeErrorMessage: t("artifactWindowCloseFailed"),
  };

  if (!routeIsValid) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <ErrorState
          title={t("artifactWindowInvalidRoute")}
          message={t("artifactWindowInvalidRouteDescription")}
        />
      </ArtifactWindowShell>
    );
  }

  if (artifactQuery.isLoading) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <LoadingSpinner className="h-full min-h-64" ariaLabel={t("loadingArtifact")} />
      </ArtifactWindowShell>
    );
  }

  if (artifactQuery.error) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <ErrorState
          title={t("errorLoadingArtifact")}
          message={t("failedToLoadArtifacts")}
          retryLabel={t("retry")}
          onRetry={() => void artifactQuery.refetch()}
        />
      </ArtifactWindowShell>
    );
  }

  const artifact = artifactQuery.data;
  if (!artifact) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <EmptyState title={t("artifactNotFound")} description={t("artifactNotFoundDescription")} />
      </ArtifactWindowShell>
    );
  }

  if (artifact.id !== artifactId || artifact.artifactType !== artifactType) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <ErrorState
          title={t("artifactWindowMismatch")}
          message={t("artifactWindowMismatchDescription")}
          retryLabel={t("retry")}
          onRetry={() => void artifactQuery.refetch()}
        />
      </ArtifactWindowShell>
    );
  }

  const Renderer = artifactRegistry.getRenderer(artifact.artifactType);
  if (!Renderer) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <EmptyState
          title={t("noRendererAvailable")}
          description={formatMessage(t("noRendererAvailableDescription"), {
            type: artifact.artifactType,
          })}
        />
      </ArtifactWindowShell>
    );
  }

  const data = liveData ?? { value: artifactData(artifact) };
  if (data.value === null || data.value === undefined) {
    return (
      <ArtifactWindowShell {...shellProps}>
        <EmptyState
          title={t("artifactWindowNoData")}
          description={t("artifactWindowNoDataDescription")}
        />
      </ArtifactWindowShell>
    );
  }

  return (
    <ArtifactWindowShell {...shellProps}>
      <div className="flex min-h-0 flex-1 flex-col">
        <header className="flex items-center justify-between border-b border-border px-4 py-3">
          <div>
            <h1 className="text-lg font-semibold">{artifact.artifactType}</h1>
            <p className="text-sm text-muted-foreground">
              {t("artifactStatus")}: {artifact.status}
            </p>
          </div>
        </header>
        <main className="min-h-0 flex-1 overflow-auto">
          <Renderer artifactId={artifact.id} data={data.value} />
        </main>
      </div>
    </ArtifactWindowShell>
  );
}

interface ArtifactWindowShellProps {
  theme: ArtifactTheme;
  closeLabel: string;
  onClose: () => void;
  closeError: boolean;
  closeErrorMessage: string;
  children: React.ReactNode;
}

function ArtifactWindowShell({
  theme,
  closeLabel,
  onClose,
  closeError,
  closeErrorMessage,
  children,
}: ArtifactWindowShellProps) {
  const className = "flex min-h-screen flex-col bg-background text-foreground";

  return (
    <div className={className} data-theme={theme}>
      <div className="flex justify-end border-b border-border px-2 py-1">
        {closeError && (
          <span role="alert" className="mr-3 self-center text-sm text-destructive">
            {closeErrorMessage}
          </span>
        )}
        <button
          type="button"
          className="rounded p-1 text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
          onClick={onClose}
          aria-label={closeLabel}
        >
          <X className="h-4 w-4" aria-hidden="true" />
        </button>
      </div>
      {children}
    </div>
  );
}
