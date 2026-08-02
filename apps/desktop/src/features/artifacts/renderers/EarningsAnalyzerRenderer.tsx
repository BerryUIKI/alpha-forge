import type { ArtifactRendererProps } from "./registry";

interface EarningsHighlight {
  label: string;
  value: string | number;
  commentary?: string;
}

interface EarningsAnalyzerData {
  company: string;
  ticker: string;
  period: string;
  highlights: EarningsHighlight[];
}

export function EarningsAnalyzerRenderer({ data }: ArtifactRendererProps) {
  const earnings = data as EarningsAnalyzerData;
  if (!earnings?.company || !earnings?.ticker || !earnings?.period || !Array.isArray(earnings.highlights) || earnings.highlights.length === 0) {
    return <div className="p-4 text-center text-muted-foreground">No earnings analysis to display</div>;
  }

  return (
    <div className="p-4 space-y-4">
      <div>
        <h2 className="text-xl font-semibold">{earnings.company} ({earnings.ticker})</h2>
        <p className="text-sm text-muted-foreground">Earnings period: {earnings.period}</p>
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        {earnings.highlights.map((highlight, index) => (
          <section className="rounded-lg border p-3" key={`${highlight.label}-${index}`}>
            <div className="text-sm text-muted-foreground">{highlight.label}</div>
            <div className="font-medium">{String(highlight.value)}</div>
            {highlight.commentary && <p className="mt-1 text-sm text-muted-foreground">{highlight.commentary}</p>}
          </section>
        ))}
      </div>
    </div>
  );
}
