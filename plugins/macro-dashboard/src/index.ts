import { defineInternalPlugin } from "@investment-os/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "macro-dashboard", displayName: "Macro Dashboard", inputSchemaPath: "schema.json", rendererType: "macro_dashboard" });
