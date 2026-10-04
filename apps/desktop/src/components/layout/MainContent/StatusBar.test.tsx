import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { StatusBar } from "./StatusBar";
import { desktopApi } from "@/lib/desktop-api";

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    system: {
      checkDatabaseHealth: vi.fn(),
    },
  },
}));

vi.mock("@/hooks/useNetworkStatus", () => ({
  useNetworkStatus: () => ({ isOnline: true }),
}));

vi.mock("@/hooks/useAgentStatus", () => ({
  useAgentGlobalStatus: () => ({ data: "idle" }),
}));

describe("StatusBar", () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    vi.clearAllMocks();
    queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
      },
    });
  });

  it("renders live operational status when database is healthy", async () => {
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockResolvedValue("healthy");

    render(
      <QueryClientProvider client={queryClient}>
        <StatusBar />
      </QueryClientProvider>,
    );

    expect(
      await screen.findByText(/All systems operational|系统运行正常/i),
    ).toBeInTheDocument();
    expect(
      await screen.findByText(/Database ready|数据库已就绪/i),
    ).toBeInTheDocument();
    expect(screen.getByText(/v0\.1\.0/i)).toBeInTheDocument();
  });

  it("renders degraded status when database health check fails", async () => {
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockRejectedValue(
      new Error("connection failed"),
    );

    render(
      <QueryClientProvider client={queryClient}>
        <StatusBar />
      </QueryClientProvider>,
    );

    expect(
      await screen.findByText(/Database degraded|数据库状态异常/i),
    ).toBeInTheDocument();
    expect(
      await screen.findByText(/Database error|数据库异常/i),
    ).toBeInTheDocument();
  });
});
