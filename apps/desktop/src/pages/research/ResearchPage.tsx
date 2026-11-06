// Research page — workspace documents.

import { useState } from "react";
import { FolderOpen } from "lucide-react";
import { WorkspaceList } from "@/features/workspace";
import { ResearchWorkspacePanel } from "@/features/research";
import type { Workspace } from "@/lib/desktop-api/workspace";

export function ResearchPage() {
  const [selectedWorkspace, setSelectedWorkspace] = useState<Workspace | null>(null);

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">Research</h1>
        <p className="mt-2 text-muted-foreground">
          Documents and findings for your research workspaces.
        </p>
      </div>

      {!selectedWorkspace ? (
        <div className="max-w-2xl">
          <div className="mb-4 flex items-center gap-2">
            <FolderOpen className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">Select a Workspace</h2>
          </div>
          <p className="mb-6 text-sm text-muted-foreground">
            Choose a workspace to manage its research documents.
          </p>
          <WorkspaceList onSelect={setSelectedWorkspace} />
        </div>
      ) : (
        <div className="max-w-5xl">
          <div className="mb-6 flex flex-wrap items-center justify-between gap-3 rounded-lg border border-border bg-card p-4">
            <div>
              <p className="text-xs uppercase tracking-wide text-muted-foreground">Workspace</p>
              <h2 className="text-lg font-semibold">{selectedWorkspace.name}</h2>
            </div>
            <button
              type="button"
              onClick={() => setSelectedWorkspace(null)}
              className="text-sm text-primary hover:underline"
            >
              Change workspace
            </button>
          </div>
          <ResearchWorkspacePanel workspaceId={selectedWorkspace.id} />
        </div>
      )}
    </div>
  );
}
