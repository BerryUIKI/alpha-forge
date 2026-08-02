import { defineInternalPlugin } from "@investment-os/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "portfolio-risk", displayName: "Portfolio Risk", inputSchemaPath: "schema.json", rendererType: "risk_dashboard" });
