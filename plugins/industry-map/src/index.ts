import { defineInternalPlugin } from "@investment-os/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "industry-map", displayName: "Industry Map", inputSchemaPath: "schema.json", rendererType: "industry_map" });
