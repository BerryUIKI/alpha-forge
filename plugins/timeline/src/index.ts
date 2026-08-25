import { defineInternalPlugin } from "@alpha-forge/artifact-sdk";

export const plugin = defineInternalPlugin({ id: "research-timeline", displayName: "Research Timeline", inputSchemaPath: "schema.json", rendererType: "timeline" });
