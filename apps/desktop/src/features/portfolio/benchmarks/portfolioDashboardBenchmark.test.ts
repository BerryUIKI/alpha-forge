/**
 * Portfolio Dashboard Frontend Scale & Performance Benchmark (M11-03)
 *
 * Measures client-side transformation throughput and UI-thread latency
 * across representative large-portfolio datasets:
 * 1. Valuation Time-Series Aggregation & Downsampling (1,250 points across 5 accounts)
 * 2. Activity Filtering, Multi-Field Search & Chronological Sorting (1,000 rows)
 * 3. Holdings Grid Enrichment & Formatting (50 positions with gain/loss computations)
 * 4. Taxonomy Allocation & Drift Calculation (50 asset assignments across 4 categories)
 *
 * Budget: Every client-side transform must complete in < 50ms to ensure 60fps UI responsiveness.
 */

import { describe, expect, it } from "vitest";
import {
  fmtGainLoss,
  fmtMoney,
  fmtPercent,
  gainLossClass,
  parseDecimal,
} from "../components/helpers";

interface ValuationPoint {
  account_id: string;
  valuation_date: string;
  total_value_base: string;
  cash_balance_base: string;
  currency: string;
}

interface ActivityRow {
  id: string;
  account_id: string;
  activity_type: string;
  activity_date: string;
  symbol: string | null;
  quantity: string | null;
  unit_price: string | null;
  amount: string;
  currency: string;
  notes: string | null;
}

interface HoldingRow {
  id: string;
  asset_id: string;
  symbol: string;
  name: string;
  quantity: string;
  cost_basis: string;
  market_value: string;
  unrealized_gain: string;
  unrealized_gain_pct: string;
}

interface AllocationPosition {
  assetId: string;
  category: string;
  marketValue: number;
}

// Generate benchmark datasets
function generateValuationSeries(accountsCount: number, daysCount: number): ValuationPoint[] {
  const points: ValuationPoint[] = [];
  const baseDate = new Date("2025-01-01T00:00:00Z");

  for (let a = 0; a < accountsCount; a++) {
    const accId = `acc-${a + 1}`;
    for (let d = 0; d < daysCount; d++) {
      const dt = new Date(baseDate.getTime() + d * 86400000);
      const val = 100000 + a * 25000 + d * 120 + ((d * 7) % 500);
      const cash = 10000 + ((d * 3) % 1000);
      points.push({
        account_id: accId,
        valuation_date: dt.toISOString().slice(0, 10),
        total_value_base: val.toFixed(2),
        cash_balance_base: cash.toFixed(2),
        currency: "USD",
      });
    }
  }
  return points;
}

function generateActivities(count: number): ActivityRow[] {
  const types = ["BUY", "SELL", "DIVIDEND", "DEPOSIT", "FEE"];
  const symbols = ["AAPL", "MSFT", "GOOGL", "AMZN", "NVDA", "TSLA", "SPY", "QQQ", "BTC", "ETH"];
  const rows: ActivityRow[] = [];
  const baseDate = new Date("2025-01-01T00:00:00Z");

  for (let i = 0; i < count; i++) {
    const actType = types[i % types.length]!;
    const sym = actType === "DEPOSIT" || actType === "FEE" ? null : symbols[i % symbols.length]!;
    const dt = new Date(baseDate.getTime() + (i % 250) * 86400000);
    const qty = sym ? (10 + (i % 50)).toString() : null;
    const price = sym ? (150 + (i % 30)).toFixed(2) : null;
    const amount = (sym ? (parseFloat(qty!) * parseFloat(price!)).toFixed(2) : (500 + i * 5).toFixed(2));

    rows.push({
      id: `act-${i + 1}`,
      account_id: `acc-${(i % 5) + 1}`,
      activity_type: actType,
      activity_date: dt.toISOString().slice(0, 10),
      symbol: sym,
      quantity: qty,
      unit_price: price,
      amount,
      currency: "USD",
      notes: `Benchmark transaction #${i + 1} execution statement`,
    });
  }
  return rows;
}

function generateHoldings(count: number): HoldingRow[] {
  const rows: HoldingRow[] = [];
  for (let i = 0; i < count; i++) {
    const cost = 5000 + i * 200;
    const market = cost * (1 + ((i % 11) - 4) * 0.05); // -20% to +30%
    const gain = market - cost;
    const gainPct = (gain / cost) * 100;

    rows.push({
      id: `pos-${i + 1}`,
      asset_id: `asset-${i + 1}`,
      symbol: `SYM-${i + 1}`,
      name: `Core Holding Asset #${i + 1}`,
      quantity: (25 + i).toString(),
      cost_basis: cost.toFixed(2),
      market_value: market.toFixed(2),
      unrealized_gain: gain.toFixed(2),
      unrealized_gain_pct: gainPct.toFixed(2),
    });
  }
  return rows;
}

