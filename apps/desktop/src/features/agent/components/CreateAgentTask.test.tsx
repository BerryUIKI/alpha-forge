import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import type { ReactNode } from "react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { CreateAgentTask } from "./CreateAgentTask";

const agentMock = vi.hoisted(() => ({
  createAgentTask: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: { agent: agentMock },
}));

function renderComponent(props: { workspaceId: string; onSuccess?: (id: string) => void }) {
  const queryClient = new QueryClient({
    defaultOptions: { mutations: { retry: false }, queries: { retry: false } },
  });
  const wrapper = ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
  return render(<CreateAgentTask {...props} />, { wrapper });
}

describe("CreateAgentTask", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("displays raw string IPC error when mutation throws a raw string", async () => {
    agentMock.createAgentTask.mockRejectedValueOnce("Database constraint violation: duplicate title");
    const { container } = renderComponent({ workspaceId: "ws-1" });

    // Open the form by clicking the initial button
    fireEvent.click(screen.getByRole("button"));

    // Fill title
    const input = container.querySelector("#task-title") as HTMLInputElement;
    fireEvent.change(input, {
      target: { value: "My Task" },
    });

    // Submit form
    fireEvent.submit(input.closest("form")!);

    await waitFor(() => {
      expect(screen.getByText("Database constraint violation: duplicate title")).toBeInTheDocument();
    });
  });

  it("displays Error.message when mutation throws Error instance", async () => {
    agentMock.createAgentTask.mockRejectedValueOnce(new Error("Network connection dropped"));
    const { container } = renderComponent({ workspaceId: "ws-1" });

    fireEvent.click(screen.getByRole("button"));
    const input = container.querySelector("#task-title") as HTMLInputElement;
    fireEvent.change(input, {
      target: { value: "My Task" },
    });
    fireEvent.submit(input.closest("form")!);

    await waitFor(() => {
      expect(screen.getByText("Network connection dropped")).toBeInTheDocument();
    });
  });
});
