// Tests for ResearchPage component.

import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MemoryRouter, useLocation, useNavigationType } from "react-router-dom";
import { useEffect, useRef } from "react";
import { ResearchPage } from "./ResearchPage";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Mock desktopApi
const researchMock = vi.hoisted(() => ({
  listResearchProjects: vi.fn(),
  listResearchDocuments: vi.fn(),
  listResearchReports: vi.fn(),
  listResearchNotes: vi.fn(),
  listResearchSources: vi.fn(),
  createResearchProject: vi.fn(),
  createResearchDocument: vi.fn(),
  createResearchReport: vi.fn(),
  createResearchNote: vi.fn(),
  createResearchSource: vi.fn(),
  importResearchPdf: vi.fn(),
  importResearchWebPage: vi.fn(),
  searchResearchDocument: vi.fn(),
  semanticSearchResearchDocument: vi.fn(),
  archiveResearchProject: vi.fn(),
  completeResearchProject: vi.fn(),
  deleteResearchProject: vi.fn(),
}));

const workspaceMock = vi.hoisted(() => ({
  listWorkspaces: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    research: researchMock,
    workspace: workspaceMock,
  },
}));

type RouterState = {
  search: string;
  historyAction: string;
  navigationCount: number;
};

function NavigationProbe({ state }: { state: RouterState }) {
  const location = useLocation();
  const navigationType = useNavigationType();
  const firstRender = useRef(true);

  useEffect(() => {
    state.search = location.search;
    state.historyAction = navigationType;
    if (firstRender.current) {
      firstRender.current = false;
    } else {
      state.navigationCount += 1;
    }
  }, [location.key, location.search, navigationType, state]);

  return null;
}

function renderResearchPage(locale: Locale = "zh-CN", initialEntry = "/research") {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  const routerState: RouterState = {
    search: new URL(initialEntry, "http://localhost").search,
    historyAction: "POP",
    navigationCount: 0,
  };

  const setLocale = vi.fn();
  const t = vi.fn((key: string) => {
    const messages: Record<string, Record<string, string>> = {
      "zh-CN": {
        researchTitle: "研究",
        researchDescription: "捕获项目、来源出处和文档注释。",
        workspace: "工作区",
        selectWorkspace: "选择工作区",
        projects: "项目",
        projectTitle: "项目标题",
        create: "创建",
        documents: "文档",
        documentTitle: "文档标题",
        add: "添加",
        reports: "报告",
        notes: "笔记",
        sources: "来源",
        searchDocument: "搜索此文档",
        saveError: "无法保存研究项目。请检查必填字段后重试。",
      },
      en: {
        researchTitle: "Research",
        researchDescription: "Capture projects, source provenance, and document annotations.",
        workspace: "Workspace",
        selectWorkspace: "Select a workspace",
        projects: "Projects",
        projectTitle: "Project title",
        create: "Create",
        documents: "Documents",
        documentTitle: "Document title",
        add: "Add",
        reports: "Reports",
        notes: "Notes",
        sources: "Sources",
        searchDocument: "Search this document",
        saveError: "Unable to save the research item. Check the required fields and try again.",
      },
    };
    return messages[locale]?.[key] || key;
  });

  const view = render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <QueryClientProvider client={queryClient}>
        <MemoryRouter initialEntries={[initialEntry]}>
          <NavigationProbe state={routerState} />
          <ResearchPage />
        </MemoryRouter>
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );

  return { ...view, routerState };
}

