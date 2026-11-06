// Tests for the research workspace panel.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { ToastProvider } from "@/components/common/ToastProvider";
import { ResearchWorkspacePanel } from "./ResearchWorkspacePanel";
import type { ResearchDocument } from "@/lib/desktop-api";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

const timestamp = "2026-01-01T00:00:00Z";

function makeDocument(overrides: Partial<ResearchDocument> = {}): ResearchDocument {
  return {
    id: "doc-1",
    workspace_id: "ws-1",
    title: "Tesla notes",
    content: "Strong demand",
    created_at: timestamp,
    updated_at: timestamp,
    ...overrides,
  };
}

let documents: ResearchDocument[] = [];

function mockApi() {
  vi.mocked(invoke).mockImplementation(((command: string, args?: Record<string, unknown>) => {
    switch (command) {
      case "list_research_documents":
        return Promise.resolve(documents);
      case "create_research_document": {
        const document: ResearchDocument = {
          id: "doc-new",
          workspace_id: "ws-1",
          title: args?.title as string,
          content: (args?.content as string | null) ?? null,
          created_at: timestamp,
          updated_at: timestamp,
        };
        documents = [document, ...documents];
        return Promise.resolve(document);
      }
      case "update_research_document": {
        const id = args?.id as string;
        documents = documents.map((document) =>
          document.id === id
            ? {
                ...document,
                title: args?.title as string,
                content: (args?.content as string | null) ?? null,
              }
            : document,
        );
        return Promise.resolve(documents.find((document) => document.id === id));
      }
      case "delete_research_document": {
        documents = documents.filter((document) => document.id !== args?.id);
        return Promise.resolve(null);
      }
      default:
        return Promise.reject(new Error(`Unexpected command: ${command}`));
    }
  }) as unknown as typeof invoke);
}

function renderPanel() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <ResearchWorkspacePanel workspaceId="ws-1" />
      </ToastProvider>
    </QueryClientProvider>,
  );
}

describe("ResearchWorkspacePanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    documents = [makeDocument()];
  });

  afterEach(() => {
    cleanup();
  });

  it("opens a document in the editor", async () => {
    mockApi();
    renderPanel();

    fireEvent.click(await screen.findByText("Tesla notes"));

    expect(await screen.findByDisplayValue("Tesla notes")).toBeInTheDocument();
    expect(screen.getByDisplayValue("Strong demand")).toBeInTheDocument();
  });

  it("saves document edits", async () => {
    mockApi();
    renderPanel();

    fireEvent.click(await screen.findByText("Tesla notes"));
    fireEvent.change(await screen.findByLabelText("Title"), {
      target: { value: "Tesla Q4" },
    });
    fireEvent.change(screen.getByLabelText("Content"), {
      target: { value: "Updated findings" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save changes" }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("update_research_document", {
        id: "doc-1",
        title: "Tesla Q4",
        content: "Updated findings",
      });
    });
    expect(await screen.findByText("Document saved")).toBeInTheDocument();
  });

  it("creates a new document", async () => {
    mockApi();
    renderPanel();

    fireEvent.click(await screen.findByRole("button", { name: /new document/i }));
    fireEvent.change(await screen.findByLabelText("Title"), {
      target: { value: "Nvidia notes" },
    });
    fireEvent.change(screen.getByLabelText("Content"), {
      target: { value: "Data center demand" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Create document" }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("create_research_document", {
        workspaceId: "ws-1",
        title: "Nvidia notes",
        content: "Data center demand",
      });
    });
    expect(await screen.findByText("Document created")).toBeInTheDocument();
  });

  it("deletes a document after confirmation", async () => {
    mockApi();
    renderPanel();

    fireEvent.click(await screen.findByText("Tesla notes"));
    fireEvent.click(await screen.findByRole("button", { name: "Delete" }));
    fireEvent.click(await screen.findByRole("button", { name: "Confirm delete" }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("delete_research_document", {
        id: "doc-1",
      });
    });
    expect(await screen.findByText("No documents yet")).toBeInTheDocument();
  });

  it("shows the empty state without documents", async () => {
    documents = [];
    mockApi();
    renderPanel();

    expect(await screen.findByText("No documents yet")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /create document/i })).toBeInTheDocument();
  });
});
