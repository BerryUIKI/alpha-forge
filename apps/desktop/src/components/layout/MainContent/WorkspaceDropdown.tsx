/**
 * Workspace Dropdown Component
 *
 * Dropdown selector for workspaces, located in the top toolbar (OperationBar).
 * Displays current workspace name and allows switching between workspaces.
 *
 * @module components/layout/MainContent/WorkspaceDropdown
 */

import { useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ChevronDown, Plus, Check } from "lucide-react";
import { useWorkspaces } from "@/features/workspace";
import { useLocale } from "@/lib/i18n/useLocale";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { CreateWorkspaceDialog } from "@/features/workspace/components/CreateWorkspaceDialog";

export function WorkspaceDropdown() {
  const { t } = useLocale();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const { data: workspaces, isLoading } = useWorkspaces();
  const [isOpen, setIsOpen] = useState(false);
  const [showCreateDialog, setShowCreateDialog] = useState(false);

  const currentWorkspaceId = searchParams.get("workspace");
  const currentWorkspace = workspaces?.find(w => w.id === currentWorkspaceId);

  const handleSelectWorkspace = (workspaceId: string) => {
    setIsOpen(false);
    navigate(`/research?workspace=${workspaceId}`);
  };

  const handleCreateSuccess = (workspace: { id: string; name: string }) => {
    setShowCreateDialog(false);
    navigate(`/research?workspace=${workspace.id}`);
  };

  if (isLoading) {
    return (
      <div className="flex items-center gap-2 px-3 py-2">
        <LoadingSpinner size="sm" />
      </div>
    );
  }

  if (!workspaces || workspaces.length === 0) {
    return (
      <button
        onClick={() => setShowCreateDialog(true)}
        className="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent"
      >
        <Plus className="h-4 w-4" />
        <span>{t("createWorkspace" as any) || "创建工作区"}</span>
      </button>
    );
  }

  return (
    <>
      <div className="relative">
        {/* Trigger Button */}
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent"
          aria-expanded={isOpen}
          aria-haspopup="listbox"
          aria-label={t("selectWorkspace" as any) || "Select workspace"}
        >
          <span className="truncate max-w-[200px]">
            {currentWorkspace?.name || t("selectWorkspace" as any) || "选择工作区"}
          </span>
          <ChevronDown
            className={`h-4 w-4 flex-shrink-0 text-muted-foreground transition-transform ${
              isOpen ? "rotate-180" : ""
            }`}
          />
        </button>

        {/* Dropdown Menu */}
        {isOpen && (
          <>
            {/* Backdrop */}
            <div
              className="fixed inset-0 z-40"
              onClick={() => setIsOpen(false)}
            />

            {/* Menu */}
            <div
              className="absolute left-0 top-full z-50 mt-1 w-64 rounded-lg border border-border bg-card p-1 shadow-lg"
              role="listbox"
            >
              {/* Workspace List */}
              <div className="max-h-64 overflow-y-auto">
                {workspaces.map((workspace) => {
                  const isSelected = currentWorkspaceId === workspace.id;
                  return (
                    <button
                      key={workspace.id}
                      onClick={() => handleSelectWorkspace(workspace.id)}
                      className={`flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors ${
                        isSelected
                          ? "bg-primary/10 font-medium text-primary"
                          : "hover:bg-accent"
                      }`}
                      role="option"
                      aria-selected={isSelected}
                    >
                      <span className="flex-1 truncate">{workspace.name}</span>
                      {isSelected && (
                        <Check className="h-4 w-4 flex-shrink-0" />
                      )}
                    </button>
                  );
                })}
              </div>

              {/* Divider */}
              <div className="my-1 h-px bg-border" />

              {/* Create New Workspace Button */}
              <button
                onClick={() => {
                  setIsOpen(false);
                  setShowCreateDialog(true);
                }}
                className="flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm font-medium text-primary transition-colors hover:bg-accent"
              >
                <Plus className="h-4 w-4" />
                <span>{t("createWorkspace" as any) || "创建新工作区"}</span>
              </button>
            </div>
          </>
        )}
      </div>

      {/* Create Workspace Dialog */}
      <CreateWorkspaceDialog
        isOpen={showCreateDialog}
        onClose={() => setShowCreateDialog(false)}
        onSuccess={handleCreateSuccess}
      />
    </>
  );
}
