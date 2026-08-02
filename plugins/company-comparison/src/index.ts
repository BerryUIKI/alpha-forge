import { defineInternalPlugin } from "@investment-os/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "company-comparison", displayName: "Company Comparison", inputSchemaPath: "schema.json", rendererType: "comparison_table" });
