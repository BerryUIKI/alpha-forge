import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import "@testing-library/jest-dom";

import { ComparisonTableRenderer } from "./ComparisonTableRenderer";

describe("ComparisonTableRenderer", () => {
  it("renders the validated company comparison payload", () => {
    render(
      <ComparisonTableRenderer
        artifactId="2a707687-3fc5-4b02-81ba-043830213244"
        data={{
          companies: [
            { ticker: "AAPL", name: "Apple", metrics: { market_cap: 3_100_000_000_000 } },
            { ticker: "MSFT", name: "Microsoft", metrics: { market_cap: 2_900_000_000_000 } },
          ],
          comparisonDimensions: ["market_cap"],
        }}
      />,
    );

    expect(screen.getByRole("columnheader", { name: "Market Cap" })).toBeInTheDocument();
    expect(screen.getByText("AAPL")).toBeInTheDocument();
    expect(screen.getByText("Apple")).toBeInTheDocument();
    expect(screen.getByText("$3.10T")).toBeInTheDocument();
    expect(screen.getByText("$2.90T")).toBeInTheDocument();
  });
});
