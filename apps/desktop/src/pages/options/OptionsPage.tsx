/**
 * Options Page
 *
 * Options analysis page using M9 components.
 * Displays Greeks Calculator, Option Chains, and Strategy Builder.
 *
 * @module pages/options/OptionsPage
 */

import { useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import { GreeksCalculator, OptionChainList, StrategyBuilder } from "@/features/options";
import { EmptyState } from "@/components/common/EmptyState";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@radix-ui/react-select";

export function OptionsPage() {
  const { t } = useLocale();
  const [searchParams, setSearchParams] = useSearchParams();
  const workspaceIdFromUrl = searchParams.get("workspace") || "";
  const [selectedWorkspaceId, setSelectedWorkspaceId] = useState(workspaceIdFromUrl);

  // Fetch workspaces for selection
  const { data: workspaces } = useQuery({
    queryKey: ["workspaces"],
    queryFn: desktopApi.workspace.listWorkspaces,
  });

  // Handle workspace selection
  const handleWorkspaceChange = (id: string) => {
    setSelectedWorkspaceId(id);
    setSearchParams({ workspace: id });
  };

  // No workspace selected state
  if (!selectedWorkspaceId) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6">
        <EmptyState
          title={t("selectWorkspace" as any) || "Select a Workspace"}
          description={t("selectWorkspaceDescription" as any) || "Choose a workspace to access options analysis tools."}
        />
        {workspaces && workspaces.length > 0 && (
          <div className="mt-4 w-full max-w-xs">
            <select
              className="w-full rounded-lg border border-border bg-background p-2"
              value={selectedWorkspaceId}
              onChange={(e) => handleWorkspaceChange(e.target.value)}
            >
              <option value="">{t("selectWorkspace" as any) || "Select a workspace"}</option>
              {workspaces.map((workspace) => (
                <option key={workspace.id} value={workspace.id}>
                  {workspace.name}
                </option>
              ))}
            </select>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className="space-y-6 p-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold">{t("optionsTitle" as any) || "Options Analysis"}</h1>
        <p className="text-sm text-muted-foreground">
          {t("optionsDescription" as any) || "Black-Scholes pricing, Greeks calculation, and strategy analysis."}
        </p>
      </div>

      {/* Workspace Selector */}
      <div className="max-w-xs">
        <label className="block text-sm font-medium">
          {t("workspace" as any) || "Workspace"}
        </label>
        <select
          className="mt-1 w-full rounded-lg border border-border bg-background p-2"
          value={selectedWorkspaceId}
          onChange={(e) => handleWorkspaceChange(e.target.value)}
        >
          <option value="">{t("selectWorkspace" as any) || "Select a workspace"}</option>
          {workspaces?.map((workspace) => (
            <option key={workspace.id} value={workspace.id}>
              {workspace.name}
            </option>
          ))}
        </select>
      </div>

      {/* Tools Grid */}
      <div className="grid gap-6 lg:grid-cols-2">
        {/* Greeks Calculator */}
        <GreeksCalculator />

        {/* Strategy Builder */}
        <StrategyBuilder />
      </div>

      {/* Option Chains */}
      <OptionChainList
        workspaceId={selectedWorkspaceId}
        onSelectChain={(chainId) => {
          // TODO: Navigate to chain detail or show in modal
          console.log("Selected chain:", chainId);
        }}
      />
    </div>
  );
}