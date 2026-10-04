import "@testing-library/jest-dom";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LocaleContext } from "@/lib/i18n/locale-context";
import { ActiveWorkspaceContext } from "@/features/workspace/hooks/useActiveWorkspace.context";
import { OnboardingWizardModal } from "./OnboardingWizardModal";
import type { Workspace } from "@/lib/desktop-api/workspace";

const demoDataMock = vi.hoisted(() => ({
  seedDemoData: vi.fn(),
}));

const workspaceMock = vi.hoisted(() => ({
  createWorkspace: vi.fn(),
}));

vi.mock("@/lib/demoData", () => ({
  seedDemoData: demoDataMock.seedDemoData,
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: workspaceMock,
  },
}));

const messages: Record<string, string> = {
  onboardingTitle: "Welcome to AlphaForge Investment Research OS",
  onboardingDesc: "No existing research workspaces were detected.",
  onboardingLoadDemoOption: "Load Demo Dataset (Recommended)",
  onboardingLoadDemoDesc: "Preloads AI & semiconductor theses...",
  onboardingCustomOption: "Start from Scratch",
  onboardingCustomDesc: "Create an empty research workspace...",
  loadDemoDataBtn: "Load Full Demo Portfolio",
  loadingDemoDataBtn: "Seeding Demo Data…",
  onboardingStartFreshBtn: "Create & Enter Workspace",
  demoDataLoadedSuccess: "Demo data loaded successfully! Switched to workspace \"{name}\".",
  workspaceName: "Workspace Name",
  onboardingWorkspaceNamePlaceholder: "Enter workspace name",
  workspaceNameRequired: "Workspace name is required",
};

interface RenderOptions {
  workspaces?: Workspace[];
  isLoading?: boolean;
  setActiveWorkspace?: (id: string) => void;
}

function renderModal({
  workspaces = [],
  isLoading = false,
  setActiveWorkspace = vi.fn(),
}: RenderOptions = {}) {
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
      <ActiveWorkspaceContext.Provider
        value={{
          workspaceId: workspaces[0]?.id ?? "",
          workspace: workspaces[0] ?? null,
          workspaces,
          isLoading,
          setActiveWorkspace,
        }}
      >
        <QueryClientProvider client={queryClient}>
          <OnboardingWizardModal />
        </QueryClientProvider>
      </ActiveWorkspaceContext.Provider>
    </LocaleContext.Provider>,
  );
}

describe("OnboardingWizardModal", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("does not render when workspaces are still loading", () => {
    renderModal({ isLoading: true, workspaces: [] });
    expect(screen.queryByTestId("onboarding-wizard-modal")).not.toBeInTheDocument();
  });

  it("does not render when user already has at least one workspace", () => {
    const existingWs: Workspace = {
      id: "ws-1",
      name: "Existing Tech Alpha",
      createdAt: "2026-10-01T00:00:00Z",
      updatedAt: "2026-10-01T00:00:00Z",
    };
    renderModal({ isLoading: false, workspaces: [existingWs] });
    expect(screen.queryByTestId("onboarding-wizard-modal")).not.toBeInTheDocument();
  });

  it("renders when workspaces array is empty and loading is false", () => {
    renderModal({ isLoading: false, workspaces: [] });
    expect(screen.getByTestId("onboarding-wizard-modal")).toBeInTheDocument();
    expect(screen.getByText("Welcome to AlphaForge Investment Research OS")).toBeInTheDocument();
    expect(screen.getByText("Load Full Demo Portfolio")).toBeInTheDocument();
  });

  it("executes seedDemoData when demo option is submitted", async () => {
    const setActiveWorkspace = vi.fn();
    demoDataMock.seedDemoData.mockResolvedValueOnce({
      workspaceId: "ws-demo",
      workspaceName: "AI & Semiconductor Alpha",
    });

    renderModal({ isLoading: false, workspaces: [], setActiveWorkspace });

    const submitBtn = screen.getByRole("button", { name: "Load Full Demo Portfolio" });
    fireEvent.click(submitBtn);

    await waitFor(() => expect(demoDataMock.seedDemoData).toHaveBeenCalledOnce());
    await waitFor(() => expect(setActiveWorkspace).toHaveBeenCalledWith("ws-demo"));
    await waitFor(() =>
      expect(
        screen.getByText('Demo data loaded successfully! Switched to workspace "AI & Semiconductor Alpha".'),
      ).toBeInTheDocument(),
    );
  });

  it("allows switching to custom workspace mode and creating a new workspace", async () => {
    const setActiveWorkspace = vi.fn();
    workspaceMock.createWorkspace.mockResolvedValueOnce({
      id: "ws-custom",
      name: "Global Macro 2026",
      createdAt: 2000,
      updatedAt: 2000,
    });

    renderModal({ isLoading: false, workspaces: [], setActiveWorkspace });

    // Click "Start from Scratch"
    fireEvent.click(screen.getByText("Start from Scratch"));

    // Verify input appears
    const input = screen.getByPlaceholderText("Enter workspace name");
    expect(input).toBeInTheDocument();

    // Verify button text changes
    const submitBtn = screen.getByRole("button", { name: "Create & Enter Workspace" });
    expect(submitBtn).toBeInTheDocument();

    // Submit with text
    fireEvent.change(input, { target: { value: "Global Macro 2026" } });
    fireEvent.click(submitBtn);

    await waitFor(() =>
      expect(workspaceMock.createWorkspace).toHaveBeenCalledWith("Global Macro 2026"),
    );
    await waitFor(() => expect(setActiveWorkspace).toHaveBeenCalledWith("ws-custom"));
  });
});
