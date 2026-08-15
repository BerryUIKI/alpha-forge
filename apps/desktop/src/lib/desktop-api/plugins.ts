// Plugin registry desktop API.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import type { Artifact } from "./artifacts";

export type PluginPermission = "network";

export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  entry: string;
  inputSchema: string;
  permissions: PluginPermission[];
  window: {
    width: number;
    height: number;
    resizable: boolean;
  };
}

export interface PluginStatus {
  manifest: PluginManifest;
  enabled: boolean;
}

const nonEmptyText = z.string().trim().min(1);
const safeRelativeFile = nonEmptyText.refine(
  (value) => !value.startsWith("/") && !value.includes("..") && !value.includes("\\"),
  "Plugin file references must be safe relative paths",
);
const PluginPermissionSchema = z.enum(["network"]);
const PluginManifestSchema = z
  .object({
    id: z.string().regex(/^[a-z0-9-]{1,64}$/),
    name: nonEmptyText,
    version: nonEmptyText,
    entry: safeRelativeFile,
    inputSchema: safeRelativeFile,
    permissions: z.array(PluginPermissionSchema),
    window: z
      .object({
        width: z.number().int().positive().max(1600),
        height: z.number().int().positive().max(1200),
        resizable: z.boolean(),
      })
      .strict(),
  })
  .strict();
const PluginStatusSchema = z
  .object({ manifest: PluginManifestSchema, enabled: z.boolean() })
  .strict();
const VoidResponseSchema = z.union([z.null(), z.undefined()]);
const payloadSchemas = {
  "company-comparison": z.object({
    companies: z.array(z.object({ ticker: nonEmptyText }).passthrough()).min(2),
    comparisonDimensions: z.array(nonEmptyText).min(1),
  }).passthrough(),
  "valuation-model": z.object({
    company: nonEmptyText, ticker: nonEmptyText, currentPrice: z.number().finite(),
    methodology: nonEmptyText, scenarios: z.array(z.unknown()).min(1),
  }).passthrough(),
  "industry-map": z.object({
    industry: nonEmptyText, companies: z.array(z.unknown()).min(1), categories: z.array(nonEmptyText).min(1),
  }).passthrough(),
  "portfolio-risk": z.object({
    portfolioName: nonEmptyText, totalRiskScore: z.number().finite().min(0).max(100), risks: z.array(z.unknown()).min(1),
  }).passthrough(),
  "research-timeline": z.object({ events: z.array(z.unknown()).min(1) }).passthrough(),
  "earnings-analyzer": z.object({
    company: nonEmptyText, ticker: nonEmptyText, period: nonEmptyText, highlights: z.array(z.unknown()).min(1),
  }).passthrough(),
  "macro-dashboard": z.object({ asOf: nonEmptyText, indicators: z.array(z.unknown()).min(1) }).passthrough(),
} as const;

export type InternalPluginId = keyof typeof payloadSchemas;

export async function listPlugins(): Promise<PluginStatus[]> {
  const response: unknown = await invoke("list_plugins");
  return z.array(PluginStatusSchema).parse(response);
}

export async function setPluginEnabled(pluginId: string, enabled: boolean): Promise<void> {
  const response: unknown = await invoke("set_plugin_enabled", { pluginId, enabled });
  VoidResponseSchema.parse(response);
}

export function createPluginArtifact(
  workspaceId: string,
  pluginId: InternalPluginId,
  input: unknown,
): Promise<Artifact> {
  const validatedInput = payloadSchemas[pluginId].parse(input);
  return invoke("create_plugin_artifact", { workspaceId, pluginId, input: validatedInput });
}
