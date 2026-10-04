/**
 * Demo Data Seeding & Workspace Orchestration Service
 *
 * Provides one-click population of rich investment research, portfolio,
 * market quotes, and taxonomy demo data. Enables instant exploration
 * without manual setup.
 *
 * @module lib/demoData
 */

import { desktopApi } from "@/lib/desktop-api";
import type {
  CreateAccountInput,
  CreateAssetInput,
  CreateActivityInput,
  UpsertQuoteInput,
  CreateTaxonomyInput,
  AssetTaxonomyAssignmentInput,
  CreateAllocationTargetInput,
} from "@/types/financial";

export interface SeedDemoResult {
  workspaceId: string;
  workspaceName: string;
  accountId: string;
  thesesCount: number;
  assetsCount: number;
  quotesCount: number;
  activitiesCount: number;
}

/**
 * Generates an array of historical daily dates in YYYY-MM-DD format ending today.
 */
function generatePastDates(days: number): string[] {
  const dates: string[] = [];
  const now = new Date();
  for (let i = days - 1; i >= 0; i--) {
    const d = new Date(now);
    d.setDate(d.getDate() - i);
    dates.push(d.toISOString().slice(0, 10));
  }
  return dates;
}

/**
 * Seeds a full demonstration workspace, portfolio, theses, and quotes.
 */
