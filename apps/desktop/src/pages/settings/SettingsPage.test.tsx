import "@testing-library/jest-dom";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { SettingsPage } from "./SettingsPage";

const credentialsMock = vi.hoisted(() => ({
  hasOpenAiApiKey: vi.fn(),
  saveOpenAiApiKey: vi.fn(),
}));
const systemMock = vi.hoisted(() => ({
  exportLocalBackup: vi.fn(),
  checkForUpdate: vi.fn(),
  checkDatabaseHealth: vi.fn(),
}));
const pluginsMock = vi.hoisted(() => ({
  listPlugins: vi.fn(),
  setPluginEnabled: vi.fn(),
}));
const settingsMock = vi.hoisted(() => ({
  getSetting: vi.fn(),
  setSetting: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    credentials: credentialsMock,
    plugins: pluginsMock,
    settings: settingsMock,
    system: systemMock,
  },
}));
vi.mock("@tauri-apps/plugin-opener", () => ({ openUrl: vi.fn() }));

const messages: Record<string, string> = {
  save: "Save",
  apiKeyPlaceholder: "A key is securely stored; enter a new value to replace it",
  agentConfigSaved: "Agent configuration saved",
};

function renderSettings() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false }, mutations: { retry: false } },
  });
  return render(
    <LocaleContext.Provider
      value={{
        locale: "en",
        setLocale: vi.fn(),
        t: (key) => messages[key] ?? key,
      }}
    >
      <QueryClientProvider client={queryClient}>
        <SettingsPage />
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("Settings OpenAI credentials", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    credentialsMock.hasOpenAiApiKey.mockResolvedValue(true);
    credentialsMock.saveOpenAiApiKey.mockResolvedValue(undefined);
    pluginsMock.listPlugins.mockResolvedValue([]);
    pluginsMock.setPluginEnabled.mockResolvedValue(undefined);
    settingsMock.getSetting.mockResolvedValue(null);
    settingsMock.setSetting.mockResolvedValue(undefined);
  });

  it("shows stored status without placing a secret mask in the editable value", async () => {
    renderSettings();
    const input = screen.getByLabelText("API Key");

    await waitFor(() =>
      expect(input).toHaveAttribute(
        "placeholder",
        "A key is securely stored; enter a new value to replace it",
      ),
    );
    expect(input).toHaveValue("");
    expect(credentialsMock.hasOpenAiApiKey).toHaveBeenCalledOnce();
  });

  it("saves only a newly entered OpenAI API key and then clears the field", async () => {
    renderSettings();
    const input = screen.getByLabelText("API Key");
    fireEvent.change(input, { target: { value: "sk-new" } });
    fireEvent.click(screen.getByRole("button", { name: "Save" }));

    await waitFor(() => expect(credentialsMock.saveOpenAiApiKey).toHaveBeenCalledWith("sk-new"));
    await waitFor(() => expect(input).toHaveValue(""));
  });

  it("shows a status error when the keychain cannot be checked", async () => {
    credentialsMock.hasOpenAiApiKey.mockRejectedValueOnce(new Error("keychain unavailable"));

    renderSettings();

    await waitFor(() => expect(screen.getByText("agentConfigError")).toBeInTheDocument());
  });

  it("mounts the internal plugin settings surface", async () => {
    renderSettings();

    expect(screen.getByText("internalPlugins")).toBeInTheDocument();
    await waitFor(() => expect(pluginsMock.listPlugins).toHaveBeenCalledOnce());
  });
});
