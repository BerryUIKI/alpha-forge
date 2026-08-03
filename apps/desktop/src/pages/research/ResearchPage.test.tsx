// Tests for ResearchPage component.

import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
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

function renderResearchPage(locale: Locale = "zh-CN") {
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
      "en": {
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

  return render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <QueryClientProvider client={queryClient}>
        <ResearchPage />
      </QueryClientProvider>
    </LocaleContext.Provider>,
  );
}

describe("ResearchPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    workspaceMock.listWorkspaces.mockResolvedValue([]);
  });

  it("renders research page title in Chinese locale", () => {
    renderResearchPage("zh-CN");

    expect(screen.getByText("研究")).toBeInTheDocument();
    expect(screen.getByText("捕获项目、来源出处和文档注释。")).toBeInTheDocument();
  });

  it("renders research page title in English locale", () => {
    renderResearchPage("en");

    expect(screen.getByText("Research")).toBeInTheDocument();
    expect(screen.getByText("Capture projects, source provenance, and document annotations.")).toBeInTheDocument();
  });

  it("renders workspace selector", async () => {
    const mockWorkspaces = [
      { id: "1", name: "Test Workspace", createdAt: "2024-01-01T00:00:00Z", updatedAt: "2024-01-01T00:00:00Z" },
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
});