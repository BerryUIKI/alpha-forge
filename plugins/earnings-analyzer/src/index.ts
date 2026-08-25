import { defineInternalPlugin } from "@alpha-forge/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "earnings-analyzer", displayName: "Earnings Analyzer", inputSchemaPath: "schema.json", rendererType: "earnings_analysis" });
