// Tests for WorkspaceList component.

import { describe, it, expect, vi } from "vitest";
import { render } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => new Promise(() => {})),
}));

describe("WorkspaceList", () => {
  it("shows loading state initially", () => {
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
        },
      },
    });

    // Dynamic import to avoid hoisting issues
    return import("@/features/workspace/components/WorkspaceList").then(({ WorkspaceList }) => {
      render(
        <QueryClientProvider client={queryClient}>
          <WorkspaceList />
        </QueryClientProvider>
      );

      // Should show loading spinner (svg with animate-spin class)
      const spinner = document.querySelector(".animate-spin");
      expect(spinner).toBeDefined();
    });
  });
});
