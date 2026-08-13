import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("OpenAI credentials API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("saves the OpenAI API key through the provider-specific command", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    const { saveOpenAiApiKey } = await import("./credentials");

    await saveOpenAiApiKey("sk-test");

    expect(invoke).toHaveBeenCalledWith("save_openai_api_key", {
      value: "sk-test",
    });
  });

  it("checks status without accepting a caller-controlled credential name", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(true);
    const { hasOpenAiApiKey } = await import("./credentials");

    await expect(hasOpenAiApiKey()).resolves.toBe(true);
    expect(invoke).toHaveBeenCalledWith("has_openai_api_key");
  });

  it("deletes the OpenAI API key through the provider-specific command", async () => {
    vi.mocked(invoke).mockResolvedValueOnce(undefined);
    const { deleteOpenAiApiKey } = await import("./credentials");

    await deleteOpenAiApiKey();

    expect(invoke).toHaveBeenCalledWith("delete_openai_api_key");
  });
});
