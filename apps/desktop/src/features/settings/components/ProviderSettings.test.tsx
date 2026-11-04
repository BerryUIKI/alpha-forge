// Tests for ProviderSettings component.

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "@testing-library/jest-dom";
import { invoke } from "@tauri-apps/api/core";

import { ToastProvider } from "@/components/common/ToastProvider";
import { ProviderSettings } from "./ProviderSettings";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

type CommandHandler = (args?: Record<string, unknown>) => unknown;

function mockCommands(handlers: Record<string, CommandHandler>) {
  vi.mocked(invoke).mockImplementation(((command: string, args?: Record<string, unknown>) => {
    const handler = handlers[command];
    if (!handler) {
      return Promise.reject(new Error(`Unexpected command: ${command}`));
    }
    return Promise.resolve(handler(args));
  }) as unknown as typeof invoke);
}

function renderSettings() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });

  return render(
    <QueryClientProvider client={queryClient}>
      <ToastProvider>
        <ProviderSettings />
      </ToastProvider>
    </QueryClientProvider>,
  );
}

const unconfigured = { provider: "OpenAI", configured: false, model: "gpt-4o-mini" };

describe("ProviderSettings", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    cleanup();
  });

  it("shows an unconfigured provider", async () => {
    mockCommands({ get_provider_status: () => unconfigured });

    renderSettings();

    expect(await screen.findByText("Not configured")).toBeInTheDocument();
    expect(screen.getByText("Model: gpt-4o-mini")).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Remove key" })).not.toBeInTheDocument();
  });

  it("saves an API key and clears the input", async () => {
    let configured = false;
    mockCommands({
      get_provider_status: () => ({ ...unconfigured, configured }),
      set_provider_api_key: () => {
        configured = true;
        return null;
      },
    });

    renderSettings();

    const input = await screen.findByLabelText("OpenAI API key");
    fireEvent.change(input, { target: { value: "  sk-test  " } });
    fireEvent.click(screen.getByRole("button", { name: "Save key" }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("set_provider_api_key", {
        apiKey: "sk-test",
      });
    });
    expect(await screen.findByText("API key saved")).toBeInTheDocument();
    await waitFor(() => expect(input).toHaveValue(""));
    expect(await screen.findByText("Configured")).toBeInTheDocument();
  });

  it("removes a stored key", async () => {
    let configured = true;
    mockCommands({
      get_provider_status: () => ({ ...unconfigured, configured }),
      clear_provider_api_key: () => {
        configured = false;
        return null;
      },
    });

    renderSettings();

    fireEvent.click(await screen.findByRole("button", { name: "Remove key" }));

    await waitFor(() => {
      expect(vi.mocked(invoke)).toHaveBeenCalledWith("clear_provider_api_key");
    });
    expect(await screen.findByText("API key removed")).toBeInTheDocument();
    expect(await screen.findByText("Not configured")).toBeInTheDocument();
  });

  it("reports a failed save", async () => {
    mockCommands({
      get_provider_status: () => unconfigured,
      set_provider_api_key: () =>
        Promise.reject({
          code: "VALIDATION",
          message: "key rejected",
          recoverable: true,
        }),
    });

    renderSettings();

    fireEvent.change(await screen.findByLabelText("OpenAI API key"), {
      target: { value: "sk-bad" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save key" }));

    expect(await screen.findByText("Failed to save API key")).toBeInTheDocument();
    expect(screen.getByText("key rejected")).toBeInTheDocument();
  });
});