export async function seedDemoData(): Promise<SeedDemoResult> {
  // 1. Create Demo Research Workspace
  const workspaceName = "AI & Semiconductor Alpha";
  const workspace = await desktopApi.workspace.createWorkspace(workspaceName);
  const workspaceId = workspace.id;

  // 2. Create Knowledge Entities
  const semiEntity = await desktopApi.knowledgeGraph.createKnowledgeEntity(
    workspaceId,
    "industry",
    "Semiconductor & Compute",
    "Next-generation accelerated computing, advanced packaging, and EUV lithography",
  );

  const aiEntity = await desktopApi.knowledgeGraph.createKnowledgeEntity(
    workspaceId,
    "macro_theme",
    "Hyperscale AI CapEx Cycle",
    "Tier-1 Cloud service providers sustaining $200B+ annual infrastructure investments",
  );

  await desktopApi.knowledgeGraph.createKnowledgeRelationship(
    semiEntity.id,
    aiEntity.id,
    "enables",
  );

  // 3. Create Investment Theses with Evidence
  const thesis1 = await desktopApi.thesis.createThesis({
    workspaceId,
    title: "NVIDIA Blackwell B200 Rack-Scale Inflection",
    thesis: "GB200 NVL72 server rack shipments will ramp faster than consensus in 2H 2026, driving datacenter gross margins to 76% despite initial thermal packaging yields.",
    confidence: 85,
  });

  await desktopApi.thesis.addThesisEvidence(
    thesis1.id,
    "supporting",
    "Hyperscalers (Microsoft, Meta, Google) raised 2026 CapEx guidance citing sustained generative AI inference token demand.",
    "SEC 10-Q Q2 2026",
  );
  await desktopApi.thesis.addThesisEvidence(
    thesis1.id,
    "supporting",
    "TSMC CoWoS-L packaging capacity expanded by 120% YoY, clearing previous liquid-cooling manifold supply bottlenecks.",
    "TSMC Supply Chain Channel Check",
  );
  await desktopApi.thesis.addThesisEvidence(
    thesis1.id,
    "contradicting",
    "Enterprise AI ROI gestation period stretching beyond 18 months, causing potential mid-tier cloud capex pause in Q4.",
    "Gartner Enterprise AI Survey",
  );

  const thesis2 = await desktopApi.thesis.createThesis({
    workspaceId,
    title: "ASML High-NA EUV Commercial Adoption",
    thesis: "High-NA EUV lithography scanner commercial insertion accelerates at 2nm node pilot lines, giving ASML monopolistic order visibility through 2028.",
    confidence: 78,
  });

  await desktopApi.thesis.addThesisEvidence(
    thesis2.id,
    "supporting",
    "EXE:5000 High-NA scanners achieve sub-8nm resolution in Intel and TSMC pilot facilities.",
    "ASML Technology Symposium",
  );
  await desktopApi.thesis.addThesisEvidence(
    thesis2.id,
    "contradicting",
    "Extreme scanner cost ($380M/unit) prompts foundries to optimize Low-NA double patterning for initial 2nm layers.",
    "Semiconductor Digest Report",
  );

  // Link Knowledge Graph to Thesis 1
  await desktopApi.knowledgeGraph.linkThesisKnowledgeEntity(thesis1.id, semiEntity.id);
  await desktopApi.knowledgeGraph.linkThesisKnowledgeEntity(thesis1.id, aiEntity.id);

  // 4. Create Financial Account
  const accountInput: CreateAccountInput = {
    workspace_id: workspaceId,
    name: "Interactive Brokers Core Portfolio",
    account_type: "securities",
    group_name: "Prime Brokerage",
    currency: "USD",
    is_default: true,
    platform_id: null,
    account_number: "U889210-DEMO",
    tracking_mode: "transactions",
  };
  const account = await desktopApi.financial.createFinancialAccount(accountInput);
  const accountId = account.id;

  // 5. Create Financial Assets (Instruments)
  const assetsSpec: CreateAssetInput[] = [
    {
      kind: "investment",
      name: "NVIDIA Corporation",
      display_code: "NVDA",
      notes: "Core AI accelerator position",
      is_active: true,
      quote_mode: "market",
      quote_ccy: "USD",
      instrument_type: "equity",
      instrument_symbol: "NVDA",
      instrument_exchange_mic: "XNAS",
      provider_config: null,
    },
    {
      kind: "investment",
      name: "Apple Inc.",
      display_code: "AAPL",
      notes: "Edge AI consumer ecosystem",
      is_active: true,
      quote_mode: "market",
      quote_ccy: "USD",
      instrument_type: "equity",
      instrument_symbol: "AAPL",
      instrument_exchange_mic: "XNAS",
      provider_config: null,
    },
    {
      kind: "investment",
      name: "Microsoft Corporation",
      display_code: "MSFT",
      notes: "Enterprise cloud & OpenAI partner",
      is_active: true,
      quote_mode: "market",
      quote_ccy: "USD",
      instrument_type: "equity",
      instrument_symbol: "MSFT",
      instrument_exchange_mic: "XNAS",
      provider_config: null,
    },
    {
      kind: "investment",
      name: "Taiwan Semiconductor Mfg Co",
      display_code: "TSM",
      notes: "Foundry monopoly on leading-edge silicon",
      is_active: true,
      quote_mode: "market",
      quote_ccy: "USD",
      instrument_type: "equity",
      instrument_symbol: "TSM",
      instrument_exchange_mic: "XNYS",
      provider_config: null,
    },
  ];

  const createdAssets = [];
  for (const spec of assetsSpec) {
    const asset = await desktopApi.financial.createAsset(spec);
    createdAssets.push(asset);
  }

  // Link NVDA to Thesis 1
  const nvdaAsset = createdAssets.find(
    (a) => a.display_code === "NVDA" || a.instrument_symbol === "NVDA",
  );
  if (nvdaAsset) {
    await desktopApi.thesis.linkThesisAsset(thesis1.id, nvdaAsset.id);
  }

  // 6. Record Historical Transactions & Lot Activities
  const activitiesSpec: CreateActivityInput[] = [
    // Deposit cash
    {
      account_id: accountId,
      asset_id: null,
      activity_type: "deposit",
      activity_type_override: null,
      source_type: "manual",
      subtype: null,
      status: "posted",
      activity_date: "2026-08-01",
      settlement_date: "2026-08-01",
      quantity: null,
      unit_price: null,
      amount: "150000.00",
      fee: "0.00",
      tax: "0.00",
      currency: "USD",
      fx_rate: "1.0",
      notes: "Initial portfolio seed capital",
      metadata: null,
      source_system: "demo_seed",
      source_record_id: "demo-dep-1",
      source_group_id: null,
      idempotency_key: "demo-dep-1",
      import_run_id: null,
    },
    // Buy NVDA
    {
      account_id: accountId,
      asset_id: createdAssets[0]!.id,
      activity_type: "buy",
      activity_type_override: null,
      source_type: "manual",
      subtype: null,
      status: "posted",
      activity_date: "2026-08-10",
      settlement_date: "2026-08-12",
      quantity: "350",
      unit_price: "118.50",
      amount: "41475.00",
      fee: "2.00",
      tax: "0.00",
      currency: "USD",
      fx_rate: "1.0",
      notes: "Initiated NVDA core position",
      metadata: null,
      source_system: "demo_seed",
      source_record_id: "demo-buy-nvda",
      source_group_id: null,
      idempotency_key: "demo-buy-nvda",
      import_run_id: null,
    },
    // Buy MSFT
    {
      account_id: accountId,
      asset_id: createdAssets[2]!.id,
      activity_type: "buy",
      activity_type_override: null,
      source_type: "manual",
      subtype: null,
      status: "posted",
      activity_date: "2026-08-15",
      settlement_date: "2026-08-17",
      quantity: "80",
      unit_price: "420.00",
      amount: "33600.00",
      fee: "2.00",
      tax: "0.00",
      currency: "USD",
      fx_rate: "1.0",
      notes: "Cloud SaaS anchor",
      metadata: null,
      source_system: "demo_seed",
      source_record_id: "demo-buy-msft",
      source_group_id: null,
      idempotency_key: "demo-buy-msft",
      import_run_id: null,
    },
    // Buy TSM
    {
      account_id: accountId,
      asset_id: createdAssets[3]!.id,
      activity_type: "buy",
      activity_type_override: null,
      source_type: "manual",
      subtype: null,
      status: "posted",
      activity_date: "2026-08-20",
      settlement_date: "2026-08-22",
      quantity: "150",
      unit_price: "165.00",
      amount: "24750.00",
      fee: "2.00",
      tax: "0.00",
      currency: "USD",
      fx_rate: "1.0",
      notes: "Advanced packaging proxy",
      metadata: null,
      source_system: "demo_seed",
      source_record_id: "demo-buy-tsm",
      source_group_id: null,
      idempotency_key: "demo-buy-tsm",
      import_run_id: null,
    },
  ];

  for (const act of activitiesSpec) {
    await desktopApi.financial.createActivity(act);
  }

  // 7. Seed 30 Days of Historical Price Quotes for Assets
  // This enables Market Quotes charts, SMA 20, OHLC, and timeframe cuts.
  const pastDates = generatePastDates(30);
  let totalQuotesCount = 0;

  const basePrices: Record<string, number> = {
    NVDA: 118.5,
    AAPL: 220.0,
    MSFT: 420.0,
    TSM: 165.0,
  };

  for (const asset of createdAssets) {
    const symbol = asset.display_code || asset.instrument_symbol || "NVDA";
    let curPrice = basePrices[symbol] || 100.0;

    for (let i = 0; i < pastDates.length; i++) {
      const date = pastDates[i]!;
      // Small realistic drift with volatility
      const changePct = ((i % 5) - 2) * 0.015 + 0.003;
      curPrice = Math.round(curPrice * (1 + changePct) * 100) / 100;
      const open = Math.round((curPrice - 0.8) * 100) / 100;
      const high = Math.round((curPrice + 2.1) * 100) / 100;
      const low = Math.round((curPrice - 1.4) * 100) / 100;
      const volume = String(15000000 + Math.floor(Math.sin(i) * 5000000));

      const quoteInput: UpsertQuoteInput = {
        asset_id: asset.id,
        day: date,
        source: "DEMO_FEED",
        open: open.toFixed(2),
        high: high.toFixed(2),
        low: low.toFixed(2),
        close: curPrice.toFixed(2),
        adjclose: curPrice.toFixed(2),
        volume,
        currency: "USD",
        notes: "Historical simulation point",
      };

      await desktopApi.financial.upsertQuote(quoteInput);
      totalQuotesCount++;
    }
  }

  // 8. Seed Taxonomies & Allocation Targets (Category 11)
  const taxonomyInput: CreateTaxonomyInput = {
    name: "Asset Class",
    color: "#2563eb",
    description: "Macro asset allocation classes",
    is_system: false,
    is_single_select: true,
    sort_order: 10,
  };
  const taxonomy = await desktopApi.financial.createTaxonomy(taxonomyInput);

  const equityCat = await desktopApi.financial.createTaxonomyCategory({
    taxonomy_id: taxonomy.id,
    parent_id: null,
    name: "Equities",
    key: "equities",
    color: "#3b82f6",
    description: "Public stock positions",
    sort_order: 10,
  });

  const cashCat = await desktopApi.financial.createTaxonomyCategory({
    taxonomy_id: taxonomy.id,
    parent_id: null,
    name: "Cash & Equivalents",
    key: "cash",
    color: "#10b981",
    description: "Liquid reserve funds",
    sort_order: 20,
  });

  // Assign assets to category
  for (const asset of createdAssets) {
    const assignment: AssetTaxonomyAssignmentInput = {
      asset_id: asset.id,
      taxonomy_id: taxonomy.id,
      category_id: equityCat.id,
      weight: 1.0,
      source: "manual",
    };
    await desktopApi.financial.assignAssetToTaxonomyCategory(assignment);
  }

  // Create Allocation Target Policy
  const targetInput: CreateAllocationTargetInput = {
    name: "Core 70/30 Balanced Model",
    scope_type: "account",
    scope_id: accountId,
    taxonomy_id: taxonomy.id,
    trigger_type: "drift_band",
    drift_band_bps: 500, // 5% tolerance band
    rebalance_goal: "full",
    min_trade_amount: "500.00",
    whole_shares_only: false,
    allow_sells: true,
    max_turnover_bps: 1500,
  };
  const targetPolicy = await desktopApi.financial.createAllocationTarget(targetInput);

  // Target weights: 70% Equities / 30% Cash
  await desktopApi.financial.addAllocationWeight({
    target_id: targetPolicy.id,
    taxonomy_id: taxonomy.id,
    category_id: equityCat.id,
    target_bps: 7000,
    is_locked: false,
    is_required: true,
  });

  await desktopApi.financial.addAllocationWeight({
    target_id: targetPolicy.id,
    taxonomy_id: taxonomy.id,
    category_id: cashCat.id,
    target_bps: 3000,
    is_locked: false,
    is_required: true,
  });

  // Persist valuation for today so holdings summaries and allocation compute immediately
  const today = new Date().toISOString().slice(0, 10);
  try {
    await desktopApi.financial.calculateValuationDay(accountId, today);
  } catch {
    // Best-effort calculation
  }

  return {
    workspaceId,
    workspaceName,
    accountId,
    thesesCount: 2,
    assetsCount: createdAssets.length,
    quotesCount: totalQuotesCount,
    activitiesCount: activitiesSpec.length,
  };
}
