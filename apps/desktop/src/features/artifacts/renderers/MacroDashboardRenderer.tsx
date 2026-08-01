import type { ArtifactRendererProps } from "./registry";

interface MacroIndicator {
  name: string;
  value: string | number;
  change?: string | number;
  interpretation?: string;
}

interface MacroDashboardData {
  asOf: string;
  indicators: MacroIndicator[];
}

export function MacroDashboardRenderer({ data }: ArtifactRendererProps) {
  const dashboard = data as MacroDashboardData;
  if (!dashboard?.asOf || !Array.isArray(dashboard.indicators) || dashboard.indicators.length === 0) {
    return <div className="p-4 text-center text-muted-foreground">No macro indicators to display</div>;
  }

  return (
    <div className="p-4 space-y-4">
      <div>
        <h2 className="text-xl font-semibold">Macro Dashboard</h2>
        <p className="text-sm text-muted-foreground">As of {dashboard.asOf}</p>
      </div>
      <div className="grid gap-3 md:grid-cols-2 lg:grid-cols-3">
        {dashboard.indicators.map((indicator, index) => (
          <section className="rounded-lg border p-3" key={`${indicator.name}-${index}`}>
            <div className="text-sm text-muted-foreground">{indicator.name}</div>
            <div className="font-medium">{String(indicator.value)}</div>
            {indicator.change !== undefined && <div className="text-sm">Change: {String(indicator.change)}</div>}
            {indicator.interpretation && <p className="mt-1 text-sm text-muted-foreground">{indicator.interpretation}</p>}
          </section>
        ))}
      </div>
    </div>
  );
}
