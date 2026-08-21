// Tests for workspace desktop API.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("Workspace API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("createWorkspace calls invoke with correct parameters", async () => {
    const mockWorkspace = {
      id: "test-uuid",
      name: "Test Workspace",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockWorkspace);

    const { createWorkspace } = await import("@/lib/desktop-api/workspace");
    const result = await createWorkspace("Test Workspace");

    expect(invoke).toHaveBeenCalledWith("create_workspace", { name: "Test Workspace" });
    expect(result).toEqual(mockWorkspace);
  });

  it("listWorkspaces calls invoke with correct command", async () => {
    const mockWorkspaces = [
      {
        id: "workspace-1",
        name: "Workspace 1",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValueOnce(mockWorkspaces);

    const { listWorkspaces } = await import("@/lib/desktop-api/workspace");
    const result = await listWorkspaces();

    expect(invoke).toHaveBeenCalledWith("list_workspaces");
    expect(result).toEqual(mockWorkspaces);
  });

  it("getWorkspace calls invoke with correct parameters", async () => {
    const mockWorkspace = {
      id: "test-uuid",
      name: "Test Workspace",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockWorkspace);

    const { getWorkspace } = await import("@/lib/desktop-api/workspace");
    const result = await getWorkspace("test-uuid");

    expect(invoke).toHaveBeenCalledWith("get_workspace", { id: "test-uuid" });
    expect(result).toEqual(mockWorkspace);
  });

  it("updateWorkspace calls invoke with correct parameters", async () => {
    const mockWorkspace = {
      id: "test-uuid",
      name: "Updated Name",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValueOnce(mockWorkspace);

    const { updateWorkspace } = await import("@/lib/desktop-api/workspace");
    const result = await updateWorkspace("test-uuid", "Updated Name");

    expect(invoke).toHaveBeenCalledWith("update_workspace", {
      id: "test-uuid",
      name: "Updated Name",
    });
    expect(result).toEqual(mockWorkspace);
  });

  it("deleteWorkspace calls invoke with correct parameters", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(undefined);

    const { deleteWorkspace } = await import("@/lib/desktop-api/workspace");
    await deleteWorkspace("test-uuid");

    expect(invoke).toHaveBeenCalledWith("delete_workspace", { id: "test-uuid" });
  });

  it("handles API errors correctly", async () => {
    const mockError = {
      code: "VALIDATION",
      message: "Workspace name cannot be empty",
      recoverable: true,
    };

    vi.mocked(invoke).mockRejectedValueOnce(mockError);

    const { createWorkspace } = await import("@/lib/desktop-api/workspace");

    await expect(createWorkspace("")).rejects.toEqual(mockError);
  });

  it("rejects malformed workspace response shape via Zod", async () => {
    const malformed = {
      id: "test-uuid",
      // missing name, createdAt, updatedAt
    };

    vi.mocked(invoke).mockResolvedValueOnce(malformed);

    const { getWorkspace } = await import("@/lib/desktop-api/workspace");
    await expect(getWorkspace("test-uuid")).rejects.toThrow();
  });
});