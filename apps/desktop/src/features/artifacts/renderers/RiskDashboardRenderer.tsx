// Risk dashboard artifact renderer.

import type { ArtifactRendererProps } from "./registry";

interface RiskItem {
  category: string;
  risk: string;
  severity: "high" | "medium" | "low";
  description?: string;
  mitigation?: string;
}

interface RiskDashboardData {
  portfolioName: string;
  totalRiskScore: number;
  risks: RiskItem[];
}

/**
 * Risk dashboard renderer for displaying portfolio risks.
 */
export function RiskDashboardRenderer({ data }: ArtifactRendererProps) {
  const riskData = data as RiskDashboardData;

  if (!riskData?.risks || riskData.risks.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        No risk data to display
      </div>
    );
  }

  const { portfolioName, totalRiskScore, risks } = riskData;

  const severityColors = {
    high: "bg-red-100 text-red-800 border-red-200",
    medium: "bg-yellow-100 text-yellow-800 border-yellow-200",
    low: "bg-green-100 text-green-800 border-green-200",
  };

  return (
    <div className="p-4">
      <div className="mb-6">
        <h2 className="text-xl font-semibold">{portfolioName} Risk Dashboard</h2>
        <div className="mt-2">
          <span className="text-sm text-muted-foreground">
            Overall Risk Score:{" "}
          </span>
          <span
            className={`font-semibold ${
              totalRiskScore > 70
                ? "text-red-600"
                : totalRiskScore > 40
                ? "text-yellow-600"
                : "text-green-600"
            }`}
          >
            {totalRiskScore.toFixed(0)}/100
          </span>
        </div>
      </div>

      <div className="space-y-3">
        {risks
          .sort((a, b) => {
            const severityOrder = { high: 0, medium: 1, low: 2 };
            return severityOrder[a.severity] - severityOrder[b.severity];
          })
          .map((risk, idx) => (
            <div
              key={idx}
              className="border rounded-lg p-4 hover:bg-muted/50"
            >
              <div className="flex items-start gap-3">
                <span
                  className={`inline-block px-2 py-1 text-xs font-medium rounded border ${
                    severityColors[risk.severity]
                  }`}
                >
                  {risk.severity.toUpperCase()}
                </span>
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-muted-foreground">
                      {risk.category}
                    </span>
                    <span className="font-medium">{risk.risk}</span>
                  </div>
                  {risk.description && (
                    <p className="text-sm text-muted-foreground mt-1">
                      {risk.description}
                    </p>
                  )}
                  {risk.mitigation && (
                    <div className="mt-2 text-sm">
                      <span className="font-medium">Mitigation: </span>
                      <span className="text-muted-foreground">
                        {risk.mitigation}
                      </span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          ))}
      </div>
    </div>
  );
}