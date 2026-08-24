import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor, act } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { type ReactNode } from "react";
import { desktopApi } from "@/lib/desktop-api";
import {
  useGooseHealth,
  useGooseProposals,
  useStartShadowAnalysis,
  useCancelAnalysis,
  useAcceptProposal,
  useRejectProposal,
  useGooseShadowAnalysis,
} from "./useGooseAnalysis";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    goose: {
      checkGooseHealth: vi.fn(),
      startShadowAnalysis: vi.fn(),
      cancelAnalysis: vi.fn(),
      listProposals: vi.fn(),
      acceptProposal: vi.fn(),
      rejectProposal: vi.fn(),
    },
  },
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
}

describe("useGooseAnalysis hooks", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("useGooseHealth", () => {
    it("fetches goose service health status", async () => {
      vi.mocked(desktopApi.goose.checkGooseHealth).mockResolvedValue({
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      });

      const { result } = renderHook(() => useGooseHealth(), { wrapper: createWrapper() });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data?.binary_available).toBe(true);
      expect(result.current.data?.shadow_mode_enabled).toBe(true);
    });
  });

  describe("useGooseProposals", () => {
    it("fetches proposals list for workspace", async () => {
      const mockProposal = {
        id: "prop-1",
        workspace_id: "ws-1",
        run_id: "run-1",
        proposal_type: "evidence_candidate" as const,
        title: "Test proposal",
        summary: "Summary",
        payload: {},
        status: "pending" as const,
        created_at: "2026-08-24T16:00:00Z",
      };

      vi.mocked(desktopApi.goose.listProposals).mockResolvedValue([mockProposal]);

      const { result } = renderHook(() => useGooseProposals("ws-1", "pending"), {
        wrapper: createWrapper(),
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data).toHaveLength(1);
      expect(result.current.data?.[0]?.title).toBe("Test proposal");
      expect(desktopApi.goose.listProposals).toHaveBeenCalledWith("ws-1", "pending");
    });
  });

  describe("useAcceptProposal and useRejectProposal", () => {
    it("accepts a proposal", async () => {
      vi.mocked(desktopApi.goose.acceptProposal).mockResolvedValue({
        id: "prop-1",
        workspace_id: "ws-1",
        run_id: "run-1",
        proposal_type: "evidence_candidate" as const,
        title: "Test proposal",
        summary: "Summary",
        payload: {},
        status: "accepted" as const,
        created_at: "2026-08-24T16:00:00Z",
        reviewed_at: "2026-08-24T16:05:00Z",
        resulting_entity_id: "ev-1",
      });

      const { result } = renderHook(() => useAcceptProposal(), { wrapper: createWrapper() });

      act(() => {
        result.current.mutate("prop-1");
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(desktopApi.goose.acceptProposal).toHaveBeenCalledWith("prop-1");
    });

    it("rejects a proposal", async () => {
      vi.mocked(desktopApi.goose.rejectProposal).mockResolvedValue({
        id: "prop-1",
        workspace_id: "ws-1",
        run_id: "run-1",
        proposal_type: "evidence_candidate" as const,
        title: "Test proposal",
        summary: "Summary",
        payload: {},
        status: "rejected" as const,
        created_at: "2026-08-24T16:00:00Z",
        reviewed_at: "2026-08-24T16:06:00Z",
      });

      const { result } = renderHook(() => useRejectProposal(), { wrapper: createWrapper() });

      act(() => {
        result.current.mutate("prop-1");
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(desktopApi.goose.rejectProposal).toHaveBeenCalledWith("prop-1");
    });
  });

  describe("useStartShadowAnalysis", () => {
    it("executes shadow analysis mutation successfully", async () => {
      const mockResult = {
        run_id: "run-123",
        workspace_id: "ws-1",
        duration_ms: 1500,
        response: {
          summary: "Market outlook positive",
          confidence: 85,
          claims: [],
          evidence: [],
          contradictions: [],
          risks: [],
          unknowns: [],
          source_ids: [],
        },
      };

      vi.mocked(desktopApi.goose.startShadowAnalysis).mockResolvedValue(mockResult as any);

      const { result } = renderHook(() => useStartShadowAnalysis(), { wrapper: createWrapper() });

      act(() => {
        result.current.mutate({ workspace_id: "ws-1" });
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.data?.run_id).toBe("run-123");
      expect(desktopApi.goose.startShadowAnalysis).toHaveBeenCalledWith({
        workspace_id: "ws-1",
      });
    });
  });

  describe("useCancelAnalysis", () => {
    it("executes cancel mutation successfully", async () => {
      vi.mocked(desktopApi.goose.cancelAnalysis).mockResolvedValue(undefined);

      const { result } = renderHook(() => useCancelAnalysis(), { wrapper: createWrapper() });

      act(() => {
        result.current.mutate("run-123");
      });

      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(desktopApi.goose.cancelAnalysis).toHaveBeenCalledWith("run-123");
    });
  });

  describe("useGooseShadowAnalysis", () => {
    it("handles validation error when workspaceId is missing", () => {
      vi.mocked(desktopApi.goose.checkGooseHealth).mockResolvedValue({
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      });

      const { result } = renderHook(() => useGooseShadowAnalysis(""), { wrapper: createWrapper() });

      act(() => {
        result.current.startAnalysis();
      });

      expect(result.current.validationError).toBe(true);
      expect(desktopApi.goose.startShadowAnalysis).not.toHaveBeenCalled();
    });

    it("triggers analysis when workspaceId is provided", async () => {
      vi.mocked(desktopApi.goose.checkGooseHealth).mockResolvedValue({
        binary_available: true,
        shadow_mode_enabled: true,
        max_concurrent: 1,
      });

      const mockResult = {
        run_id: "run-777",
        workspace_id: "ws-valid",
        duration_ms: 1200,
        response: {
          summary: "Thesis analysis complete",
          confidence: 80,
          claims: [],
          evidence: [],
          contradictions: [],
          risks: [],
          unknowns: [],
          source_ids: [],
        },
      };

      vi.mocked(desktopApi.goose.startShadowAnalysis).mockResolvedValue(mockResult as any);

      const { result } = renderHook(() => useGooseShadowAnalysis("ws-valid"), { wrapper: createWrapper() });

      act(() => {
        result.current.startAnalysis({ thesis_id: "thesis-1" });
      });

      expect(result.current.validationError).toBe(false);
      await waitFor(() => expect(result.current.isSuccess).toBe(true));
      expect(result.current.result?.run_id).toBe("run-777");
    });
  });
});
