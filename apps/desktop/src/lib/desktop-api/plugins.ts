// Plugin registry desktop API.

import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";
import type { Artifact } from "./artifacts";

export const COMPANY_COMPARISON_PLUGIN_ID = "company-comparison" as const;
export const COMPANY_COMPARISON_DIMENSIONS = ["revenue", "market_cap", "pe_ratio"] as const;

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
const ArtifactResponseSchema = z
  .object({
    id: z.string().uuid(),
    workspace_id: nonEmptyText,
    task_id: z.string().nullable(),
    artifact_type: nonEmptyText,
    status: z.enum(["pending", "generating", "completed", "viewing", "closed", "failed"]),
    input: z.unknown(),
    output: z.unknown().nullable(),
    error: z.string().nullable(),
    created_at: z.string().datetime({ offset: true }),
    updated_at: z.string().datetime({ offset: true }),
  })
  .strict();
const companyComparisonDimensionSchema = z.enum(COMPANY_COMPARISON_DIMENSIONS);
export const CompanyComparisonPayloadSchema = z
  .object({
    companies: z
      .array(
        z
          .object({
            ticker: z
              .string()
              .trim()
              .regex(/^[A-Za-z0-9.-]{1,12}$/)
              .transform((ticker) => ticker.toUpperCase()),
            name: nonEmptyText.max(100),
            metrics: z.record(z.number().finite()),
          })
          .strict(),
      )
      .min(2),
    comparisonDimensions: z.array(companyComparisonDimensionSchema).min(1),
  })
  .strict()
  .superRefine((payload, context) => {
    const tickers = payload.companies.map((company) => company.ticker);
    if (new Set(tickers).size !== tickers.length) {
      context.addIssue({ code: z.ZodIssueCode.custom, message: "Company tickers must be unique" });
    }
    if (new Set(payload.comparisonDimensions).size !== payload.comparisonDimensions.length) {
      context.addIssue({
        code: z.ZodIssueCode.custom,
        message: "Comparison dimensions must be unique",
      });
    }
    payload.companies.forEach((company, companyIndex) => {
      payload.comparisonDimensions.forEach((dimension) => {
        if (company.metrics[dimension] === undefined) {
          context.addIssue({
            code: z.ZodIssueCode.custom,
            message: `Company metrics must include ${dimension}`,
            path: ["companies", companyIndex, "metrics", dimension],
          });
        }
      });
    });
  });
const payloadSchemas = {
  "company-comparison": CompanyComparisonPayloadSchema,
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
export type CompanyComparisonDimension = (typeof COMPANY_COMPARISON_DIMENSIONS)[number];
export type CompanyComparisonPayload = z.infer<typeof CompanyComparisonPayloadSchema>;

export async function listPlugins(): Promise<PluginStatus[]> {
  const response: unknown = await invoke("list_plugins");
  return z.array(PluginStatusSchema).parse(response);
}

export async function setPluginEnabled(pluginId: string, enabled: boolean): Promise<void> {
  const response: unknown = await invoke("set_plugin_enabled", { pluginId, enabled });
  VoidResponseSchema.parse(response);
}

export async function createPluginArtifact(
  workspaceId: string,
  pluginId: InternalPluginId,
  input: unknown,
): Promise<Artifact> {
  const validatedInput = payloadSchemas[pluginId].parse(input);
  const response: unknown = await invoke("create_plugin_artifact", {
    workspaceId,
    pluginId,
    input: validatedInput,
  });
  return ArtifactResponseSchema.parse(response) as Artifact;
}
