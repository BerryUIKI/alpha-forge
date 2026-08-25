import { defineInternalPlugin } from "@alpha-forge/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "valuation-model", displayName: "Valuation Model", inputSchemaPath: "schema.json", rendererType: "valuation_model" });
