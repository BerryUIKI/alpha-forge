// Comparison table artifact renderer.

import type { ArtifactRendererProps } from "./registry";

interface CompanyData {
  ticker: string;
  name: string;
  metrics: Record<string, number | string>;
}

interface ComparisonTableData {
  companies: CompanyData[];
  comparisonDimensions: string[];
}

/**
 * Comparison table renderer for comparing multiple companies.
 */
export function ComparisonTableRenderer({ data }: ArtifactRendererProps) {
  const tableData = data as ComparisonTableData;

  if (!tableData?.companies || !tableData?.comparisonDimensions) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        Invalid comparison table data
      </div>
    );
  }

  const { companies, comparisonDimensions } = tableData;

  if (companies.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        No companies to compare
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b">
            <th className="p-3 text-left font-medium">Company</th>
            {comparisonDimensions.map((dim) => (
              <th key={dim} className="p-3 text-right font-medium">
                {formatDimensionName(dim)}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {companies.map((company) => (
            <tr key={company.ticker} className="border-b hover:bg-muted/50">
              <td className="p-3">
                <div className="font-medium">{company.ticker}</div>
                <div className="text-sm text-muted-foreground">
                  {company.name}
                </div>
              </td>
              {comparisonDimensions.map((dim) => (
                <td key={dim} className="p-3 text-right">
                  {formatMetric(company.metrics[dim])}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

/**
 * Format a dimension name for display.
 */
function formatDimensionName(name: string): string {
  return name
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

/**
 * Format a metric value for display.
 */
function formatMetric(value: number | string | undefined): string {
  if (value === undefined || value === null) {
    return "-";
  }

  if (typeof value === "string") {
    return value;
  }

  // Format large numbers
  if (value >= 1e12) {
    return `$${(value / 1e12).toFixed(2)}T`;
  }
  if (value >= 1e9) {
    return `$${(value / 1e9).toFixed(2)}B`;
  }
  if (value >= 1e6) {
    return `$${(value / 1e6).toFixed(2)}M`;
  }

  // Format percentages
  if (value < 1 && value > -1 && value !== 0) {
    return `${(value * 100).toFixed(2)}%`;
  }

  // Default formatting
  return value.toFixed(2);
}