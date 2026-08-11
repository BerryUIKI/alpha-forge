/**
 * OperationBar Component
 *
 * Top toolbar for main content area with new structure:
 * [搜索图标] [工作区下拉选择器] [+新建按钮] [Agent设置按钮]
 *
 * @version GUI-M3
 */

import { useState } from "react";
import { useNavigate, useLocation, useSearchParams } from "react-router-dom";
import { Search, Plus, Settings, ChevronDown } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useWorkspaces } from "@/features/workspace";
import { WorkspaceDropdown } from "./WorkspaceDropdown";
import { CreateResearchProjectDialog } from "@/features/research/components/CreateResearchProjectDialog";
import { CreateWorkspaceDialog } from "@/features/workspace/components/CreateWorkspaceDialog";
import type { OperationBarProps } from "@/components/layout/types";

export function OperationBar({
  isRightSidebarExpanded = false,
  onToggleRightSidebar,
}: OperationBarProps) {
  const { t } = useLocale();
  const navigate = useNavigate();
  const location = useLocation();
  const [searchParams] = useSearchParams();
  const [showCreateMenu, setShowCreateMenu] = useState(false);
  const [showCreateProjectDialog, setShowCreateProjectDialog] = useState(false);
  const [showCreateWorkspaceDialog, setShowCreateWorkspaceDialog] = useState(false);

  const { data: workspaces } = useWorkspaces();
  const workspaceId = searchParams.get("workspace") || workspaces?.[0]?.id || "";

  const handleAgentSettings = () => {
    navigate("/settings#agent");
  };

  const handleCreateProject = () => {
    setShowCreateMenu(false);
    if (workspaceId) {
      setShowCreateProjectDialog(true);
    }
  };

  const handleCreateWorkspace = () => {
    setShowCreateMenu(false);
    setShowCreateWorkspaceDialog(true);
  };

  const handleCreateSuccess = (project: { id: string; title: string }) => {
    setShowCreateProjectDialog(false);
    navigate(`/research?workspace=${workspaceId}&project=${project.id}`);
  };

  const handleWorkspaceCreated = (workspace: { id: string; name: string }) => {
    setShowCreateWorkspaceDialog(false);
    navigate(`/research?workspace=${workspace.id}`);
  };

  return (
    <>
      <div className="flex items-center justify-between border-b border-border bg-card px-4 py-3">
        {/* Left: Search */}
        <div className="flex items-center gap-3">
          <button
            className="flex h-8 w-8 items-center justify-center rounded-lg transition-colors hover:bg-accent"
            aria-label={t("search" as any) || "Search"}
            title={`${t("search" as any) || "Search"} (Ctrl+K)`}
          >
            <Search className="h-4 w-4 text-muted-foreground" />
          </button>

          {/* Workspace Dropdown */}
          <WorkspaceDropdown />
        </div>

        {/* Right: Create + Agent Settings */}
        <div className="flex items-center gap-2">
          {/* Create Button with Dropdown */}
          <div className="relative">
            <button
              onClick={() => setShowCreateMenu(!showCreateMenu)}
              className="flex h-8 items-center gap-1 rounded-lg px-3 transition-colors hover:bg-accent"
              aria-label={t("createNew" as any) || "Create new"}
            >
              <Plus className="h-4 w-4" />
              <span className="text-sm font-medium">{t("createNew" as any) || "新建"}</span>
              <ChevronDown className="h-3 w-3" />
            </button>

            {/* Create Menu */}
            {showCreateMenu && (
              <>
                <div
                  className="fixed inset-0 z-40"
                  onClick={() => setShowCreateMenu(false)}
                />
                <div className="absolute right-0 top-full z-50 mt-1 w-48 rounded-lg border border-border bg-card p-1 shadow-lg">
                  <button
                    onClick={handleCreateWorkspace}
                    className="flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-accent"
                  >
                    <Plus className="h-4 w-4" />
                    <span>{t("createWorkspace" as any) || "新建工作区"}</span>
                  </button>
                  <button
                    onClick={handleCreateProject}
                    disabled={!workspaceId}
                    className="flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors hover:bg-accent disabled:cursor-not-allowed disabled:opacity-50"
                  >
                    <Plus className="h-4 w-4" />
                    <span>{t("createResearchProject" as any) || "新建研究任务"}</span>
                  </button>
                </div>
              </>
            )}
          </div>

          {/* Divider */}
          <div className="h-6 w-px bg-border" />

          {/* Agent Settings Button */}
          <button
            onClick={handleAgentSettings}
            className="flex h-8 items-center gap-2 rounded-lg px-3 transition-colors hover:bg-accent"
            aria-label={t("agentSettings" as any) || "Agent settings"}
          >
            <Settings className="h-4 w-4" />
            <span className="text-sm font-medium">{t("agentSettings" as any) || "Agent设置"}</span>
          </button>
        </div>
      </div>

      {/* Create Research Project Dialog */}
      <CreateResearchProjectDialog
        isOpen={showCreateProjectDialog}
        onClose={() => setShowCreateProjectDialog(false)}
        workspaceId={workspaceId}
        onSuccess={handleCreateSuccess}
      />

      {/* Create Workspace Dialog */}
      <CreateWorkspaceDialog
        isOpen={showCreateWorkspaceDialog}
        onClose={() => setShowCreateWorkspaceDialog(false)}
        onSuccess={handleWorkspaceCreated}
      />
    </>
  );
}
