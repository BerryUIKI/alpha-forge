// Tests for WorkspaceList component.

import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { WorkspaceList } from "./WorkspaceList";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock desktopApi
const workspacesMock = vi.hoisted(() => ({
  listWorkspaces: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    workspace: workspacesMock,
  },
}));

function renderWorkspaceList(locale: Locale = "zh-CN") {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  const setLocale = vi.fn();
  const t = vi.fn((key: string) => {
    const messages: Record<string, Record<string, string>> = {
      "zh-CN": {
        loading: "加载中…",
        failedToLoadWorkspaces: "加载工作区失败",
        retry: "重试",
        noWorkspaces: "暂无工作区",
        noWorkspacesDescription: "创建你的第一个工作区，开始整理你的研究。",
        createWorkspace: "创建工作区",
        created: "创建于 {date}",
      },
      "en": {
        loading: "Loading…",
        failedToLoadWorkspaces: "Failed to load workspaces",
        retry: "Try Again",
        noWorkspaces: "No workspaces yet",
        noWorkspacesDescription: "Create your first workspace to start organizing your research.",
        createWorkspace: "Create Workspace",
        created: "Created {date}",
      },
    };
    return messages[locale]?.[key] || key;
  });

  return {
    ...render(
      <LocaleContext.Provider value={{ locale, setLocale, t }}>
        <QueryClientProvider client={queryClient}>
          <WorkspaceList />
        </QueryClientProvider>
      </LocaleContext.Provider>,
    ),
    t,
  };
}

describe("WorkspaceList", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("shows loading state initially", () => {
    workspacesMock.listWorkspaces.mockReturnValue(new Promise(() => {}));
    renderWorkspaceList();

    const spinner = document.querySelector(".animate-spin");
    expect(spinner).toBeDefined();
  });

  it("shows empty state in Chinese locale", async () => {
    workspacesMock.listWorkspaces.mockResolvedValue([]);

    renderWorkspaceList("zh-CN");

    await waitFor(() => {
      expect(screen.getByText("暂无工作区")).toBeInTheDocument();
    });
    expect(screen.getByText("创建你的第一个工作区，开始整理你的研究。")).toBeInTheDocument();
  });

  it("shows empty state in English locale", async () => {
    workspacesMock.listWorkspaces.mockResolvedValue([]);

    renderWorkspaceList("en");

    await waitFor(() => {
      expect(screen.getByText("No workspaces yet")).toBeInTheDocument();
    });
    expect(screen.getByText("Create your first workspace to start organizing your research.")).toBeInTheDocument();
  });

  it("shows error state with localized message", async () => {
    workspacesMock.listWorkspaces.mockRejectedValue(new Error("Network error"));

    renderWorkspaceList("zh-CN");

    await waitFor(() => {
      expect(screen.getByRole("alert")).toBeInTheDocument();
    });
  });

  it("renders workspace list with localized date", async () => {
    const mockWorkspaces = [
      {
        id: "1",
        name: "Test Workspace",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      },
    ];
    workspacesMock.listWorkspaces.mockResolvedValue(mockWorkspaces);

    renderWorkspaceList("zh-CN");

    await waitFor(() => {
      expect(screen.getByText("Test Workspace")).toBeInTheDocument();
    });
  });
});
