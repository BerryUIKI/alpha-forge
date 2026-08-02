// Typed contract for internal plugins. Plugin code is declarative only; the
// desktop runtime owns validation, permissions, lifecycle, and rendering.

export type ArtifactRendererType =
  | "comparison_table"
  | "valuation_model"
  | "industry_map"
  | "risk_dashboard"
  | "timeline"
  | "earnings_analysis"
  | "macro_dashboard";

export interface InternalPluginDefinition {
  id: string;
  displayName: string;
  inputSchemaPath: "schema.json";
  rendererType: ArtifactRendererType;
}

export function defineInternalPlugin(
  definition: InternalPluginDefinition,
): Readonly<InternalPluginDefinition> {
  if (!/^[a-z0-9-]{1,64}$/.test(definition.id)) {
    throw new Error("Internal plugin id must be lowercase kebab-case");
  }
  if (!definition.displayName.trim()) {
    throw new Error("Internal plugin display name is required");
  }
  return Object.freeze({ ...definition });
}
