import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { renderHook, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { useAgentGlobalStatus, useAgentStatus } from "./useAgentStatus";

const agentMock = vi.hoisted(() => ({
  listAgentTasks: vi.fn(),
}));
const credentialsMock = vi.hoisted(() => ({
  hasOpenAiApiKey: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    agent: agentMock,
    credentials: credentialsMock,
  },
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: ReactNode }) {
    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
  };
}

describe("Agent credential status", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    agentMock.listAgentTasks.mockResolvedValue([]);
  });

  it("reports idle when the OpenAI key is configured", async () => {
    credentialsMock.hasOpenAiApiKey.mockResolvedValue(true);
    const { result } = renderHook(() => useAgentGlobalStatus(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => expect(result.current.data).toBe("idle"));
    expect(credentialsMock.hasOpenAiApiKey).toHaveBeenCalledOnce();
  });

  it("reports an unconfigured workspace status when the key is missing", async () => {
    credentialsMock.hasOpenAiApiKey.mockResolvedValue(false);
    const { result } = renderHook(() => useAgentStatus("workspace-1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => expect(result.current.status).toBe("unconfigured"));
    expect(credentialsMock.hasOpenAiApiKey).toHaveBeenCalledOnce();
  });
});
