// Industry map artifact renderer.

import type { ArtifactRendererProps } from "./registry";

interface Company {
  name: string;
  ticker?: string;
  marketShare?: number;
  category: string;
}

interface IndustryMapData {
  industry: string;
  companies: Company[];
  categories: string[];
}

/**
 * Industry map renderer for visualizing industry landscape.
 */
export function IndustryMapRenderer({ data }: ArtifactRendererProps) {
  const mapData = data as IndustryMapData;

  if (!mapData?.companies || mapData.companies.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        No industry data to display
      </div>
    );
  }

  const { industry, companies, categories } = mapData;

  return (
    <div className="p-4">
      <h2 className="text-xl font-semibold mb-4">{industry} Industry Map</h2>
      <div className="space-y-6">
        {categories.map((category) => {
          const categoryCompanies = companies.filter(
            (c) => c.category === category
          );
          if (categoryCompanies.length === 0) return null;

          return (
            <div key={category}>
              <h3 className="text-sm font-medium text-muted-foreground mb-2">
                {category}
              </h3>
              <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
                {categoryCompanies.map((company, idx) => (
                  <div
                    key={idx}
                    className="border rounded-lg p-3 hover:bg-muted/50"
                  >
                    <div className="font-medium">{company.name}</div>
                    {company.ticker && (
                      <div className="text-sm text-muted-foreground">
                        {company.ticker}
                      </div>
                    )}
                    {company.marketShare && (
                      <div className="text-sm mt-1">
                        Market Share: {(company.marketShare * 100).toFixed(1)}%
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}