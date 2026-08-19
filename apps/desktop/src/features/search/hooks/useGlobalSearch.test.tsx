// Tests for useGlobalSearch.

import "@testing-library/jest-dom";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { PropsWithChildren } from "react";
import { useGlobalSearch } from "./useGlobalSearch";
import type { Artifact } from "@/lib/desktop-api/artifacts";
import type { ResearchProject } from "@/lib/desktop-api/research";
import type { InvestmentThesis } from "@/lib/desktop-api/thesis";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: { listWorkspaces: vi.fn() },
    research: {
      listResearchProjects: vi.fn(),
      listResearchDocuments: vi.fn(),
      listResearchReports: vi.fn(),
    },
    thesis: { listTheses: vi.fn() },
    knowledgeGraph: { listKnowledgeEntities: vi.fn() },
    artifacts: { listArtifacts: vi.fn() },
  },
}));

const { desktopApi } = await import("@/lib/desktop-api");

const mocks = desktopApi as unknown as {
  workspace: { listWorkspaces: ReturnType<typeof vi.fn> };
  research: {
    listResearchProjects: ReturnType<typeof vi.fn>;
    listResearchDocuments: ReturnType<typeof vi.fn>;
    listResearchReports: ReturnType<typeof vi.fn>;
  };
  thesis: { listTheses: ReturnType<typeof vi.fn> };
  knowledgeGraph: { listKnowledgeEntities: ReturnType<typeof vi.fn> };
  artifacts: { listArtifacts: ReturnType<typeof vi.fn> };
};

const projects: ResearchProject[] = [
  {
    id: "project-1",
    workspace_id: "workspace-1",
    title: "AI Infrastructure",
    description: "Demand for compute",
    status: "active",
    created_at: "2026-08-01T00:00:00Z",
    updated_at: "2026-08-01T00:00:00Z",
  },
  {
    id: "project-2",
    workspace_id: "workspace-1",
    title: "Semiconductors",
    description: null,
    status: "active",
    created_at: "2026-08-01T00:00:00Z",
    updated_at: "2026-08-01T00:00:00Z",
  },
];

const theses: InvestmentThesis[] = [
  {
    id: "thesis-1",
    workspace_id: "workspace-1",
    title: "GPU demand stays strong",
    thesis: "Hyperscaler capex continues to grow",
    confidence: 70,
    status: "active",
    validation_date: null,
    outcome: null,
    created_at: "2026-08-01T00:00:00Z",
    updated_at: "2026-08-01T00:00:00Z",
  },
];

const artifacts: Artifact[] = [
  {
    id: "artifact-1",
    workspace_id: "workspace-1",
    task_id: null,
    artifact_type: "comparison_table",
    status: "completed",
    input: { companies: ["NVDA", "AMD"] },
    output: null,
    error: null,
    created_at: "2026-08-01T00:00:00Z",
    updated_at: "2026-08-01T00:00:00Z",
  },
];

function wrapper({ children }: PropsWithChildren) {
  const client = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return (
    <QueryClientProvider client={client}>
      <MemoryRouter initialEntries={["/research"]}>{children}</MemoryRouter>
    </QueryClientProvider>
  );
}

describe("useGlobalSearch", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.workspace.listWorkspaces.mockResolvedValue([
      { id: "workspace-1", name: "Workspace" },
    ]);
    mocks.research.listResearchProjects.mockResolvedValue(projects);
    mocks.research.listResearchDocuments.mockResolvedValue([]);
    mocks.research.listResearchReports.mockResolvedValue([]);
    mocks.thesis.listTheses.mockResolvedValue(theses);
    mocks.knowledgeGraph.listKnowledgeEntities.mockResolvedValue([]);
    mocks.artifacts.listArtifacts.mockResolvedValue(artifacts);
  });

  it("returns no sections for an empty query", async () => {
    const { result } = renderHook(() => useGlobalSearch(""), { wrapper });

    await waitFor(() => expect(result.current.workspaceId).toBe("workspace-1"));
    expect(result.current.total).toBe(0);
    expect(result.current.sections).toEqual([]);
  });

  it("filters projects by title with a deep link to the project", async () => {
    const { result } = renderHook(() => useGlobalSearch("AI Infra"), { wrapper });

    await waitFor(() => expect(result.current.total).toBeGreaterThan(0));
    const projectSection = result.current.sections.find((s) => s.id === "projects");
    expect(projectSection?.entries).toHaveLength(1);
    expect(projectSection?.entries[0]).toMatchObject({
      title: "AI Infrastructure",
      to: "/research?workspace=workspace-1&project=project-1",
    });
  });

  it("filters theses by thesis body and links to the journal page", async () => {
    const { result } = renderHook(() => useGlobalSearch("hyperscaler"), { wrapper });

    await waitFor(() => expect(result.current.total).toBeGreaterThan(0));
    const thesisSection = result.current.sections.find((s) => s.id === "theses");
    expect(thesisSection?.entries).toHaveLength(1);
    expect(thesisSection?.entries[0]).toMatchObject({
      title: "GPU demand stays strong",
      to: "/journal",
    });
  });

  it("filters artifacts by input payload and links to the artifact window", async () => {
    const { result } = renderHook(() => useGlobalSearch("NVDA"), { wrapper });

    await waitFor(() => expect(result.current.total).toBeGreaterThan(0));
    const artifactSection = result.current.sections.find((s) => s.id === "artifacts");
    expect(artifactSection?.entries).toHaveLength(1);
    expect(artifactSection?.entries[0]).toMatchObject({
      title: "comparison_table",
      to: "/artifact/artifact-1/comparison_table",
    });
  });

  it("treats the search as workspace-scoped and case-insensitive", async () => {
    const { result } = renderHook(() => useGlobalSearch("SEMICONDUCTORS"), { wrapper });

    await waitFor(() => expect(result.current.total).toBeGreaterThan(0));
    const projectSection = result.current.sections.find((s) => s.id === "projects");
    expect(projectSection?.entries).toHaveLength(1);
    expect(projectSection?.entries[0]?.title).toBe("Semiconductors");
  });
});
