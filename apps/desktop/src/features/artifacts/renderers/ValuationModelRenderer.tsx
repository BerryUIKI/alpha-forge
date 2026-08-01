// Valuation model artifact renderer.

import type { ArtifactRendererProps } from "./registry";

interface ValuationScenario {
  name: string;
  fairValue: number;
  upside: number;
  assumptions: Record<string, number | string>;
}

interface ValuationData {
  company: string;
  ticker: string;
  currentPrice: number;
  methodology: string;
  scenarios: ValuationScenario[];
}

/**
 * Valuation model renderer for displaying company valuations.
 */
export function ValuationModelRenderer({ data }: ArtifactRendererProps) {
  const valData = data as ValuationData;

  if (!valData?.scenarios || valData.scenarios.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        No valuation data to display
      </div>
    );
  }

  const { company, ticker, currentPrice, methodology, scenarios } = valData;

  return (
    <div className="p-4">
      <div className="mb-6">
        <h2 className="text-xl font-semibold">
          {company} ({ticker}) Valuation
        </h2>
        <p className="text-sm text-muted-foreground mt-1">
          Methodology: {methodology} | Current Price: ${currentPrice.toFixed(2)}
        </p>
      </div>

      <div className="grid gap-4">
        {scenarios.map((scenario, idx) => (
          <div key={idx} className="border rounded-lg p-4">
            <div className="flex justify-between items-start mb-3">
              <h3 className="font-medium">{scenario.name}</h3>
              <div className="text-right">
                <div className="font-semibold">
                  ${scenario.fairValue.toFixed(2)}
                </div>
                <div
                  className={`text-sm ${
                    scenario.upside > 0
                      ? "text-green-600"
                      : scenario.upside < 0
                      ? "text-red-600"
                      : "text-muted-foreground"
                  }`}
                >
                  {scenario.upside > 0 ? "+" : ""}
                  {(scenario.upside * 100).toFixed(1)}% upside
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-2 text-sm">
              {Object.entries(scenario.assumptions).map(([key, value]) => (
                <div key={key} className="flex justify-between">
                  <span className="text-muted-foreground">{formatKey(key)}:</span>
                  <span className="font-medium">{formatValue(value)}</span>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function formatKey(key: string): string {
  return key
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

function formatValue(value: number | string): string {
  if (typeof value === "string") {
    return value;
  }
  if (value < 1 && value > -1) {
    return `${(value * 100).toFixed(1)}%`;
  }
  return value.toFixed(2);
}