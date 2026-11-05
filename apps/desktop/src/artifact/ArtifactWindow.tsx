// Standalone artifact window.
//
// This entry is bundled separately from the main application and does not
// import desktopApi or app-shell code. It renders only the injected artifact
// payload with predefined components.

import type { Artifact } from "@/lib/desktop-api";
import { ArtifactRenderer } from "@/features/artifacts/components/ArtifactRenderer";

interface ArtifactWindowPayload {
  id: string;
  title: string;
  artifact_type: string;
  data: unknown;
  created_at: string;
}

declare global {
  interface Window {
    __ALPHAFORGE_ARTIFACT__?: ArtifactWindowPayload;
  }
}

export function ArtifactWindow() {
  const payload = window.__ALPHAFORGE_ARTIFACT__;

  if (!payload) {
    return (
      <main className="p-6 text-sm text-muted-foreground">No artifact data was provided.</main>
    );
  }

  const artifact: Artifact = {
    id: payload.id,
    task_id: null,
    workspace_id: null,
    artifact_type: payload.artifact_type,
    title: payload.title,
    data: JSON.stringify(payload.data),
    status: "completed",
    created_at: payload.created_at,
    updated_at: payload.created_at,
  };

  return (
    <main className="min-h-screen bg-background p-6 text-foreground">
      <header className="mb-4">
        <h1 className="text-xl font-semibold">{artifact.title}</h1>
        <p className="mt-1 text-xs text-muted-foreground">
          Created {new Date(artifact.created_at).toLocaleString()}
        </p>
      </header>
      <section className="rounded-lg border border-border bg-card p-4">
        <ArtifactRenderer artifact={artifact} />
      </section>
    </main>
  );
}