describe("ResearchPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    workspaceMock.listWorkspaces.mockResolvedValue([]);
    researchMock.listResearchProjects.mockResolvedValue([]);
    researchMock.listResearchDocuments.mockResolvedValue([]);
    researchMock.listResearchReports.mockResolvedValue([]);
    researchMock.listResearchNotes.mockResolvedValue([]);
    researchMock.listResearchSources.mockResolvedValue([]);
    researchMock.deleteResearchProject.mockResolvedValue(undefined);
  });

  it("renders research page title in Chinese locale", () => {
    renderResearchPage("zh-CN");

    expect(screen.getByText("研究")).toBeInTheDocument();
    expect(screen.getByText("捕获项目、来源出处和文档注释。")).toBeInTheDocument();
  });

  it("renders research page title in English locale", () => {
    renderResearchPage("en");

    expect(screen.getByText("Research")).toBeInTheDocument();
    expect(
      screen.getByText("Capture projects, source provenance, and document annotations."),
    ).toBeInTheDocument();
  });

  it("renders workspace selector", async () => {
    const mockWorkspaces = [
      {
        id: "1",
        name: "Test Workspace",
        createdAt: "2024-01-01T00:00:00Z",
        updatedAt: "2024-01-01T00:00:00Z",
      },
    ];
    workspaceMock.listWorkspaces.mockResolvedValue(mockWorkspaces);

    renderResearchPage("zh-CN");

    await waitFor(() => {
      expect(screen.getByText("工作区")).toBeInTheDocument();
    });
  });

  it("shows workspace select dropdown", async () => {
    renderResearchPage("zh-CN");

    await waitFor(() => {
      expect(screen.getByText("选择工作区")).toBeInTheDocument();
    });
  });

  it("restores a valid workspace and project from a deep link", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockResolvedValue([
      { id: "p1", workspace_id: "w1", title: "Project 1", status: "active" },
    ]);

    const { routerState } = renderResearchPage("en", "/research?workspace=w1&project=p1&tab=notes");

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Project 1" })).toBeInTheDocument();
    });
    await waitFor(() => {
      expect(researchMock.listResearchDocuments).toHaveBeenCalledWith("p1");
      expect(researchMock.listResearchReports).toHaveBeenCalledWith("p1");
    });
    expect(screen.getByLabelText("Workspace")).toHaveValue("w1");
    expect(routerState.search).toBe("?workspace=w1&project=p1&tab=notes");
    expect(routerState.navigationCount).toBe(0);
  });

  it("restores the URL context after a remount", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockResolvedValue([
      { id: "p1", workspace_id: "w1", title: "Project 1", status: "active" },
    ]);

    const first = renderResearchPage("en", "/research?workspace=w1&project=p1");
    await waitFor(() => expect(screen.getByLabelText("Workspace")).toHaveValue("w1"));
    first.unmount();

    renderResearchPage("en", "/research?workspace=w1&project=p1");
    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Project 1" })).toBeInTheDocument();
    });
    expect(screen.getByLabelText("Workspace")).toHaveValue("w1");
  });

  it("clears an invalid workspace and project once with replace", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    const { routerState } = renderResearchPage(
      "en",
      "/research?workspace=missing&project=p1&tab=notes",
    );

    await waitFor(() => expect(routerState.search).toBe("?tab=notes"));
    expect(routerState.historyAction).toBe("REPLACE");
    expect(routerState.navigationCount).toBe(1);
  });

  it("clears an invalid project once with replace after project loading succeeds", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockResolvedValue([
      { id: "p1", workspace_id: "w1", title: "Project 1", status: "active" },
    ]);
    const { routerState } = renderResearchPage(
      "en",
      "/research?workspace=w1&project=missing&tab=notes",
    );

    await waitFor(() => expect(routerState.search).toBe("?workspace=w1&tab=notes"));
    expect(routerState.historyAction).toBe("REPLACE");
    expect(routerState.navigationCount).toBe(1);
  });

  it("does not clean a deep link while the workspace query is loading or errors", async () => {
    workspaceMock.listWorkspaces.mockReturnValue(new Promise(() => undefined));
    const loading = renderResearchPage("en", "/research?workspace=missing&tab=notes");
    await waitFor(() => expect(workspaceMock.listWorkspaces).toHaveBeenCalled());
    expect(loading.routerState.search).toBe("?workspace=missing&tab=notes");
    loading.unmount();

    workspaceMock.listWorkspaces.mockRejectedValue(new Error("offline"));
    const failed = renderResearchPage("en", "/research?workspace=missing&tab=notes");
    await waitFor(() => expect(workspaceMock.listWorkspaces).toHaveBeenCalledTimes(2));
    expect(failed.routerState.search).toBe("?workspace=missing&tab=notes");
    expect(failed.routerState.navigationCount).toBe(0);
  });

  it("does not clean an invalid project while the projects query is loading or errors", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockReturnValue(new Promise(() => undefined));
    const loading = renderResearchPage("en", "/research?workspace=w1&project=missing");
    await waitFor(() => expect(researchMock.listResearchProjects).toHaveBeenCalledWith("w1"));
    expect(loading.routerState.search).toBe("?workspace=w1&project=missing");
    expect(researchMock.listResearchDocuments).not.toHaveBeenCalled();
    expect(researchMock.listResearchReports).not.toHaveBeenCalled();
    loading.unmount();

    researchMock.listResearchProjects.mockRejectedValue(new Error("offline"));
    const failed = renderResearchPage("en", "/research?workspace=w1&project=missing");
    await waitFor(() => expect(researchMock.listResearchProjects).toHaveBeenCalledTimes(2));
    expect(failed.routerState.search).toBe("?workspace=w1&project=missing");
    expect(failed.routerState.navigationCount).toBe(0);
    expect(researchMock.listResearchDocuments).not.toHaveBeenCalled();
    expect(researchMock.listResearchReports).not.toHaveBeenCalled();
  });

  it("writes workspace and project selections as pushes while preserving other params", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([
      { id: "w1", name: "Workspace 1" },
      { id: "w2", name: "Workspace 2" },
    ]);
    researchMock.listResearchProjects.mockImplementation(async (id: string) =>
      id === "w1"
        ? [{ id: "p1", workspace_id: "w1", title: "Project 1", status: "active" }]
        : [{ id: "p2", workspace_id: "w2", title: "Project 2", status: "active" }],
    );
    const { routerState } = renderResearchPage("en", "/research?workspace=w1&project=p1&tab=notes");
    await waitFor(() =>
      expect(screen.getByRole("button", { name: "Project 1" })).toBeInTheDocument(),
    );
    fireEvent.change(screen.getByLabelText("Workspace"), { target: { value: "w2" } });
    await waitFor(() => expect(routerState.search).toBe("?workspace=w2&tab=notes"));
    expect(routerState.historyAction).toBe("PUSH");
    await waitFor(() =>
      expect(screen.getByRole("button", { name: "Project 2" })).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole("button", { name: "Project 2" }));
    await waitFor(() => expect(routerState.search).toBe("?workspace=w2&tab=notes&project=p2"));
    expect(routerState.historyAction).toBe("PUSH");
    expect(routerState.navigationCount).toBe(2);
  });

  it("removes a deleted selected project from the URL with replace", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockResolvedValue([
      { id: "p1", workspace_id: "w1", title: "Project 1", status: "active" },
    ]);
    const { routerState } = renderResearchPage("en", "/research?workspace=w1&project=p1&tab=notes");
    await waitFor(() =>
      expect(screen.getByRole("button", { name: "Project 1" })).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByTitle("deleteProject"));
    await waitFor(() => expect(routerState.search).toBe("?workspace=w1&tab=notes"));
    expect(routerState.historyAction).toBe("REPLACE");
    expect(routerState.navigationCount).toBe(1);
  });

  it("resets local document selection when the project context changes", async () => {
    workspaceMock.listWorkspaces.mockResolvedValue([{ id: "w1", name: "Workspace 1" }]);
    researchMock.listResearchProjects.mockResolvedValue([
      { id: "p1", workspace_id: "w1", title: "Project 1", status: "active" },
      { id: "p2", workspace_id: "w1", title: "Project 2", status: "active" },
    ]);
    researchMock.listResearchDocuments.mockImplementation(async (id: string) =>
      id === "p1" ? [{ id: "d1", project_id: "p1", title: "Document 1" }] : [],
    );
    renderResearchPage("en", "/research?workspace=w1&project=p1");
    await waitFor(() =>
      expect(screen.getByRole("button", { name: "Document 1" })).toBeInTheDocument(),
    );
    fireEvent.click(screen.getByRole("button", { name: "Document 1" }));
    await waitFor(() => expect(screen.getByText("Notes")).toBeInTheDocument());

    fireEvent.click(screen.getByRole("button", { name: "Project 2" }));
    await waitFor(() => expect(screen.queryByText("Notes")).not.toBeInTheDocument());
  });
});
