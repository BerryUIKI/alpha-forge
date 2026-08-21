import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  getHoldings,
  getAllHoldings,
  recordSell,
  getOpenLots,
  getOpenLotsForAccount,
  calculateValuationDay,
  getValuation,
  getValuationSeries,
  calculateAllValuations,
  computePerformanceSummary,
  getPerformanceTimeSeries,
  getAllocation,
  checkAllocationConstraints,
  createSnapshot,
  getSnapshot,
  listSnapshots,
  deleteSnapshot,
  computeNetWorth,
  createPlatform,
  listPlatforms,
  getPlatform,
  createFinancialAccount,
  listFinancialAccounts,
  listAllFinancialAccounts,
  getFinancialAccount,
  archiveFinancialAccount,
  createAsset,
  getAsset,
  findAssetByInstrumentKey,
  listActiveAssets,
  upsertQuote,
  getQuoteForDay,
  listQuotesForAsset,
  createActivity,
  getActivity,
  listActivitiesByAccount,
  listActivitiesByAsset,
  createImportRun,
  listImportRuns,
  createLot,
  getLot,
  upsertValuation,
  listValuationsByAccount,
  deleteValuationForDate,
  createTaxonomy,
  getTaxonomy,
  listTaxonomies,
  createTaxonomyCategory,
  listTaxonomyCategories,
  assignAssetToTaxonomyCategory,
  listAssignmentsForAsset,
  listAssignmentsByTaxonomy,
  removeTaxonomyAssignment,
  createAllocationTarget,
  getAllocationTarget,
  listAllocationTargets,
  archiveAllocationTarget,
  addAllocationWeight,
  listAllocationWeights,
  addAllocationConstraint,
  listAllocationConstraints,
} from "./financial";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
const mockInvoke = vi.mocked(invoke);

const mockHoldingSummary = {
  account_id: "account-1",
  as_of_date: "2026-08-17",
  total_market_value: "100",
  total_cost_basis: "80",
  total_unrealized_gain: "20",
  total_realized_gain: "0",
  total_market_value_base: "100",
  total_cost_basis_base: "80",
  total_unrealized_gain_base: "20",
  total_realized_gain_base: "0",
  holdings: [],
  cash_balance: "50",
  cash_balance_base: "50",
};

const mockFifoReduction = {
  account_id: "account-1",
  asset_id: "asset-1",
  disposal_date: "2026-08-17",
  total_quantity: "10",
  total_proceeds: "100",
  total_cost_basis: "80",
  total_realized_pnl: "20",
  total_proceeds_base: "100",
  total_cost_basis_base: "80",
  total_realized_pnl_base: "20",
  lots_consumed: 1,
  lots_partially_consumed: 0,
};

