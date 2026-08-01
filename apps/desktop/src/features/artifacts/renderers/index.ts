// Artifact renderers index.

export * from "./registry";
export * from "./ComparisonTableRenderer";
export * from "./TimelineRenderer";
export * from "./IndustryMapRenderer";
export * from "./ValuationModelRenderer";
export * from "./RiskDashboardRenderer";

// Register built-in renderers
import { artifactRegistry } from "./registry";
import { ComparisonTableRenderer } from "./ComparisonTableRenderer";
import { TimelineRenderer } from "./TimelineRenderer";
import { IndustryMapRenderer } from "./IndustryMapRenderer";
import { ValuationModelRenderer } from "./ValuationModelRenderer";
import { RiskDashboardRenderer } from "./RiskDashboardRenderer";

// Auto-register built-in artifact renderers
artifactRegistry.register("comparison_table", ComparisonTableRenderer, {
  type: "comparison_table",
  name: "Comparison Table",
  description: "Compare multiple companies across dimensions",
});

artifactRegistry.register("timeline", TimelineRenderer, {
  type: "timeline",
  name: "Timeline",
  description: "Display chronological events and milestones",
});

artifactRegistry.register("industry_map", IndustryMapRenderer, {
  type: "industry_map",
  name: "Industry Map",
  description: "Visualize industry landscape and positioning",
});

artifactRegistry.register("valuation_model", ValuationModelRenderer, {
  type: "valuation_model",
  name: "Valuation Model",
  description: "Display company valuation scenarios",
});

artifactRegistry.register("risk_dashboard", RiskDashboardRenderer, {
  type: "risk_dashboard",
  name: "Risk Dashboard",
  description: "Portfolio risk analysis and monitoring",
});