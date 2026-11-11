import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { StatusBar } from "./StatusBar";
import { desktopApi } from "@/lib/desktop-api";

const networkMock = vi.hoisted(() => ({ isOnline: true }));
const agentMock = vi.hoisted(() => ({
  status: "idle" as string,
  isLoading: false,
  isError: false,
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    system: {
      checkDatabaseHealth: vi.fn(),
    },
  },
}));

vi.mock("@/hooks/useNetworkStatus", () => ({
  useNetworkStatus: () => ({ isOnline: networkMock.isOnline }),
}));

vi.mock("@/hooks/useAgentStatus", () => ({
  useAgentGlobalStatusDisplay: () => agentMock,
}));

vi.mock("@/lib/i18n/useLocale", async () => {
  const { translate } = await import("@/lib/i18n/locale");
  return {
    useLocale: () => ({
      t: (key: string) => translate("en", key as never),
    }),
  };
});

describe("StatusBar", () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    vi.clearAllMocks();
    networkMock.isOnline = true;
    agentMock.status = "idle";
    agentMock.isLoading = false;
    agentMock.isError = false;
    queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
      },
    });
  });

  function renderBar() {
    return render(
      <QueryClientProvider client={queryClient}>
        <StatusBar />
      </QueryClientProvider>,
    );
  }

  it("reports backend health without claiming whole-application health", async () => {
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockResolvedValue("healthy");

    renderBar();

    expect(await screen.findByText(/Backend operational/i)).toBeInTheDocument();
    expect(await screen.findByText(/Database ready/i)).toBeInTheDocument();
    expect(screen.queryByText(/All systems operational/i)).not.toBeInTheDocument();
    expect(screen.getByText(/v0\.1\.0/i)).toBeInTheDocument();
  });

  it("shows an unknown state while the health probe is loading", () => {
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockReturnValue(new Promise(() => {}));

    renderBar();

    expect(screen.getByText(/Checking system health/i)).toBeInTheDocument();
    expect(screen.getByText(/Checking database/i)).toBeInTheDocument();
    expect(screen.queryByText(/Backend operational/i)).not.toBeInTheDocument();
  });

  it("reports degraded state when database health fails", async () => {
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockRejectedValue(
      new Error("connection failed"),
    );

    renderBar();

    expect(await screen.findByText(/Database degraded/i)).toBeInTheDocument();
    expect(await screen.findByText(/Database error/i)).toBeInTheDocument();
  });

  it("labels connectivity as connectivity, not application health", async () => {
    networkMock.isOnline = false;
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockResolvedValue("healthy");

    renderBar();

    expect((await screen.findAllByText(/Offline/i)).length).toBeGreaterThan(0);
    expect(screen.queryByText(/Backend operational/i)).not.toBeInTheDocument();
  });

  it("does not render a healthy Agent state while the probe is checking", () => {
    agentMock.status = "checking";
    agentMock.isLoading = true;
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockReturnValue(new Promise(() => {}));

    renderBar();

    expect(screen.getAllByText(/Checking/i).length).toBeGreaterThan(0);
    expect(screen.queryByText(/^Idle$/i)).not.toBeInTheDocument();
  });

  it("shows an unavailable Agent state on probe failure", () => {
    agentMock.status = "unavailable";
    agentMock.isError = true;
    vi.mocked(desktopApi.system.checkDatabaseHealth).mockReturnValue(new Promise(() => {}));

    renderBar();

    expect(screen.getByText(/Unavailable/i)).toBeInTheDocument();
  });
});
