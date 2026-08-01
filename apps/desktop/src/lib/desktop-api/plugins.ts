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

export function listPlugins(): Promise<PluginStatus[]> {
  return invoke("list_plugins");
}

export function setPluginEnabled(pluginId: string, enabled: boolean): Promise<void> {
  return invoke("set_plugin_enabled", { pluginId, enabled });
}

export function createPluginArtifact(
  workspaceId: string,
  pluginId: InternalPluginId,
  input: unknown,
): Promise<Artifact> {
  const validatedInput = payloadSchemas[pluginId].parse(input);
  return invoke("create_plugin_artifact", { workspaceId, pluginId, input: validatedInput });
}
