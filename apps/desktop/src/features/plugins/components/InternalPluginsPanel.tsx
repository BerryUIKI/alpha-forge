import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Plug, ShieldCheck } from "lucide-react";

import { EmptyState } from "@/components/common/EmptyState";
import { ErrorState } from "@/components/common/ErrorState";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "@/lib/i18n/useLocale";
import type { PluginStatus } from "@/lib/desktop-api/plugins";

const PLUGINS_QUERY_KEY = ["internalPlugins"] as const;

export function InternalPluginsPanel() {
  const { t } = useLocale();
  const queryClient = useQueryClient();
  const pluginsQuery = useQuery({
    queryKey: PLUGINS_QUERY_KEY,
    queryFn: desktopApi.plugins.listPlugins,
  });
  const toggleMutation = useMutation({
    mutationFn: ({ pluginId, enabled }: { pluginId: string; enabled: boolean }) =>
      desktopApi.plugins.setPluginEnabled(pluginId, enabled),
    onSuccess: (_, variables) => {
      queryClient.setQueryData<PluginStatus[]>(PLUGINS_QUERY_KEY, (current) =>
        current?.map((plugin) =>
          plugin.manifest.id === variables.pluginId
            ? { ...plugin, enabled: variables.enabled }
            : plugin,
        ),
      );
      void queryClient.invalidateQueries({ queryKey: PLUGINS_QUERY_KEY });
    },
  });

  return (
    <section id="internal-plugins" className="rounded-lg border border-border bg-card p-5">
      <div className="flex items-start gap-3">
        <Plug className="mt-0.5 h-5 w-5" aria-hidden="true" />
        <div>
          <h2 className="font-semibold">{t("internalPlugins")}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t("internalPluginsDescription")}</p>
        </div>
      </div>

      {pluginsQuery.isLoading && (
        <div className="mt-4 flex items-center gap-2 text-sm" role="status">
          <LoadingSpinner size="sm" />
          {t("loadingInternalPlugins")}
        </div>
      )}

      {pluginsQuery.isError && (
        <ErrorState
          message={t("failedToLoadInternalPlugins")}
          onRetry={() => void pluginsQuery.refetch()}
        />
      )}

      {toggleMutation.isError && <ErrorState message={t("failedToUpdateInternalPlugin")} />}

      {pluginsQuery.data?.length === 0 && (
        <EmptyState
          title={t("noInternalPlugins")}
          description={t("noInternalPluginsDescription")}
        />
      )}

      {pluginsQuery.data && pluginsQuery.data.length > 0 && (
        <ul className="mt-4 space-y-3">
          {pluginsQuery.data.map((plugin) => {
            const isUpdating =
              toggleMutation.isPending && toggleMutation.variables?.pluginId === plugin.manifest.id;
            const action = plugin.enabled ? t("disableInternalPlugin") : t("enableInternalPlugin");
            return (
              <li
                key={plugin.manifest.id}
                className="flex flex-col gap-3 rounded-md border border-border p-4 sm:flex-row sm:items-center sm:justify-between"
              >
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <h3 className="font-medium">{plugin.manifest.name}</h3>
                    <span className="rounded bg-muted px-2 py-0.5 text-xs">
                      {t("internalPluginBadge")}
                    </span>
                    <span className="text-xs text-muted-foreground">
                      v{plugin.manifest.version}
                    </span>
                  </div>
                  <p className="mt-1 break-all text-xs text-muted-foreground">
                    {plugin.manifest.id}
                  </p>
                  <div className="mt-2 flex items-center gap-1 text-xs text-muted-foreground">
                    <ShieldCheck className="h-3.5 w-3.5" aria-hidden="true" />
                    <span>{t("pluginPermissions")}:</span>
                    <span>
                      {plugin.manifest.permissions.length === 0
                        ? t("noPluginPermissions")
                        : plugin.manifest.permissions
                            .map(() => t("pluginPermissionNetwork"))
                            .join(", ")}
                    </span>
                  </div>
                </div>
                <button
                  type="button"
                  role="switch"
                  aria-checked={plugin.enabled}
                  aria-label={`${plugin.manifest.name}: ${action}`}
                  disabled={toggleMutation.isPending}
                  onClick={() =>
                    toggleMutation.mutate({
                      pluginId: plugin.manifest.id,
                      enabled: !plugin.enabled,
                    })
                  }
                  className="rounded-md border border-input px-3 py-2 text-sm font-medium hover:bg-accent disabled:opacity-50"
                >
                  {isUpdating
                    ? t("updatingInternalPlugin")
                    : plugin.enabled
                      ? t("pluginEnabled")
                      : t("pluginDisabled")}
                </button>
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}