const mockValuation = {
  id: "val-1",
  account_id: "account-1",
  valuation_date: "2026-08-17",
  total_value: "100",
  cash_balance: "50",
  investment_value: "50",
  currency: "USD",
  fx_rate: "1",
  total_value_base: "100",
  status: "calculated" as const,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockPerfSummary = {
  account_id: "account-1",
  start_date: "2026-01-01",
  end_date: "2026-08-17",
  total_return_pct: "10",
  xirr_pct: "12",
  twr_pct: "11",
  start_value: "1000",
  end_value: "1100",
  net_contribution: "0",
  total_gain: "100",
  total_gain_base: "100",
  data_quality: "good",
};

const mockAllocation = {
  scope_type: "account" as const,
  scope_id: "account-1",
  total_market_value: "1000",
  total_market_value_base: "1000",
  categories: [],
  unassigned_market_value: "0",
  unassigned_market_value_base: "0",
};

const mockSnapshot = {
  id: "snap-1",
  account_id: "account-1",
  snapshot_date: "2026-08-17",
  label: "snapshot",
  source: "calculated" as const,
  total_value: "1000",
  total_value_base: "1000",
  cash_balance: "100",
  cash_balance_base: "100",
  positions: [],
  notes: null,
  created_at: "2026-08-17T00:00:00Z",
};

const mockNetWorth = {
  as_of_date: "2026-08-17",
  base_currency: "USD",
  total_assets: "1000",
  total_liabilities: "0",
  net_worth: "1000",
  accounts: [],
};

const mockPlatform = {
  id: "plat-1",
  name: "IBKR",
  url: "https://ibkr.com",
  external_id: null,
  kind: "broker",
  website_url: null,
  logo_url: null,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockAccount = {
  id: "acc-1",
  workspace_id: "ws-1",
  name: "Main",
  account_type: "securities" as const,
  group_name: null,
  currency: "USD",
  is_default: true,
  is_active: true,
  platform_id: null,
  account_number: null,
  meta: null,
  provider: null,
  provider_account_id: null,
  is_archived: false,
  tracking_mode: "transactions" as const,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockAsset = {
  id: "asset-1",
  kind: "investment" as const,
  name: "Apple",
  display_code: "AAPL",
  notes: null,
  metadata: null,
  is_active: true,
  quote_mode: "market" as const,
  quote_ccy: "USD",
  instrument_type: "equity" as const,
  instrument_symbol: "AAPL",
  instrument_exchange_mic: "XNAS",
  instrument_key: "AAPL.XNAS",
  provider_config: null,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockQuote = {
  id: "q-1",
  asset_id: "asset-1",
  day: "2026-08-17",
  source: "market",
  open: "150",
  high: "155",
  low: "149",
  close: "154",
  adjclose: "154",
  volume: "1000000",
  currency: "USD",
  notes: null,
  created_at: "2026-08-17T00:00:00Z",
  timestamp: "2026-08-17T00:00:00Z",
};

const mockActivity = {
  id: "act-1",
  account_id: "acc-1",
  asset_id: "asset-1",
  activity_type: "buy" as const,
  activity_type_override: null,
  source_type: null,
  subtype: null,
  status: "posted" as const,
  activity_date: "2026-08-17",
  settlement_date: null,
  quantity: "10",
  unit_price: "150",
  amount: "1500",
  fee: "0",
  tax: "0",
  currency: "USD",
  fx_rate: "1",
  notes: null,
  metadata: null,
  source_system: null,
  source_record_id: null,
  source_group_id: null,
  idempotency_key: null,
  import_run_id: null,
  is_user_modified: false,
  needs_review: false,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockImportRun = {
  id: "imp-1",
  account_id: "acc-1",
  source_system: "csv",
  run_type: "import",
  mode: "full",
  status: "completed",
  started_at: "2026-08-17T00:00:00Z",
  finished_at: "2026-08-17T00:01:00Z",
  review_mode: "none",
  applied_at: "2026-08-17T00:01:00Z",
  checkpoint_in: null,
  checkpoint_out: null,
  summary: null,
  warnings: null,
  error: null,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:01:00Z",
};

const mockLot = {
  id: "lot-1",
  account_id: "acc-1",
  asset_id: "asset-1",
  open_date: "2026-08-17",
  open_activity_id: "act-1",
  original_quantity: "10",
  remaining_quantity: "10",
  open_price: "150",
  cost_basis: "1500",
  cost_basis_base: "1500",
  currency: "USD",
  fx_rate: "1",
  realized_gain: "0",
  realized_gain_base: "0",
  status: "filled" as const,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockTaxonomy = {
  id: "tax-1",
  name: "Asset Class",
  color: "#10b981",
  description: null,
  is_system: true,
  is_single_select: true,
  sort_order: 1,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockTaxonomyCategory = {
  id: "cat-1",
  taxonomy_id: "tax-1",
  parent_id: null,
  name: "Equities",
  key: "EQ",
  color: "#10b981",
  description: null,
  sort_order: 1,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockTaxonomyAssignment = {
  id: "asgn-1",
  asset_id: "asset-1",
  taxonomy_id: "tax-1",
  category_id: "cat-1",
  weight: 10000,
  source: "manual",
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockAllocationTarget = {
  id: "tgt-1",
  name: "Balanced",
  description: null,
  scope_type: "portfolio" as const,
  scope_id: null,
  taxonomy_id: "tax-1",
  trigger_type: "drift",
  drift_band_bps: 500,
  rebalance_goal: "target",
  min_trade_amount: "100",
  whole_shares_only: true,
  allow_sells: true,
  max_turnover_bps: null,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
  archived_at: null,
};

const mockAllocationWeight = {
  id: "w-1",
  target_id: "tgt-1",
  taxonomy_id: "tax-1",
  category_id: "cat-1",
  target_bps: 6000,
  is_locked: false,
  is_required: true,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

const mockAllocationConstraint = {
  id: "c-1",
  target_id: "tgt-1",
  subject_type: "category",
  subject_id: "cat-1",
  action: "buy",
  effect: "allow",
  reason: null,
  metadata_json: null,
  created_at: "2026-08-17T00:00:00Z",
  updated_at: "2026-08-17T00:00:00Z",
};

describe("financial API — Phase 2 services", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("getHoldings calls get_holdings", async () => {
    mockInvoke.mockResolvedValue(mockHoldingSummary);
    const res = await getHoldings("account-1", "2026-08-17");
    expect(res.account_id).toBe("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_holdings", {
      accountId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("getAllHoldings calls get_all_holdings", async () => {
    mockInvoke.mockResolvedValue([mockHoldingSummary]);
    const res = await getAllHoldings("2026-08-17");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_all_holdings", {
      asOfDate: "2026-08-17",
    });
  });

  it("recordSell calls record_sell", async () => {
    mockInvoke.mockResolvedValue(mockFifoReduction);
    const res = await recordSell("account-1", "asset-1", "activity-1");
    expect(res.account_id).toBe("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("record_sell", {
      accountId: "account-1",
      assetId: "asset-1",
      activityId: "activity-1",
    });
  });

  it("getOpenLots calls get_open_lots", async () => {
    mockInvoke.mockResolvedValue([mockLot]);
    const res = await getOpenLots("account-1", "asset-1");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_open_lots", {
      accountId: "account-1",
      assetId: "asset-1",
    });
  });

  it("getOpenLotsForAccount calls get_open_lots_for_account", async () => {
    mockInvoke.mockResolvedValue([mockLot]);
    const res = await getOpenLotsForAccount("account-1");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_open_lots_for_account", {
      accountId: "account-1",
    });
  });

  it("calculateValuationDay calls calculate_valuation_day", async () => {
    mockInvoke.mockResolvedValue(mockValuation);
    const res = await calculateValuationDay("account-1", "2026-08-17");
    expect(res.id).toBe("val-1");
    expect(mockInvoke).toHaveBeenCalledWith("calculate_valuation_day", {
      accountId: "account-1",
      date: "2026-08-17",
    });
  });

  it("getValuation calls get_valuation", async () => {
    mockInvoke.mockResolvedValue(null);
    const res = await getValuation("account-1", "2026-08-17");
    expect(res).toBeNull();
    expect(mockInvoke).toHaveBeenCalledWith("get_valuation", {
      accountId: "account-1",
      date: "2026-08-17",
    });
  });

  it("getValuationSeries calls get_valuation_series", async () => {
    mockInvoke.mockResolvedValue([mockValuation]);
    const res = await getValuationSeries("account-1");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_valuation_series", {
      accountId: "account-1",
    });
  });

  it("calculateAllValuations calls calculate_all_valuations", async () => {
    mockInvoke.mockResolvedValue([mockValuation]);
    const res = await calculateAllValuations("2026-08-17");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("calculate_all_valuations", {
      date: "2026-08-17",
    });
  });

  it("computePerformanceSummary calls compute_performance_summary", async () => {
    mockInvoke.mockResolvedValue(mockPerfSummary);
    const res = await computePerformanceSummary(
      "account-1",
      "2026-01-01",
      "2026-08-17",
    );
    expect(res.account_id).toBe("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("compute_performance_summary", {
      accountId: "account-1",
      startDate: "2026-01-01",
      endDate: "2026-08-17",
    });
  });

  it("getPerformanceTimeSeries calls get_performance_time_series", async () => {
    mockInvoke.mockResolvedValue([
      {
        date: "2026-08-17",
        total_value: "1000",
        total_value_base: "1000",
        net_contribution: "0",
        net_contribution_base: "0",
        cumulative_return_pct: "10",
        daily_return_pct: "1",
      },
    ]);
    const res = await getPerformanceTimeSeries("account-1");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_performance_time_series", {
      accountId: "account-1",
    });
  });

  it("getAllocation calls get_allocation", async () => {
    mockInvoke.mockResolvedValue(mockAllocation);
    const res = await getAllocation("account", "account-1", "2026-08-17");
    expect(res.scope_type).toBe("account");
    expect(mockInvoke).toHaveBeenCalledWith("get_allocation", {
      scopeType: "account",
      scopeId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("checkAllocationConstraints calls check_allocation_constraints", async () => {
    mockInvoke.mockResolvedValue(["Constraint violated"]);
    const res = await checkAllocationConstraints(
      "account",
      "account-1",
      "2026-08-17",
    );
    expect(res).toEqual(["Constraint violated"]);
    expect(mockInvoke).toHaveBeenCalledWith("check_allocation_constraints", {
      scopeType: "account",
      scopeId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("createSnapshot calls create_snapshot", async () => {
    mockInvoke.mockResolvedValue(mockSnapshot);
    const res = await createSnapshot("account-1", "2026-08-17", "snapshot");
    expect(res.id).toBe("snap-1");
    expect(mockInvoke).toHaveBeenCalledWith("create_snapshot", {
      accountId: "account-1",
      snapshotDate: "2026-08-17",
      label: "snapshot",
    });
  });

  it("getSnapshot calls get_snapshot", async () => {
    mockInvoke.mockResolvedValue(null);
    const res = await getSnapshot("snapshot-1");
    expect(res).toBeNull();
    expect(mockInvoke).toHaveBeenCalledWith("get_snapshot", {
      id: "snapshot-1",
    });
  });

  it("listSnapshots calls list_snapshots", async () => {
    mockInvoke.mockResolvedValue([mockSnapshot]);
    const res = await listSnapshots("account-1");
    expect(res).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("list_snapshots", {
      accountId: "account-1",
    });
  });

  it("deleteSnapshot calls delete_snapshot", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await deleteSnapshot("snapshot-1");
    expect(mockInvoke).toHaveBeenCalledWith("delete_snapshot", {
      id: "snapshot-1",
    });
  });

  it("computeNetWorth calls compute_net_worth", async () => {
    mockInvoke.mockResolvedValue(mockNetWorth);
    const res = await computeNetWorth("2026-08-17", "USD");
    expect(res.as_of_date).toBe("2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("compute_net_worth", {
      asOfDate: "2026-08-17",
      baseCurrency: "USD",
    });
  });
});

describe("financial API — Phase 3.5 CRUD services", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("createPlatform calls create_platform", async () => {
    mockInvoke.mockResolvedValue(mockPlatform);
    const res = await createPlatform({
      name: "IBKR",
      url: "https://ibkr.com",
      kind: "broker",
    });
    expect(res.name).toBe("IBKR");
  });

  it("listPlatforms calls list_platforms", async () => {
    mockInvoke.mockResolvedValue([mockPlatform]);
    const res = await listPlatforms();
    expect(res).toHaveLength(1);
  });

  it("getPlatform calls get_platform", async () => {
    mockInvoke.mockResolvedValue(mockPlatform);
    const res = await getPlatform("plat-1");
    expect(res?.id).toBe("plat-1");
  });

  it("createFinancialAccount calls create_financial_account", async () => {
    mockInvoke.mockResolvedValue(mockAccount);
    const res = await createFinancialAccount({
      workspace_id: "ws-1",
      name: "Main",
      account_type: "securities",
      group_name: null,
      currency: "USD",
      is_default: true,
      platform_id: null,
      account_number: null,
      tracking_mode: "transactions",
    });
    expect(res.id).toBe("acc-1");
  });

  it("listFinancialAccounts calls list_financial_accounts", async () => {
    mockInvoke.mockResolvedValue([mockAccount]);
    const res = await listFinancialAccounts("ws-1");
    expect(res).toHaveLength(1);
  });

  it("listAllFinancialAccounts calls list_all_financial_accounts", async () => {
    mockInvoke.mockResolvedValue([mockAccount]);
    const res = await listAllFinancialAccounts();
    expect(res).toHaveLength(1);
  });

  it("getFinancialAccount calls get_financial_account", async () => {
    mockInvoke.mockResolvedValue(mockAccount);
    const res = await getFinancialAccount("acc-1");
    expect(res?.id).toBe("acc-1");
  });

  it("archiveFinancialAccount calls archive_financial_account", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await archiveFinancialAccount("acc-1");
    expect(mockInvoke).toHaveBeenCalledWith("archive_financial_account", {
      id: "acc-1",
    });
  });

  it("createAsset calls create_asset", async () => {
    mockInvoke.mockResolvedValue(mockAsset);
    const res = await createAsset({
      kind: "investment",
      name: "Apple",
      display_code: "AAPL",
      notes: null,
      is_active: true,
      quote_mode: "market",
      quote_ccy: "USD",
      instrument_type: "equity",
      instrument_symbol: "AAPL",
      instrument_exchange_mic: "XNAS",
      provider_config: null,
    });
    expect(res.id).toBe("asset-1");
  });

  it("getAsset calls get_asset", async () => {
    mockInvoke.mockResolvedValue(mockAsset);
    const res = await getAsset("asset-1");
    expect(res?.id).toBe("asset-1");
  });

  it("findAssetByInstrumentKey calls find_asset_by_instrument_key", async () => {
    mockInvoke.mockResolvedValue(mockAsset);
    const res = await findAssetByInstrumentKey("AAPL.XNAS");
    expect(res?.id).toBe("asset-1");
  });

  it("listActiveAssets calls list_active_assets", async () => {
    mockInvoke.mockResolvedValue([mockAsset]);
    const res = await listActiveAssets();
    expect(res).toHaveLength(1);
  });

  it("upsertQuote calls upsert_quote", async () => {
    mockInvoke.mockResolvedValue(mockQuote);
    const res = await upsertQuote({
      asset_id: "asset-1",
      day: "2026-08-17",
      source: "market",
      open: "150",
      high: "155",
      low: "149",
      close: "154",
      adjclose: "154",
      volume: "1000000",
      currency: "USD",
      notes: null,
    });
    expect(res.id).toBe("q-1");
  });

  it("getQuoteForDay calls get_quote_for_day", async () => {
    mockInvoke.mockResolvedValue(mockQuote);
    const res = await getQuoteForDay("asset-1", "2026-08-17", "market");
    expect(res?.id).toBe("q-1");
  });

  it("listQuotesForAsset calls list_quotes_for_asset", async () => {
    mockInvoke.mockResolvedValue([mockQuote]);
    const res = await listQuotesForAsset("asset-1");
    expect(res).toHaveLength(1);
  });

  it("createActivity calls create_activity", async () => {
    mockInvoke.mockResolvedValue(mockActivity);
    const res = await createActivity({
      account_id: "acc-1",
      asset_id: "asset-1",
      activity_type: "buy",
      activity_type_override: null,
      source_type: null,
      subtype: null,
      status: "posted",
      activity_date: "2026-08-17",
      settlement_date: null,
      quantity: "10",
      unit_price: "150",
      amount: "1500",
      fee: "0",
      tax: "0",
      currency: "USD",
      fx_rate: "1",
      notes: null,
      metadata: null,
      source_system: null,
      source_record_id: null,
      source_group_id: null,
      idempotency_key: null,
      import_run_id: null,
    });
    expect(res.id).toBe("act-1");
  });

  it("getActivity calls get_activity", async () => {
    mockInvoke.mockResolvedValue(mockActivity);
    const res = await getActivity("act-1");
    expect(res?.id).toBe("act-1");
  });

  it("listActivitiesByAccount calls list_activities_by_account", async () => {
    mockInvoke.mockResolvedValue([mockActivity]);
    const res = await listActivitiesByAccount("acc-1");
    expect(res).toHaveLength(1);
  });

  it("listActivitiesByAsset calls list_activities_by_asset", async () => {
    mockInvoke.mockResolvedValue([mockActivity]);
    const res = await listActivitiesByAsset("asset-1");
    expect(res).toHaveLength(1);
  });

  it("createImportRun calls create_import_run", async () => {
    mockInvoke.mockResolvedValue(mockImportRun);
    const res = await createImportRun({
      account_id: "acc-1",
      source_system: "csv",
      run_type: "import",
      mode: "full",
      status: "completed",
      review_mode: "none",
    });
    expect(res.id).toBe("imp-1");
  });

  it("listImportRuns calls list_import_runs", async () => {
    mockInvoke.mockResolvedValue([mockImportRun]);
    const res = await listImportRuns("acc-1");
    expect(res).toHaveLength(1);
  });

  it("createLot calls create_lot", async () => {
    mockInvoke.mockResolvedValue(mockLot);
    const res = await createLot({
      account_id: "acc-1",
      asset_id: "asset-1",
      open_date: "2026-08-17",
      open_activity_id: null,
      original_quantity: "10",
      cost_per_unit: "150",
      original_cost_basis: "1500",
      fee_allocated: "0",
      currency: "USD",
      base_currency: "USD",
      fx_rate_to_base: "1",
      fx_rate_to_account: null,
      account_currency: null,
      cost_basis_method: "fifo",
    });
    expect(res.id).toBe("lot-1");
  });

  it("getLot calls get_lot", async () => {
    mockInvoke.mockResolvedValue(mockLot);
    const res = await getLot("lot-1");
    expect(res?.id).toBe("lot-1");
  });

  it("upsertValuation calls upsert_valuation", async () => {
    mockInvoke.mockResolvedValue(mockValuation);
    const res = await upsertValuation({
      account_id: "acc-1",
      valuation_date: "2026-08-17",
      account_currency: "USD",
      base_currency: "USD",
      fx_rate_to_base: "1",
      cash_balance: "50",
      investment_market_value: "50",
      total_value: "100",
      cost_basis: "80",
      net_contribution: "0",
      cash_balance_base: "50",
      investment_market_value_base: "50",
      total_value_base: "100",
      cost_basis_base: "80",
      net_contribution_base: "0",
      external_inflow_base: "0",
      external_outflow_base: "0",
      performance_eligible_value_base: "100",
      external_flow_source: "no_flow",
      value_status: "calculated",
      basis_status: "filled",
    });
    expect(res.id).toBe("val-1");
  });

  it("listValuationsByAccount calls list_valuations_by_account", async () => {
    mockInvoke.mockResolvedValue([mockValuation]);
    const res = await listValuationsByAccount("acc-1");
    expect(res).toHaveLength(1);
  });

  it("deleteValuationForDate calls delete_valuation_for_date", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await deleteValuationForDate("acc-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("delete_valuation_for_date", {
      accountId: "acc-1",
      date: "2026-08-17",
    });
  });

  it("createTaxonomy calls create_taxonomy", async () => {
    mockInvoke.mockResolvedValue(mockTaxonomy);
    const res = await createTaxonomy({
      name: "Asset Class",
      color: "#10b981",
      description: null,
      is_system: true,
      is_single_select: true,
      sort_order: 1,
    });
    expect(res.id).toBe("tax-1");
  });

  it("getTaxonomy calls get_taxonomy", async () => {
    mockInvoke.mockResolvedValue(mockTaxonomy);
    const res = await getTaxonomy("tax-1");
    expect(res?.id).toBe("tax-1");
  });

  it("listTaxonomies calls list_taxonomies", async () => {
    mockInvoke.mockResolvedValue([mockTaxonomy]);
    const res = await listTaxonomies();
    expect(res).toHaveLength(1);
  });

  it("createTaxonomyCategory calls create_taxonomy_category", async () => {
    mockInvoke.mockResolvedValue(mockTaxonomyCategory);
    const res = await createTaxonomyCategory({
      taxonomy_id: "tax-1",
      parent_id: null,
      name: "Equities",
      key: "EQ",
      color: "#10b981",
      description: null,
      sort_order: 1,
    });
    expect(res.id).toBe("cat-1");
  });

  it("listTaxonomyCategories calls list_taxonomy_categories", async () => {
    mockInvoke.mockResolvedValue([mockTaxonomyCategory]);
    const res = await listTaxonomyCategories("tax-1");
    expect(res).toHaveLength(1);
  });

  it("assignAssetToTaxonomyCategory calls assign_asset_to_taxonomy_category", async () => {
    mockInvoke.mockResolvedValue(mockTaxonomyAssignment);
    const res = await assignAssetToTaxonomyCategory({
      asset_id: "asset-1",
      taxonomy_id: "tax-1",
      category_id: "cat-1",
      weight: 10000,
      source: "manual",
    });
    expect(res.id).toBe("asgn-1");
  });

  it("listAssignmentsForAsset calls list_assignments_for_asset", async () => {
    mockInvoke.mockResolvedValue([mockTaxonomyAssignment]);
    const res = await listAssignmentsForAsset("asset-1");
    expect(res).toHaveLength(1);
  });

  it("listAssignmentsByTaxonomy calls list_assignments_by_taxonomy", async () => {
    mockInvoke.mockResolvedValue([mockTaxonomyAssignment]);
    const res = await listAssignmentsByTaxonomy("tax-1");
    expect(res).toHaveLength(1);
  });

  it("removeTaxonomyAssignment calls remove_taxonomy_assignment", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await removeTaxonomyAssignment("asgn-1");
    expect(mockInvoke).toHaveBeenCalledWith("remove_taxonomy_assignment", {
      id: "asgn-1",
    });
  });

  it("createAllocationTarget calls create_allocation_target", async () => {
    mockInvoke.mockResolvedValue(mockAllocationTarget);
    const res = await createAllocationTarget({
      name: "Balanced",
      scope_type: "portfolio",
      scope_id: null,
      taxonomy_id: "tax-1",
      trigger_type: "drift",
      drift_band_bps: 500,
      rebalance_goal: "target",
      min_trade_amount: "100",
      whole_shares_only: true,
      allow_sells: true,
      max_turnover_bps: null,
    });
    expect(res.id).toBe("tgt-1");
  });

  it("getAllocationTarget calls get_allocation_target", async () => {
    mockInvoke.mockResolvedValue(mockAllocationTarget);
    const res = await getAllocationTarget("tgt-1");
    expect(res?.id).toBe("tgt-1");
  });

  it("listAllocationTargets calls list_allocation_targets", async () => {
    mockInvoke.mockResolvedValue([mockAllocationTarget]);
    const res = await listAllocationTargets(false);
    expect(res).toHaveLength(1);
  });

  it("archiveAllocationTarget calls archive_allocation_target", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await archiveAllocationTarget("tgt-1");
    expect(mockInvoke).toHaveBeenCalledWith("archive_allocation_target", {
      id: "tgt-1",
    });
  });

  it("addAllocationWeight calls add_allocation_weight", async () => {
    mockInvoke.mockResolvedValue(mockAllocationWeight);
    const res = await addAllocationWeight({
      target_id: "tgt-1",
      taxonomy_id: "tax-1",
      category_id: "cat-1",
      target_bps: 6000,
      is_locked: false,
      is_required: true,
    });
    expect(res.id).toBe("w-1");
  });

  it("listAllocationWeights calls list_allocation_weights", async () => {
    mockInvoke.mockResolvedValue([mockAllocationWeight]);
    const res = await listAllocationWeights("tgt-1");
    expect(res).toHaveLength(1);
  });

  it("addAllocationConstraint calls add_allocation_constraint", async () => {
    mockInvoke.mockResolvedValue(mockAllocationConstraint);
    const res = await addAllocationConstraint({
      target_id: "tgt-1",
      subject_type: "category",
      subject_id: "cat-1",
      action: "buy",
      effect: "allow",
      reason: null,
      metadata_json: null,
    });
    expect(res.id).toBe("c-1");
  });

  it("listAllocationConstraints calls list_allocation_constraints", async () => {
    mockInvoke.mockResolvedValue([mockAllocationConstraint]);
    const res = await listAllocationConstraints("tgt-1");
    expect(res).toHaveLength(1);
  });

  it("rejects malformed responses with Zod schema validation", async () => {
    mockInvoke.mockResolvedValue({
      id: "acc-1",
      // missing required fields
    });
    await expect(
      createFinancialAccount({
        workspace_id: null,
        name: "Test",
        account_type: "securities",
        group_name: null,
        currency: "USD",
        is_default: true,
        platform_id: null,
        account_number: null,
        tracking_mode: "transactions",
      }),
    ).rejects.toThrow();
  });
});