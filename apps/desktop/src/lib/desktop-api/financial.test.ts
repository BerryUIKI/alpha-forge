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

describe("financial API — Phase 2 services", () => {
  beforeEach(() => mockInvoke.mockReset());

  it("getHoldings calls get_holdings", async () => {
    mockInvoke.mockResolvedValue({});
    await getHoldings("account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("get_holdings", {
      accountId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("getAllHoldings calls get_all_holdings", async () => {
    mockInvoke.mockResolvedValue([]);
    await getAllHoldings("2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("get_all_holdings", {
      asOfDate: "2026-08-17",
    });
  });

  it("recordSell calls record_sell", async () => {
    mockInvoke.mockResolvedValue({});
    await recordSell("account-1", "asset-1", "activity-1");
    expect(mockInvoke).toHaveBeenCalledWith("record_sell", {
      accountId: "account-1",
      assetId: "asset-1",
      activityId: "activity-1",
    });
  });

  it("getOpenLots calls get_open_lots", async () => {
    mockInvoke.mockResolvedValue([]);
    await getOpenLots("account-1", "asset-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_open_lots", {
      accountId: "account-1",
      assetId: "asset-1",
    });
  });

  it("getOpenLotsForAccount calls get_open_lots_for_account", async () => {
    mockInvoke.mockResolvedValue([]);
    await getOpenLotsForAccount("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_open_lots_for_account", {
      accountId: "account-1",
    });
  });

  it("calculateValuationDay calls calculate_valuation_day", async () => {
    mockInvoke.mockResolvedValue({});
    await calculateValuationDay("account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("calculate_valuation_day", {
      accountId: "account-1",
      date: "2026-08-17",
    });
  });

  it("getValuation calls get_valuation", async () => {
    mockInvoke.mockResolvedValue(null);
    await getValuation("account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("get_valuation", {
      accountId: "account-1",
      date: "2026-08-17",
    });
  });

  it("getValuationSeries calls get_valuation_series", async () => {
    mockInvoke.mockResolvedValue([]);
    await getValuationSeries("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_valuation_series", {
      accountId: "account-1",
    });
  });

  it("calculateAllValuations calls calculate_all_valuations", async () => {
    mockInvoke.mockResolvedValue([]);
    await calculateAllValuations("2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("calculate_all_valuations", {
      date: "2026-08-17",
    });
  });

  it("computePerformanceSummary calls compute_performance_summary", async () => {
    mockInvoke.mockResolvedValue({});
    await computePerformanceSummary("account-1", "2026-01-01", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("compute_performance_summary", {
      accountId: "account-1",
      startDate: "2026-01-01",
      endDate: "2026-08-17",
    });
  });

  it("getPerformanceTimeSeries calls get_performance_time_series", async () => {
    mockInvoke.mockResolvedValue([]);
    await getPerformanceTimeSeries("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_performance_time_series", {
      accountId: "account-1",
    });
  });

  it("getAllocation calls get_allocation", async () => {
    mockInvoke.mockResolvedValue({});
    await getAllocation("account", "account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("get_allocation", {
      scopeType: "account",
      scopeId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("checkAllocationConstraints calls check_allocation_constraints", async () => {
    mockInvoke.mockResolvedValue([]);
    await checkAllocationConstraints("account", "account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("check_allocation_constraints", {
      scopeType: "account",
      scopeId: "account-1",
      asOfDate: "2026-08-17",
    });
  });

  it("createSnapshot calls create_snapshot", async () => {
    mockInvoke.mockResolvedValue({});
    await createSnapshot("account-1", "2026-08-17", "snapshot");
    expect(mockInvoke).toHaveBeenCalledWith("create_snapshot", {
      accountId: "account-1",
      snapshotDate: "2026-08-17",
      label: "snapshot",
    });
  });

  it("getSnapshot calls get_snapshot", async () => {
    mockInvoke.mockResolvedValue(null);
    await getSnapshot("snap-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_snapshot", { id: "snap-1" });
  });

  it("listSnapshots calls list_snapshots", async () => {
    mockInvoke.mockResolvedValue([]);
    await listSnapshots("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_snapshots", {
      accountId: "account-1",
    });
  });

  it("deleteSnapshot calls delete_snapshot", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await deleteSnapshot("snap-1");
    expect(mockInvoke).toHaveBeenCalledWith("delete_snapshot", { id: "snap-1" });
  });

  it("computeNetWorth calls compute_net_worth", async () => {
    mockInvoke.mockResolvedValue({});
    await computeNetWorth("2026-08-17", "USD");
    expect(mockInvoke).toHaveBeenCalledWith("compute_net_worth", {
      asOfDate: "2026-08-17",
      baseCurrency: "USD",
    });
  });
});

describe("financial API — Phase 3.5 CRUD", () => {
  beforeEach(() => mockInvoke.mockReset());

  // ── Platform ──

  it("createPlatform calls create_platform", async () => {
    mockInvoke.mockResolvedValue({ id: "platform-1" });
    await createPlatform({ name: "Test", url: "https://test.com", kind: "broker" });
    expect(mockInvoke).toHaveBeenCalledWith("create_platform", {
      input: { name: "Test", url: "https://test.com", kind: "broker" },
    });
  });

  it("listPlatforms calls list_platforms", async () => {
    mockInvoke.mockResolvedValue([]);
    await listPlatforms();
    expect(mockInvoke).toHaveBeenCalledWith("list_platforms");
  });

  it("getPlatform calls get_platform", async () => {
    mockInvoke.mockResolvedValue(null);
    await getPlatform("platform-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_platform", { id: "platform-1" });
  });

  // ── Financial Account ──

  it("createFinancialAccount calls create_financial_account", async () => {
    mockInvoke.mockResolvedValue({ id: "account-1" });
    await createFinancialAccount({
      workspace_id: "ws-1",
      name: "Brokerage",
      account_type: "securities",
      group_name: null,
      currency: "USD",
      is_default: false,
      platform_id: null,
      account_number: null,
      tracking_mode: "transactions",
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_financial_account", {
      input: {
        workspace_id: "ws-1",
        name: "Brokerage",
        account_type: "securities",
        group_name: null,
        currency: "USD",
        is_default: false,
        platform_id: null,
        account_number: null,
        tracking_mode: "transactions",
      },
    });
  });

  it("listFinancialAccounts calls list_financial_accounts", async () => {
    mockInvoke.mockResolvedValue([]);
    await listFinancialAccounts("ws-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_financial_accounts", {
      workspaceId: "ws-1",
    });
  });

  it("listAllFinancialAccounts calls list_all_financial_accounts", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAllFinancialAccounts();
    expect(mockInvoke).toHaveBeenCalledWith("list_all_financial_accounts");
  });

  it("getFinancialAccount calls get_financial_account", async () => {
    mockInvoke.mockResolvedValue(null);
    await getFinancialAccount("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_financial_account", {
      id: "account-1",
    });
  });

  it("archiveFinancialAccount calls archive_financial_account", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await archiveFinancialAccount("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("archive_financial_account", {
      id: "account-1",
    });
  });

  // ── Asset ──

  it("createAsset calls create_asset", async () => {
    mockInvoke.mockResolvedValue({ id: "asset-1" });
    await createAsset({
      kind: "investment",
      name: "Apple Inc.",
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
    expect(mockInvoke).toHaveBeenCalledWith("create_asset", {
      input: {
        kind: "investment",
        name: "Apple Inc.",
        display_code: "AAPL",
        notes: null,
        is_active: true,
        quote_mode: "market",
        quote_ccy: "USD",
        instrument_type: "equity",
        instrument_symbol: "AAPL",
        instrument_exchange_mic: "XNAS",
        provider_config: null,
      },
    });
  });

  it("getAsset calls get_asset", async () => {
    mockInvoke.mockResolvedValue(null);
    await getAsset("asset-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_asset", { id: "asset-1" });
  });

  it("findAssetByInstrumentKey calls find_asset_by_instrument_key", async () => {
    mockInvoke.mockResolvedValue(null);
    await findAssetByInstrumentKey("EQUITY:AAPL@XNAS");
    expect(mockInvoke).toHaveBeenCalledWith("find_asset_by_instrument_key", {
      key: "EQUITY:AAPL@XNAS",
    });
  });

  it("listActiveAssets calls list_active_assets", async () => {
    mockInvoke.mockResolvedValue([]);
    await listActiveAssets();
    expect(mockInvoke).toHaveBeenCalledWith("list_active_assets");
  });

  // ── Quote ──

  it("upsertQuote calls upsert_quote", async () => {
    mockInvoke.mockResolvedValue({});
    await upsertQuote({
      asset_id: "asset-1",
      day: "2026-08-17",
      source: "yahoo",
      open: "150.00",
      high: "152.00",
      low: "149.00",
      close: "151.00",
      adjclose: "151.00",
      volume: "1000000",
      currency: "USD",
      notes: null,
    });
    expect(mockInvoke).toHaveBeenCalledWith("upsert_quote", {
      input: {
        asset_id: "asset-1",
        day: "2026-08-17",
        source: "yahoo",
        open: "150.00",
        high: "152.00",
        low: "149.00",
        close: "151.00",
        adjclose: "151.00",
        volume: "1000000",
        currency: "USD",
        notes: null,
      },
    });
  });

  it("getQuoteForDay calls get_quote_for_day", async () => {
    mockInvoke.mockResolvedValue(null);
    await getQuoteForDay("asset-1", "2026-08-17", "yahoo");
    expect(mockInvoke).toHaveBeenCalledWith("get_quote_for_day", {
      assetId: "asset-1",
      date: "2026-08-17",
      source: "yahoo",
    });
  });

  it("listQuotesForAsset calls list_quotes_for_asset", async () => {
    mockInvoke.mockResolvedValue([]);
    await listQuotesForAsset("asset-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_quotes_for_asset", {
      assetId: "asset-1",
    });
  });

  // ── Activity ──

  it("createActivity calls create_activity", async () => {
    mockInvoke.mockResolvedValue({ id: "activity-1" });
    await createActivity({
      account_id: "account-1",
      asset_id: "asset-1",
      activity_type: "buy",
      activity_type_override: null,
      source_type: null,
      subtype: null,
      status: "posted",
      activity_date: "2026-08-17",
      settlement_date: null,
      quantity: "10",
      unit_price: "150.00",
      amount: "1500.00",
      fee: "1.00",
      tax: "0.00",
      currency: "USD",
      fx_rate: null,
      notes: null,
      metadata: null,
      source_system: null,
      source_record_id: null,
      source_group_id: null,
      idempotency_key: null,
      import_run_id: null,
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_activity", {
      input: {
        account_id: "account-1",
        asset_id: "asset-1",
        activity_type: "buy",
        activity_type_override: null,
        source_type: null,
        subtype: null,
        status: "posted",
        activity_date: "2026-08-17",
        settlement_date: null,
        quantity: "10",
        unit_price: "150.00",
        amount: "1500.00",
        fee: "1.00",
        tax: "0.00",
        currency: "USD",
        fx_rate: null,
        notes: null,
        metadata: null,
        source_system: null,
        source_record_id: null,
        source_group_id: null,
        idempotency_key: null,
        import_run_id: null,
      },
    });
  });

  it("getActivity calls get_activity", async () => {
    mockInvoke.mockResolvedValue(null);
    await getActivity("activity-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_activity", {
      id: "activity-1",
    });
  });

  it("listActivitiesByAccount calls list_activities_by_account", async () => {
    mockInvoke.mockResolvedValue([]);
    await listActivitiesByAccount("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_activities_by_account", {
      accountId: "account-1",
    });
  });

  it("listActivitiesByAsset calls list_activities_by_asset", async () => {
    mockInvoke.mockResolvedValue([]);
    await listActivitiesByAsset("asset-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_activities_by_asset", {
      assetId: "asset-1",
    });
  });

  // ── Import Run ──

  it("createImportRun calls create_import_run", async () => {
    mockInvoke.mockResolvedValue({ id: "import-1" });
    await createImportRun({
      account_id: "account-1",
      source_system: "csv",
      run_type: "full",
      mode: "replace",
      status: "running",
      review_mode: "auto",
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_import_run", {
      input: {
        account_id: "account-1",
        source_system: "csv",
        run_type: "full",
        mode: "replace",
        status: "running",
        review_mode: "auto",
      },
    });
  });

  it("listImportRuns calls list_import_runs", async () => {
    mockInvoke.mockResolvedValue([]);
    await listImportRuns("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_import_runs", {
      accountId: "account-1",
    });
  });

  // ── Lot ──

  it("createLot calls create_lot", async () => {
    mockInvoke.mockResolvedValue({ id: "lot-1" });
    await createLot({
      account_id: "account-1",
      asset_id: "asset-1",
      open_date: "2026-08-17",
      open_activity_id: "activity-1",
      original_quantity: "10",
      cost_per_unit: "150.00",
      original_cost_basis: "1500.00",
      fee_allocated: "0.00",
      currency: "USD",
      base_currency: "USD",
      fx_rate_to_base: "1.0",
      fx_rate_to_account: null,
      account_currency: null,
      cost_basis_method: "fifo",
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_lot", {
      input: {
        account_id: "account-1",
        asset_id: "asset-1",
        open_date: "2026-08-17",
        open_activity_id: "activity-1",
        original_quantity: "10",
        cost_per_unit: "150.00",
        original_cost_basis: "1500.00",
        fee_allocated: "0.00",
        currency: "USD",
        base_currency: "USD",
        fx_rate_to_base: "1.0",
        fx_rate_to_account: null,
        account_currency: null,
        cost_basis_method: "fifo",
      },
    });
  });

  it("getLot calls get_lot", async () => {
    mockInvoke.mockResolvedValue(null);
    await getLot("lot-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_lot", { id: "lot-1" });
  });

  // ── Valuation CRUD ──

  it("upsertValuation calls upsert_valuation", async () => {
    mockInvoke.mockResolvedValue({});
    await upsertValuation({
      account_id: "account-1",
      valuation_date: "2026-08-17",
      account_currency: "USD",
      base_currency: "USD",
      fx_rate_to_base: "1.0",
      cash_balance: "1000",
      investment_market_value: "5000",
      total_value: "6000",
      cost_basis: "5000",
      net_contribution: "5000",
      cash_balance_base: "1000",
      investment_market_value_base: "5000",
      total_value_base: "6000",
      cost_basis_base: "5000",
      net_contribution_base: "5000",
      external_inflow_base: "0",
      external_outflow_base: "0",
      performance_eligible_value_base: "6000",
      external_flow_source: "manual",
      value_status: "estimated",
      basis_status: "filled",
    });
    expect(mockInvoke).toHaveBeenCalledWith("upsert_valuation", {
      input: {
        account_id: "account-1",
        valuation_date: "2026-08-17",
        account_currency: "USD",
        base_currency: "USD",
        fx_rate_to_base: "1.0",
        cash_balance: "1000",
        investment_market_value: "5000",
        total_value: "6000",
        cost_basis: "5000",
        net_contribution: "5000",
        cash_balance_base: "1000",
        investment_market_value_base: "5000",
        total_value_base: "6000",
        cost_basis_base: "5000",
        net_contribution_base: "5000",
        external_inflow_base: "0",
        external_outflow_base: "0",
        performance_eligible_value_base: "6000",
        external_flow_source: "manual",
        value_status: "estimated",
        basis_status: "filled",
      },
    });
  });

  it("listValuationsByAccount calls list_valuations_by_account", async () => {
    mockInvoke.mockResolvedValue([]);
    await listValuationsByAccount("account-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_valuations_by_account", {
      accountId: "account-1",
    });
  });

  it("deleteValuationForDate calls delete_valuation_for_date", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await deleteValuationForDate("account-1", "2026-08-17");
    expect(mockInvoke).toHaveBeenCalledWith("delete_valuation_for_date", {
      accountId: "account-1",
      date: "2026-08-17",
    });
  });

  // ── Taxonomy ──

  it("createTaxonomy calls create_taxonomy", async () => {
    mockInvoke.mockResolvedValue({ id: "taxonomy-1" });
    await createTaxonomy({
      name: "Asset Classes",
      color: "#FF0000",
      description: null,
      is_system: false,
      is_single_select: false,
      sort_order: 0,
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_taxonomy", {
      input: {
        name: "Asset Classes",
        color: "#FF0000",
        description: null,
        is_system: false,
        is_single_select: false,
        sort_order: 0,
      },
    });
  });

  it("getTaxonomy calls get_taxonomy", async () => {
    mockInvoke.mockResolvedValue(null);
    await getTaxonomy("taxonomy-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_taxonomy", {
      id: "taxonomy-1",
    });
  });

  it("listTaxonomies calls list_taxonomies", async () => {
    mockInvoke.mockResolvedValue([]);
    await listTaxonomies();
    expect(mockInvoke).toHaveBeenCalledWith("list_taxonomies");
  });

  it("createTaxonomyCategory calls create_taxonomy_category", async () => {
    mockInvoke.mockResolvedValue({ id: "cat-1" });
    await createTaxonomyCategory({
      taxonomy_id: "taxonomy-1",
      parent_id: null,
      name: "Equity",
      key: "EQUITY",
      color: "#00FF00",
      description: null,
      sort_order: 1,
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_taxonomy_category", {
      input: {
        taxonomy_id: "taxonomy-1",
        parent_id: null,
        name: "Equity",
        key: "EQUITY",
        color: "#00FF00",
        description: null,
        sort_order: 1,
      },
    });
  });

  it("listTaxonomyCategories calls list_taxonomy_categories", async () => {
    mockInvoke.mockResolvedValue([]);
    await listTaxonomyCategories("taxonomy-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_taxonomy_categories", {
      taxonomyId: "taxonomy-1",
    });
  });

  it("assignAssetToTaxonomyCategory calls assign_asset_to_taxonomy_category", async () => {
    mockInvoke.mockResolvedValue({ id: "assign-1" });
    await assignAssetToTaxonomyCategory({
      asset_id: "asset-1",
      taxonomy_id: "taxonomy-1",
      category_id: "cat-1",
      weight: 1.0,
      source: "manual",
    });
    expect(mockInvoke).toHaveBeenCalledWith(
      "assign_asset_to_taxonomy_category",
      {
        input: {
          asset_id: "asset-1",
          taxonomy_id: "taxonomy-1",
          category_id: "cat-1",
          weight: 1.0,
          source: "manual",
        },
      },
    );
  });

  it("listAssignmentsForAsset calls list_assignments_for_asset", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAssignmentsForAsset("asset-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_assignments_for_asset", {
      assetId: "asset-1",
    });
  });

  it("listAssignmentsByTaxonomy calls list_assignments_by_taxonomy", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAssignmentsByTaxonomy("taxonomy-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_assignments_by_taxonomy", {
      taxonomyId: "taxonomy-1",
    });
  });

  it("removeTaxonomyAssignment calls remove_taxonomy_assignment", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await removeTaxonomyAssignment("assign-1");
    expect(mockInvoke).toHaveBeenCalledWith("remove_taxonomy_assignment", {
      id: "assign-1",
    });
  });

  // ── Allocation Target ──

  it("createAllocationTarget calls create_allocation_target", async () => {
    mockInvoke.mockResolvedValue({ id: "target-1" });
    await createAllocationTarget({
      name: "60/40",
      scope_type: "workspace",
      scope_id: "ws-1",
      taxonomy_id: "taxonomy-1",
      trigger_type: "threshold",
      drift_band_bps: 500,
      rebalance_goal: "maintain",
      min_trade_amount: "100",
      whole_shares_only: false,
      allow_sells: true,
      max_turnover_bps: null,
    });
    expect(mockInvoke).toHaveBeenCalledWith("create_allocation_target", {
      input: {
        name: "60/40",
        scope_type: "workspace",
        scope_id: "ws-1",
        taxonomy_id: "taxonomy-1",
        trigger_type: "threshold",
        drift_band_bps: 500,
        rebalance_goal: "maintain",
        min_trade_amount: "100",
        whole_shares_only: false,
        allow_sells: true,
        max_turnover_bps: null,
      },
    });
  });

  it("getAllocationTarget calls get_allocation_target", async () => {
    mockInvoke.mockResolvedValue(null);
    await getAllocationTarget("target-1");
    expect(mockInvoke).toHaveBeenCalledWith("get_allocation_target", {
      id: "target-1",
    });
  });

  it("listAllocationTargets calls list_allocation_targets", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAllocationTargets(false);
    expect(mockInvoke).toHaveBeenCalledWith("list_allocation_targets", {
      includeArchived: false,
    });
  });

  it("archiveAllocationTarget calls archive_allocation_target", async () => {
    mockInvoke.mockResolvedValue(undefined);
    await archiveAllocationTarget("target-1");
    expect(mockInvoke).toHaveBeenCalledWith("archive_allocation_target", {
      id: "target-1",
    });
  });

  it("addAllocationWeight calls add_allocation_weight", async () => {
    mockInvoke.mockResolvedValue({ id: "weight-1" });
    await addAllocationWeight({
      target_id: "target-1",
      taxonomy_id: "taxonomy-1",
      category_id: "cat-1",
      target_bps: 6000,
      is_locked: false,
      is_required: false,
    });
    expect(mockInvoke).toHaveBeenCalledWith("add_allocation_weight", {
      input: {
        target_id: "target-1",
        taxonomy_id: "taxonomy-1",
        category_id: "cat-1",
        target_bps: 6000,
        is_locked: false,
        is_required: false,
      },
    });
  });

  it("listAllocationWeights calls list_allocation_weights", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAllocationWeights("target-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_allocation_weights", {
      targetId: "target-1",
    });
  });

  it("addAllocationConstraint calls add_allocation_constraint", async () => {
    mockInvoke.mockResolvedValue({ id: "constraint-1" });
    await addAllocationConstraint({
      target_id: "target-1",
      subject_type: "category",
      subject_id: "cat-1",
      action: "buy",
      effect: "allow",
      reason: null,
      metadata_json: null,
    });
    expect(mockInvoke).toHaveBeenCalledWith("add_allocation_constraint", {
      input: {
        target_id: "target-1",
        subject_type: "category",
        subject_id: "cat-1",
        action: "buy",
        effect: "allow",
        reason: null,
        metadata_json: null,
      },
    });
  });

  it("listAllocationConstraints calls list_allocation_constraints", async () => {
    mockInvoke.mockResolvedValue([]);
    await listAllocationConstraints("target-1");
    expect(mockInvoke).toHaveBeenCalledWith("list_allocation_constraints", {
      targetId: "target-1",
    });
  });
});