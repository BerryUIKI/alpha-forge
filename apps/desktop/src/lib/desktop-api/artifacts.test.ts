// Tests for artifacts desktop API.

import { beforeEach, describe, it, expect, vi } from "vitest";
import {
  createArtifact,
  getArtifact,
  listArtifacts,
  listTaskArtifacts,
  startArtifactGeneration,
  completeArtifactGeneration,
  failArtifactGeneration,
  startViewingArtifact,
  closeArtifact,
  deleteArtifact,
  listOpenArtifacts,
} from "./artifacts";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";

const mockInvoke = vi.mocked(invoke);

describe("artifacts API", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  describe("createArtifact", () => {
    it("should create an artifact with all fields", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: "task-1",
        artifactType: "comparison_table",
        status: "pending",
        input: { test: "data" },
        output: null,
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await createArtifact({
        workspaceId: "workspace-1",
        taskId: "task-1",
        artifactType: "comparison_table",
        input: { test: "data" },
      });

      expect(mockInvoke).toHaveBeenCalledWith("create_artifact", {
        workspaceId: "workspace-1",
        taskId: "task-1",
        artifactType: "comparison_table",
        input: { test: "data" },
      });
      expect(result).toEqual(mockArtifact);
    });

    it("should create an artifact without task_id", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "timeline",
        status: "pending",
        input: {},
        output: null,
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await createArtifact({
        workspaceId: "workspace-1",
        artifactType: "timeline",
        input: {},
      });

      expect(mockInvoke).toHaveBeenCalledWith("create_artifact", {
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "timeline",
        input: {},
      });
      expect(result).toEqual(mockArtifact);
    });
  });

  describe("getArtifact", () => {
    it("should get an artifact by ID", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "completed",
        input: {},
        output: { result: "success" },
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await getArtifact("test-id");

      expect(mockInvoke).toHaveBeenCalledWith("get_artifact", { id: "test-id" });
      expect(result).toEqual(mockArtifact);
    });

    it("should return null for nonexistent artifact", async () => {
      mockInvoke.mockResolvedValueOnce(null);

      const result = await getArtifact("nonexistent");

      expect(result).toBeNull();
    });
  });

  describe("listArtifacts", () => {
    it("should list artifacts for a workspace", async () => {
      const mockArtifacts = [
        {
          id: "artifact-1",
          workspaceId: "workspace-1",
          taskId: null,
          artifactType: "comparison_table",
          status: "completed",
          input: {},
          output: null,
          error: null,
          createdAt: "2024-01-01T00:00:00Z",
          updatedAt: "2024-01-01T00:00:00Z",
        },
        {
          id: "artifact-2",
          workspaceId: "workspace-1",
          taskId: null,
          artifactType: "timeline",
          status: "pending",
          input: {},
          output: null,
          error: null,
          createdAt: "2024-01-02T00:00:00Z",
          updatedAt: "2024-01-02T00:00:00Z",
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockArtifacts);

      const result = await listArtifacts("workspace-1");

      expect(mockInvoke).toHaveBeenCalledWith("list_artifacts", {
        workspaceId: "workspace-1",
      });
      expect(result).toEqual(mockArtifacts);
    });
  });

  describe("listTaskArtifacts", () => {
    it("should list artifacts for a task", async () => {
      const mockArtifacts = [
        {
          id: "artifact-1",
          workspaceId: "workspace-1",
          taskId: "task-1",
          artifactType: "comparison_table",
          status: "completed",
          input: {},
          output: null,
          error: null,
          createdAt: "2024-01-01T00:00:00Z",
          updatedAt: "2024-01-01T00:00:00Z",
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockArtifacts);

      const result = await listTaskArtifacts("task-1");

      expect(mockInvoke).toHaveBeenCalledWith("list_task_artifacts", {
        taskId: "task-1",
      });
      expect(result).toEqual(mockArtifacts);
    });
  });

  describe("startArtifactGeneration", () => {
    it("should start artifact generation", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "generating",
        input: {},
        output: null,
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await startArtifactGeneration("test-id");

      expect(mockInvoke).toHaveBeenCalledWith("start_artifact_generation", {
        id: "test-id",
      });
      expect(result.status).toBe("generating");
    });
  });

  describe("completeArtifactGeneration", () => {
    it("should complete artifact generation with output", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "completed",
        input: {},
        output: { data: "result" },
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await completeArtifactGeneration("test-id", {
        data: "result",
      });

      expect(mockInvoke).toHaveBeenCalledWith("complete_artifact_generation", {
        id: "test-id",
        output: { data: "result" },
      });
      expect(result.status).toBe("completed");
    });
  });

  describe("failArtifactGeneration", () => {
    it("should fail artifact generation with error", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "failed",
        input: {},
        output: null,
        error: "Test error",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await failArtifactGeneration("test-id", "Test error");

      expect(mockInvoke).toHaveBeenCalledWith("fail_artifact_generation", {
        id: "test-id",
        error: "Test error",
      });
      expect(result.status).toBe("failed");
      expect(result.error).toBe("Test error");
    });
  });

  describe("startViewingArtifact", () => {
    it("should open artifact for viewing", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "viewing",
        input: {},
        output: {},
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await startViewingArtifact("test-id");

      expect(mockInvoke).toHaveBeenCalledWith("start_viewing_artifact", {
        id: "test-id",
      });
      expect(result.status).toBe("viewing");
    });
  });

  describe("closeArtifact", () => {
    it("should close artifact", async () => {
      const mockArtifact = {
        id: "test-id",
        workspaceId: "workspace-1",
        taskId: null,
        artifactType: "comparison_table",
        status: "closed",
        input: {},
        output: {},
        error: null,
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      };

      mockInvoke.mockResolvedValueOnce(mockArtifact);

      const result = await closeArtifact("test-id");

      expect(mockInvoke).toHaveBeenCalledWith("close_artifact", { id: "test-id" });
      expect(result.status).toBe("closed");
    });
  });

  describe("deleteArtifact", () => {
    it("should delete artifact", async () => {
      mockInvoke.mockResolvedValueOnce(undefined);

      await deleteArtifact("test-id");

      expect(mockInvoke).toHaveBeenCalledWith("delete_artifact", { id: "test-id" });
    });
  });

  describe("listOpenArtifacts", () => {
    it("should list open artifact IDs", async () => {
      const mockIds = ["artifact-1", "artifact-2"];

      mockInvoke.mockResolvedValueOnce(mockIds);

      const result = await listOpenArtifacts();

      expect(mockInvoke).toHaveBeenCalledWith("list_open_artifacts");
      expect(result).toEqual(mockIds);
    });
  });

  describe("Zod validation", () => {
    it("rejects malformed artifact responses at runtime", async () => {
      mockInvoke.mockResolvedValueOnce({
        id: "test-id",
        workspace_id: "legacy_snake_case", // Missing camelCase workspaceId
        status: "invalid_status",
      });

      await expect(getArtifact("test-id")).rejects.toThrow();
    });
  });
});