describe("Portfolio Dashboard Client Scale & Performance Baseline (M11-03)", () => {
  it("aggregates and downsamples 1,250 valuation points in < 50ms", () => {
    const valuations = generateValuationSeries(5, 250);
    expect(valuations.length).toBe(1250);

    const start = performance.now();

    // Group by valuation_date across all accounts to build total portfolio line
    const dateMap = new Map<string, { total: number; cash: number }>();
    for (const v of valuations) {
      const existing = dateMap.get(v.valuation_date);
      const totalVal = parseDecimal(v.total_value_base);
      const cashVal = parseDecimal(v.cash_balance_base);

      if (existing) {
        existing.total += totalVal;
        existing.cash += cashVal;
      } else {
        dateMap.set(v.valuation_date, { total: totalVal, cash: cashVal });
      }
    }

    // Sort chronologically and build chart array
    const chartSeries = Array.from(dateMap.entries())
      .map(([date, values]) => ({
        date,
        total: Math.round(values.total * 100) / 100,
        cash: Math.round(values.cash * 100) / 100,
      }))
      .sort((a, b) => a.date.localeCompare(b.date));

    const elapsed = performance.now() - start;

    expect(chartSeries.length).toBe(250);
    expect(chartSeries[0]!.total).toBeGreaterThan(0);
    expect(elapsed).toBeLessThan(50);
  });

  it("filters, searches, and sorts 1,000 activities in < 50ms", () => {
    const activities = generateActivities(1000);
    expect(activities.length).toBe(1000);

    const start = performance.now();

    const searchQuery = "AAPL";
    const typeFilter = "BUY";
    const minDate = "2025-02-01";
    const maxDate = "2025-07-01";

    const filtered = activities
      .filter((a) => {
        if (typeFilter && a.activity_type !== typeFilter) return false;
        if (a.activity_date < minDate || a.activity_date > maxDate) return false;
        if (searchQuery) {
          const q = searchQuery.toLowerCase();
          const matchesSym = a.symbol?.toLowerCase().includes(q) ?? false;
          const matchesNotes = a.notes?.toLowerCase().includes(q) ?? false;
          if (!matchesSym && !matchesNotes) return false;
        }
        return true;
      })
      .sort((a, b) => b.activity_date.localeCompare(a.activity_date));

    const elapsed = performance.now() - start;

    expect(filtered.length).toBeGreaterThan(0);
    expect(elapsed).toBeLessThan(50);
  });

  it("enriches and formats 50 holdings table rows with locale utilities in < 50ms", () => {
    const holdings = generateHoldings(50);
    expect(holdings.length).toBe(50);

    const start = performance.now();

    const enriched = holdings.map((h) => {
      const formattedMkt = fmtMoney(h.market_value, "USD");
      const formattedCost = fmtMoney(h.cost_basis, "USD");
      const formattedGain = fmtGainLoss(h.unrealized_gain, "USD");
      const formattedGainPct = fmtPercent(h.unrealized_gain_pct);
      const colorClass = gainLossClass(h.unrealized_gain);

      return {
        ...h,
        formattedMkt,
        formattedCost,
        formattedGain,
        formattedGainPct,
        colorClass,
      };
    });

    const elapsed = performance.now() - start;

    expect(enriched.length).toBe(50);
    expect(enriched[0]!.formattedMkt).toContain("$");
    expect(elapsed).toBeLessThan(50);
  });

  it("computes taxonomy category weights and target drift for 50 positions in < 50ms", () => {
    const categories = ["Equities", "Fixed Income", "Crypto", "Cash"];
    const targets: Record<string, number> = {
      Equities: 6000,
      "Fixed Income": 2500,
      Crypto: 1000,
      Cash: 500,
    };

    const positions: AllocationPosition[] = Array.from({ length: 50 }, (_, i) => ({
      assetId: `asset-${i + 1}`,
      category: categories[i % categories.length]!,
      marketValue: 1000 + (i * 250),
    }));

    const start = performance.now();

    // 1. Sum by category
    const totalPortfolioValue = positions.reduce((acc, p) => acc + p.marketValue, 0);
    const catMap = new Map<string, number>();

    for (const p of positions) {
      catMap.set(p.category, (catMap.get(p.category) ?? 0) + p.marketValue);
    }

    // 2. Compute actual bps, target bps, drift, and withinDrift
    const driftBandBps = 500; // 5%
    const allocationSummary = categories.map((catName) => {
      const catVal = catMap.get(catName) ?? 0;
      const actualBps = Math.round((catVal / totalPortfolioValue) * 10000);
      const targetBps = targets[catName] ?? 0;
      const diffBps = actualBps - targetBps;
      const withinDrift = Math.abs(diffBps) <= driftBandBps;

      return {
        category: catName,
        marketValue: catVal,
        actualBps,
        targetBps,
        diffBps,
        withinDrift,
      };
    });

    const elapsed = performance.now() - start;

    expect(allocationSummary.length).toBe(4);
    const totalBps = allocationSummary.reduce((acc, c) => acc + c.actualBps, 0);
    expect(totalBps).toBeGreaterThanOrEqual(9995);
    expect(totalBps).toBeLessThanOrEqual(10005);
    expect(elapsed).toBeLessThan(50);
  });
});
