/**
 * TaxonomyAllocationPanel Component
 *
 * Provides Category 11 functionality for Portfolio:
 * 1. Displays multi-dimensional taxonomies and their category hierarchy.
 * 2. Displays allocation targets with model weights vs actual holding weights.
 * 3. Shows rebalancing drift warnings and compliance status against drift bands.
 * 4. Enables quick creation of custom taxonomy schemes, categories, and target allocations.
 *
 * @module features/portfolio/components/TaxonomyAllocationPanel
 */

import { useState } from "react";
import { useLocale } from "@/lib/i18n/useLocale";
import {
  useListTaxonomies,
  useCreateTaxonomy,
  useListTaxonomyCategories,
  useCreateTaxonomyCategory,
  useListAllocationTargets,
  useCreateAllocationTarget,
  useListAllocationWeights,
  useAddAllocationWeight,
  useAllocation,
} from "../hooks/useFinancialData";
import {
  Layers,
  Target,
  AlertTriangle,
  CheckCircle2,
  Plus,
  ChevronRight,
  TrendingUp,
} from "lucide-react";

interface TaxonomyAllocationPanelProps {
  scopeType?: string;
  scopeId?: string | null;
  asOfDate: string;
}

export function TaxonomyAllocationPanel({
  scopeType = "account",
  scopeId = null,
  asOfDate,
}: TaxonomyAllocationPanelProps) {
  const { t } = useLocale();

  // Selected state
  const [selectedTaxonomyId, setSelectedTaxonomyId] = useState<string>("");
  const [selectedTargetId, setSelectedTargetId] = useState<string>("");

  // Creation forms
  const [isAddingTaxonomy, setIsAddingTaxonomy] = useState(false);
  const [newTaxonomyName, setNewTaxonomyName] = useState("");
  const [isAddingCategory, setIsAddingCategory] = useState(false);
  const [newCategoryName, setNewCategoryName] = useState("");
  const [newCategoryKey, setNewCategoryKey] = useState("");
  const [isAddingTarget, setIsAddingTarget] = useState(false);
  const [newTargetName, setNewTargetName] = useState("");
  const [newDriftBandBps, setNewDriftBandBps] = useState(500); // 5% default
  const [isAddingWeight, setIsAddingWeight] = useState(false);
  const [weightCategoryId, setWeightCategoryId] = useState("");
  const [targetWeightPct, setTargetWeightPct] = useState(20);

  // Queries
  const taxonomiesQuery = useListTaxonomies();
  const taxonomies = taxonomiesQuery.data ?? [];
  const activeTaxonomyId = selectedTaxonomyId || taxonomies[0]?.id || "";

  const categoriesQuery = useListTaxonomyCategories(activeTaxonomyId);
  const categories = categoriesQuery.data ?? [];

  const allocationTargetsQuery = useListAllocationTargets();
  const targets = allocationTargetsQuery.data ?? [];
  const activeTargetId = selectedTargetId || targets[0]?.id || "";
  const activeTarget = targets.find((tg) => tg.id === activeTargetId);

  const weightsQuery = useListAllocationWeights(activeTargetId);
  const weights = weightsQuery.data ?? [];

  // Actual portfolio allocation data for comparison
  const allocationQuery = useAllocation(scopeType, scopeId, asOfDate);
  const actualCategories = allocationQuery.data?.categories ?? [];

  // Mutations
  const createTaxonomyMutation = useCreateTaxonomy();
  const createCategoryMutation = useCreateTaxonomyCategory();
  const createTargetMutation = useCreateAllocationTarget();
  const addWeightMutation = useAddAllocationWeight();

  const handleCreateTaxonomy = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTaxonomyName.trim()) return;
    try {
      const created = await createTaxonomyMutation.mutateAsync({
        name: newTaxonomyName.trim(),
        color: "#2563eb",
        description: null,
        is_system: false,
        is_single_select: true,
        sort_order: (taxonomies.length + 1) * 10,
      });
      setNewTaxonomyName("");
      setIsAddingTaxonomy(false);
      setSelectedTaxonomyId(created.id);
    } catch {
      // Handled by UI state
    }
  };

  const handleCreateCategory = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newCategoryName.trim() || !activeTaxonomyId) return;
    try {
      await createCategoryMutation.mutateAsync({
        taxonomy_id: activeTaxonomyId,
        parent_id: null,
        name: newCategoryName.trim(),
        key: newCategoryKey.trim() || newCategoryName.trim().toLowerCase().replace(/\s+/g, "_"),
        color: "#3b82f6",
        description: null,
        sort_order: (categories.length + 1) * 10,
      });
      setNewCategoryName("");
      setNewCategoryKey("");
      setIsAddingCategory(false);
    } catch {
      // Handled by UI state
    }
  };

  const handleCreateTarget = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTargetName.trim() || !activeTaxonomyId) return;
    try {
      const created = await createTargetMutation.mutateAsync({
        name: newTargetName.trim(),
        scope_type: "account",
        scope_id: scopeId,
        taxonomy_id: activeTaxonomyId,
        trigger_type: "drift_band",
        drift_band_bps: Number(newDriftBandBps) || 500,
        rebalance_goal: "full",
        min_trade_amount: "100.00",
        whole_shares_only: false,
        allow_sells: true,
        max_turnover_bps: null,
      });
      setNewTargetName("");
      setIsAddingTarget(false);
      setSelectedTargetId(created.id);
    } catch {
      // Handled by UI state
    }
  };

  const handleAddWeight = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!activeTargetId || !weightCategoryId) return;
    try {
      await addWeightMutation.mutateAsync({
        target_id: activeTargetId,
        taxonomy_id: activeTaxonomyId,
        category_id: weightCategoryId,
        target_bps: Math.round(Number(targetWeightPct) * 100),
        is_locked: false,
        is_required: true,
      });
      setIsAddingWeight(false);
      setWeightCategoryId("");
    } catch {
      // Handled by UI state
    }
  };

  return (
    <div className="rounded-lg border bg-card p-4 space-y-5" data-testid="taxonomy-allocation-panel">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b pb-3">
        <div>
          <div className="flex items-center gap-2">
            <Layers className="h-5 w-5 text-primary" />
            <h2 className="text-base font-semibold">
              {t("taxonomyManagementTitle")}
            </h2>
          </div>
          <p className="text-xs text-muted-foreground mt-0.5">
            {t("taxonomyManagementDesc")}
          </p>
        </div>
      </div>

      {/* Main Grid: Taxonomies & Categories vs Allocation Targets */}
      <div className="grid gap-6 md:grid-cols-2">
        {/* Left Column: Taxonomies & Category Hierarchy */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5">
              <Layers className="h-3.5 w-3.5" />
              {t("taxonomiesLabel")}
            </h3>
            <button
              type="button"
              onClick={() => setIsAddingTaxonomy((v) => !v)}
              className="inline-flex items-center gap-1 text-xs text-primary hover:underline font-medium"
            >
              <Plus className="h-3.5 w-3.5" />
              {t("createTaxonomyBtn")}
            </button>
          </div>

          {isAddingTaxonomy && (
            <form onSubmit={handleCreateTaxonomy} className="flex gap-2 p-2 rounded-md bg-muted/40 border">
              <input
                type="text"
                placeholder={t("taxonomyNameLabel")}
                value={newTaxonomyName}
                onChange={(e) => setNewTaxonomyName(e.target.value)}
                className="flex-1 rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
              />
              <button
                type="submit"
                disabled={!newTaxonomyName.trim() || createTaxonomyMutation.isPending}
                className="rounded bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground disabled:opacity-50"
              >
                {t("createTaxonomyBtn")}
              </button>
            </form>
          )}

          {taxonomies.length === 0 ? (
            <div className="rounded border border-dashed p-4 text-center text-xs text-muted-foreground">
              <p>{t("noTaxonomiesYet")}</p>
              <p className="mt-1">{t("noTaxonomiesDesc")}</p>
            </div>
          ) : (
            <div className="flex flex-wrap gap-1.5">
              {taxonomies.map((tx) => (
                <button
                  key={tx.id}
                  type="button"
                  onClick={() => setSelectedTaxonomyId(tx.id)}
                  className={`inline-flex items-center gap-1.5 rounded-full px-3 py-1 text-xs font-medium transition-colors ${
                    tx.id === activeTaxonomyId
                      ? "bg-primary text-primary-foreground"
                      : "bg-muted text-muted-foreground hover:bg-muted/80"
                  }`}
                >
                  <span
                    className="h-2 w-2 rounded-full"
                    style={{ backgroundColor: tx.color || "#2563eb" }}
                  />
                  {tx.name}
                </button>
              ))}
            </div>
          )}

          {/* Categories under active taxonomy */}
          {activeTaxonomyId && (
            <div className="space-y-2 pt-2 border-t">
              <div className="flex items-center justify-between">
                <h4 className="text-xs font-medium text-muted-foreground">
                  {t("taxonomyCategoriesLabel")} ({categories.length})
                </h4>
                <button
                  type="button"
                  onClick={() => setIsAddingCategory((v) => !v)}
                  className="inline-flex items-center gap-1 text-xs text-primary hover:underline font-medium"
                >
                  <Plus className="h-3.5 w-3.5" />
                  {t("createCategoryBtn")}
                </button>
              </div>

              {isAddingCategory && (
                <form onSubmit={handleCreateCategory} className="grid grid-cols-2 gap-2 p-2 rounded-md bg-muted/40 border">
                  <input
                    type="text"
                    placeholder={t("categoryNameLabel")}
                    value={newCategoryName}
                    onChange={(e) => setNewCategoryName(e.target.value)}
                    className="rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
                  />
                  <input
                    type="text"
                    placeholder={t("categoryKeyLabel")}
                    value={newCategoryKey}
                    onChange={(e) => setNewCategoryKey(e.target.value)}
                    className="rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
                  />
                  <div className="col-span-2 flex justify-end gap-1.5">
                    <button
                      type="submit"
                      disabled={!newCategoryName.trim() || createCategoryMutation.isPending}
                      className="rounded bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground disabled:opacity-50"
                    >
                      {t("createCategoryBtn")}
                    </button>
                  </div>
                </form>
              )}

              {categories.length === 0 ? (
                <p className="text-xs text-muted-foreground italic">
                  {t("noTaxonomiesYet")}
                </p>
              ) : (
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-1.5">
                  {categories.map((cat) => (
                    <div
                      key={cat.id}
                      className="flex items-center justify-between rounded border bg-card/60 px-2.5 py-1.5 text-xs"
                    >
                      <div className="flex items-center gap-1.5">
                        <ChevronRight className="h-3 w-3 text-muted-foreground" />
                        <span className="font-medium">{cat.name}</span>
                      </div>
                      <span className="font-mono text-[10px] text-muted-foreground uppercase bg-muted px-1.5 py-0.5 rounded">
                        {cat.key}
                      </span>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>

        {/* Right Column: Model Allocation Targets & Rebalancing Drift */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-semibold uppercase tracking-wider text-muted-foreground flex items-center gap-1.5">
              <Target className="h-3.5 w-3.5" />
              {t("allocationTargetsLabel")}
            </h3>
            <button
              type="button"
              onClick={() => setIsAddingTarget((v) => !v)}
              className="inline-flex items-center gap-1 text-xs text-primary hover:underline font-medium"
            >
              <Plus className="h-3.5 w-3.5" />
              {t("createTargetBtn")}
            </button>
          </div>

          {isAddingTarget && (
            <form onSubmit={handleCreateTarget} className="grid grid-cols-2 gap-2 p-2 rounded-md bg-muted/40 border">
              <input
                type="text"
                placeholder={t("targetNameLabel")}
                value={newTargetName}
                onChange={(e) => setNewTargetName(e.target.value)}
                className="rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
              />
              <div className="flex items-center gap-1">
                <span className="text-[10px] text-muted-foreground">Drift:</span>
                <input
                  type="number"
                  placeholder="500 (5%)"
                  value={newDriftBandBps}
                  onChange={(e) => setNewDriftBandBps(Number(e.target.value))}
                  className="w-full rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
                />
              </div>
              <div className="col-span-2 flex justify-end gap-1.5">
                <button
                  type="submit"
                  disabled={!newTargetName.trim() || createTargetMutation.isPending}
                  className="rounded bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground disabled:opacity-50"
                >
                  {t("createTargetBtn")}
                </button>
              </div>
            </form>
          )}

          {targets.length === 0 ? (
            <div className="rounded border border-dashed p-4 text-center text-xs text-muted-foreground">
              <p>{t("noAllocationTargetsYet")}</p>
              <p className="mt-1">{t("noAllocationTargetsDesc")}</p>
            </div>
          ) : (
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <select
                  value={activeTargetId}
                  onChange={(e) => setSelectedTargetId(e.target.value)}
                  className="rounded border bg-background px-2.5 py-1 text-xs font-medium outline-none focus:ring-1 focus:ring-primary"
                >
                  {targets.map((tg) => (
                    <option key={tg.id} value={tg.id}>
                      {tg.name} (Drift ±{(tg.drift_band_bps / 100).toFixed(1)}%)
                    </option>
                  ))}
                </select>
                <button
                  type="button"
                  onClick={() => setIsAddingWeight((v) => !v)}
                  className="inline-flex items-center gap-1 text-xs text-primary hover:underline font-medium"
                >
                  <Plus className="h-3.5 w-3.5" />
                  {t("targetBpsLabel")}
                </button>
              </div>

              {isAddingWeight && (
                <form onSubmit={handleAddWeight} className="flex items-center gap-2 p-2 rounded-md bg-muted/40 border">
                  <select
                    value={weightCategoryId}
                    onChange={(e) => setWeightCategoryId(e.target.value)}
                    className="flex-1 rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
                  >
                    <option value="">{t("categoryNameLabel")}...</option>
                    {categories.map((c) => (
                      <option key={c.id} value={c.id}>
                        {c.name}
                      </option>
                    ))}
                  </select>
                  <div className="flex items-center gap-1">
                    <input
                      type="number"
                      value={targetWeightPct}
                      onChange={(e) => setTargetWeightPct(Number(e.target.value))}
                      className="w-16 rounded border bg-background px-2 py-1 text-xs outline-none focus:ring-1 focus:ring-primary"
                    />
                    <span className="text-xs text-muted-foreground">%</span>
                  </div>
                  <button
                    type="submit"
                    disabled={!weightCategoryId || addWeightMutation.isPending}
                    className="rounded bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground disabled:opacity-50"
                  >
                    OK
                  </button>
                </form>
              )}

              {/* Weights Table with Drift Status */}
              <div className="rounded border bg-background/50 overflow-hidden">
                <table className="w-full text-left text-xs">
                  <thead className="bg-muted/50 border-b text-muted-foreground font-medium">
                    <tr>
                      <th className="px-2.5 py-1.5">{t("taxonomyCategoriesLabel")}</th>
                      <th className="px-2.5 py-1.5 text-right">{t("targetBpsLabel")}</th>
                      <th className="px-2.5 py-1.5 text-right">{t("actualBpsLabel")}</th>
                      <th className="px-2.5 py-1.5 text-right">Status</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y">
                    {weights.length === 0 ? (
                      <tr>
                        <td colSpan={4} className="px-2.5 py-3 text-center text-muted-foreground italic">
                          {t("noAllocationTargetsYet")}
                        </td>
                      </tr>
                    ) : (
                      weights.map((w) => {
                        const cat = categories.find((c) => c.id === w.category_id);
                        const actual = actualCategories.find((ac) => ac.category_id === w.category_id);
                        const targetPct = (w.target_bps / 100).toFixed(1);
                        const actualPct = actual ? (actual.actual_bps / 100).toFixed(1) : "0.0";
                        const diffBps = Math.abs((actual?.actual_bps ?? 0) - w.target_bps);
                        const driftBandBps = activeTarget?.drift_band_bps ?? 500;
                        const isCompliant = diffBps <= driftBandBps;

                        return (
                          <tr key={w.id} className="hover:bg-muted/20">
                            <td className="px-2.5 py-1.5 font-medium">
                              {cat?.name ?? w.category_id.slice(0, 8)}
                            </td>
                            <td className="px-2.5 py-1.5 text-right font-mono">
                              {targetPct}%
                            </td>
                            <td className="px-2.5 py-1.5 text-right font-mono">
                              {actualPct}%
                            </td>
                            <td className="px-2.5 py-1.5 text-right">
                              {isCompliant ? (
                                <span className="inline-flex items-center gap-1 text-[11px] text-green-600 dark:text-green-400 font-medium">
                                  <CheckCircle2 className="h-3 w-3" />
                                  {t("withinDriftStatus")}
                                </span>
                              ) : (
                                <span className="inline-flex items-center gap-1 text-[11px] text-amber-600 dark:text-amber-400 font-medium">
                                  <AlertTriangle className="h-3 w-3" />
                                  {t("outOfDriftStatus")}
                                </span>
                              )}
                            </td>
                          </tr>
                        );
                      })
                    )}
                  </tbody>
                </table>
              </div>

              {/* Drift Guidance */}
              {activeTarget && weights.length > 0 && (
                <div className="rounded-md bg-muted/40 p-2 text-xs flex items-center justify-between text-muted-foreground">
                  <div className="flex items-center gap-1.5">
                    <TrendingUp className="h-3.5 w-3.5 text-primary" />
                    <span>
                      {t("driftBandLabel")}: ±{(activeTarget.drift_band_bps / 100).toFixed(1)}%
                    </span>
                  </div>
                  <span className="text-[11px]">
                    {weights.some((w) => {
                      const actual = actualCategories.find((ac) => ac.category_id === w.category_id);
                      return Math.abs((actual?.actual_bps ?? 0) - w.target_bps) > activeTarget.drift_band_bps;
                    })
                      ? t("rebalanceActionRecommended")
                      : t("rebalanceWithinLimits")}
                  </span>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
