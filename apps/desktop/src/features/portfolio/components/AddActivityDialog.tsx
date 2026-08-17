/**
 * Add Activity Dialog
 *
 * Modal dialog for recording a new financial activity (Phase 3.5 CRUD).
 * Submits a CreateActivityInput to the create_activity Tauri command via
 * the desktop API layer.
 *
 * @module features/portfolio/components/AddActivityDialog
 */

import { useState, useRef, useEffect } from "react";
import { X } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useCreateActivity, useCreateLot, useRecordSell, useListActiveAssets } from "../hooks/useFinancialData";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";
import type {
  ActivityType,
  ActivityStatus,
  CreateActivityInput,
  Asset,
} from "@/types/financial";

const ACTIVITY_TYPES: { value: ActivityType; label: string }[] = [
  { value: "buy", label: "activityTypeBuy" },
  { value: "sell", label: "activityTypeSell" },
  { value: "dividend", label: "activityTypeDividend" },
  { value: "interest", label: "activityTypeInterest" },
  { value: "deposit", label: "activityTypeDeposit" },
  { value: "withdrawal", label: "activityTypeWithdrawal" },
  { value: "transfer_in", label: "activityTypeTransferIn" },
  { value: "transfer_out", label: "activityTypeTransferOut" },
  { value: "fee", label: "activityTypeFee" },
  { value: "tax", label: "activityTypeTax" },
  { value: "credit", label: "activityTypeCredit" },
  { value: "adjustment", label: "activityTypeAdjustment" },
  { value: "split", label: "activityTypeSplit" },
  { value: "cash_journal", label: "activityTypeCashJournal" },
];

const STATUSES: ActivityStatus[] = ["posted", "pending", "canceled"];

/** Activity types that require an asset (quantity × price semantics). */
const ASSET_REQUIRED_TYPES: ActivityType[] = ["buy", "sell", "split"];

/** Format a value as a decimal string (preserves trailing ".00" for the API). */
function decStr(v: string): string {
  const n = parseFloat(v);
  return isNaN(n) ? "0" : n.toFixed(10).replace(/\.?0+$/, "") || "0";
}

/** Multiply two decimal strings and return the result as a string. */
function decMul(a: string, b: string): string {
  const result = (parseFloat(a) || 0) * (parseFloat(b) || 0);
  return result.toFixed(10).replace(/\.?0+$/, "") || "0";
}

function todayString(): string {
  return new Date().toISOString().slice(0, 10);
}

interface AddActivityDialogProps {
  isOpen: boolean;
  onClose: () => void;
  accountId: string;
  accountCurrency?: string;
  onSuccess?: (activityId: string) => void;
}

export function AddActivityDialog({
  isOpen,
  onClose,
  accountId,
  accountCurrency = "USD",
  onSuccess,
}: AddActivityDialogProps) {
  const { t } = useLocale();
  const createMutation = useCreateActivity();
  const createLotMutation = useCreateLot();
  const recordSellMutation = useRecordSell();
  const { data: assets } = useListActiveAssets();

  const [activityType, setActivityType] = useState<ActivityType>("buy");
  const [assetId, setAssetId] = useState("");
  const [status, setStatus] = useState<ActivityStatus>("posted");
  const [activityDate, setActivityDate] = useState(todayString());
  const [settlementDate, setSettlementDate] = useState("");
  const [quantity, setQuantity] = useState("");
  const [unitPrice, setUnitPrice] = useState("");
  const [amount, setAmount] = useState("");
  const [fee, setFee] = useState("");
  const [tax, setTax] = useState("");
  const [currency, setCurrency] = useState(accountCurrency);
  const [notes, setNotes] = useState("");
  const [error, setError] = useState("");
  const triggerRef = useRef<HTMLElement | null>(null);

  const needsAsset = ASSET_REQUIRED_TYPES.includes(activityType);

  useEffect(() => {
    if (isOpen) {
      triggerRef.current = document.activeElement as HTMLElement;
    }
  }, [isOpen]);

  // Reset form each time the dialog opens
  useEffect(() => {
    if (isOpen) {
      setActivityType("buy");
      setAssetId("");
      setStatus("posted");
      setActivityDate(todayString());
      setSettlementDate("");
      setQuantity("");
      setUnitPrice("");
      setAmount("");
      setFee("");
      setTax("");
      setCurrency(accountCurrency);
      setNotes("");
      setError("");
    }
  }, [isOpen, accountCurrency]);

  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
    returnFocus: triggerRef.current,
  });

  useEscapeKey(onClose, isOpen);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (needsAsset && !assetId) {
      setError(t("assetRequired"));
      return;
    }

    if (needsAsset && !quantity.trim()) {
      setError(t("quantityRequired"));
      return;
    }

    if (needsAsset && !unitPrice.trim()) {
      setError(t("priceRequired"));
      return;
    }

    const input: CreateActivityInput = {
      account_id: accountId,
      asset_id: assetId || null,
      activity_type: activityType,
      activity_type_override: null,
      source_type: null,
      subtype: null,
      status,
      activity_date: activityDate,
      settlement_date: settlementDate.trim() || null,
      quantity: quantity.trim() || null,
      unit_price: unitPrice.trim() || null,
      amount: amount.trim() || null,
      fee: fee.trim() || null,
      tax: tax.trim() || null,
      currency: currency.trim().toUpperCase(),
      fx_rate: null,
      notes: notes.trim() || null,
      metadata: null,
      source_system: null,
      source_record_id: null,
      source_group_id: null,
      idempotency_key: null,
      import_run_id: null,
    };

    try {
      const activity = await createMutation.mutateAsync(input);

      // Auto-create tax lot for buys (cost basis = qty × unit price).
      if (activityType === "buy" && assetId && quantity.trim()) {
        const qty = decStr(quantity.trim());
        const price = decStr(unitPrice.trim() || "0");
        const costBasis = decMul(qty, price);
        const feeVal = fee.trim();
        await createLotMutation.mutateAsync({
          account_id: accountId,
          asset_id: assetId,
          open_date: activityDate,
          open_activity_id: activity.id,
          original_quantity: qty,
          cost_per_unit: price,
          original_cost_basis: costBasis,
          fee_allocated: feeVal && !isNaN(Number(feeVal)) ? feeVal : "0",
          currency: currency.trim().toUpperCase(),
          base_currency: currency.trim().toUpperCase(),
          fx_rate_to_base: "1",
          fx_rate_to_account: null,
          account_currency: null,
          cost_basis_method: "fifo",
        });
      }

      // Dispose lots via the FIFO service for sells.
      if (activityType === "sell" && assetId) {
        await recordSellMutation.mutateAsync({
          accountId,
          assetId,
          activityId: activity.id,
        });
      }

      setError("");
      onSuccess?.(activity.id);
      onClose();
    } catch (err) {
      setError(
        err instanceof Error ? err.message : t("unableToCreateActivity"),
      );
    }
  };

  const inputClass =
    "w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary";

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      role="dialog"
      aria-modal="true"
      aria-labelledby="add-activity-dialog-title"
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onClose();
        }
      }}
    >
      <div
        ref={containerRef}
        className="w-full max-w-lg rounded-lg bg-card p-6 shadow-xl"
      >
        <div className="mb-4 flex items-center justify-between">
          <h2 id="add-activity-dialog-title" className="text-lg font-semibold">
            {t("addActivityTitle")}
          </h2>
          <button
            onClick={onClose}
            className="rounded-md p-1 hover:bg-accent"
            aria-label={t("cancel")}
          >
            <X className="h-5 w-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          {/* Activity type */}
          <div className="mb-4">
            <label
              htmlFor="activity-type"
              className="mb-2 block text-sm font-medium"
            >
              {t("activityTypeLabel")}
            </label>
            <select
              id="activity-type"
              value={activityType}
              onChange={(e) => {
                setActivityType(e.target.value as ActivityType);
                setError("");
              }}
              className={inputClass}
            >
              {ACTIVITY_TYPES.map((at) => (
                <option key={at.value} value={at.value}>
                  {t(at.label as any)}
                </option>
              ))}
            </select>
          </div>

          {/* Asset selector (shown for buy/sell/split) */}
          {needsAsset && (
            <div className="mb-4">
              <label
                htmlFor="activity-asset"
                className="mb-2 block text-sm font-medium"
              >
                {t("assetNameLabel")}
              </label>
              <select
                id="activity-asset"
                value={assetId}
                onChange={(e) => {
                  setAssetId(e.target.value);
                  setError("");
                }}
                className={inputClass}
              >
                <option value="">
                  -- {t("cancel" as any) || "Select"} --
                </option>
                {assets?.map((asset) => (
                  <option key={asset.id} value={asset.id}>
                    {asset.display_code ?? asset.name ?? asset.id}
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Date fields */}
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="mb-4">
              <label
                htmlFor="activity-date"
                className="mb-2 block text-sm font-medium"
              >
                {t("activityDateLabel")}
              </label>
              <input
                id="activity-date"
                type="date"
                value={activityDate}
                onChange={(e) => setActivityDate(e.target.value)}
                className={inputClass}
              />
            </div>

            <div className="mb-4">
              <label
                htmlFor="settlement-date"
                className="mb-2 block text-sm font-medium"
              >
                {t("settlementDateLabel")}
              </label>
              <input
                id="settlement-date"
                type="date"
                value={settlementDate}
                onChange={(e) => setSettlementDate(e.target.value)}
                className={inputClass}
              />
            </div>
          </div>

          {/* Quantity & price (for buy/sell) */}
          {needsAsset && (
            <div className="grid gap-4 sm:grid-cols-2">
              <div className="mb-4">
                <label
                  htmlFor="activity-quantity"
                  className="mb-2 block text-sm font-medium"
                >
                  {t("quantityLabel")}
                </label>
                <input
                  id="activity-quantity"
                  type="text"
                  value={quantity}
                  onChange={(e) => setQuantity(e.target.value)}
                  placeholder="100"
                  className={inputClass}
                />
              </div>

              <div className="mb-4">
                <label
                  htmlFor="unit-price"
                  className="mb-2 block text-sm font-medium"
                >
                  {t("unitPriceLabel")}
                </label>
                <input
                  id="unit-price"
                  type="text"
                  value={unitPrice}
                  onChange={(e) => setUnitPrice(e.target.value)}
                  placeholder={t("unitPricePlaceholder")}
                  className={inputClass}
                />
              </div>
            </div>
          )}

          {/* Amount, fee, tax */}
          <div className="grid gap-4 sm:grid-cols-3">
            <div className="mb-4">
              <label
                htmlFor="activity-amount"
                className="mb-2 block text-sm font-medium"
              >
                {t("amountLabel")}
              </label>
              <input
                id="activity-amount"
                type="text"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder={t("amountPlaceholder")}
                className={inputClass}
              />
            </div>

            <div className="mb-4">
              <label
                htmlFor="activity-fee"
                className="mb-2 block text-sm font-medium"
              >
                {t("feeLabel")}
              </label>
              <input
                id="activity-fee"
                type="text"
                value={fee}
                onChange={(e) => setFee(e.target.value)}
                placeholder="0"
                className={inputClass}
              />
            </div>

            <div className="mb-4">
              <label
                htmlFor="activity-tax"
                className="mb-2 block text-sm font-medium"
              >
                {t("taxLabel")}
              </label>
              <input
                id="activity-tax"
                type="text"
                value={tax}
                onChange={(e) => setTax(e.target.value)}
                placeholder="0"
                className={inputClass}
              />
            </div>
          </div>

          {/* Currency & status */}
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="mb-4">
              <label
                htmlFor="activity-currency"
                className="mb-2 block text-sm font-medium"
              >
                {t("accountCurrencyLabel")}
              </label>
              <input
                id="activity-currency"
                type="text"
                value={currency}
                onChange={(e) => setCurrency(e.target.value)}
                className={inputClass}
                maxLength={3}
              />
            </div>

            <div className="mb-4">
              <label
                htmlFor="activity-status"
                className="mb-2 block text-sm font-medium"
              >
                {t("statusLabel")}
              </label>
              <select
                id="activity-status"
                value={status}
                onChange={(e) => setStatus(e.target.value as ActivityStatus)}
                className={inputClass}
              >
                {STATUSES.map((s) => (
                  <option key={s} value={s}>
                    {t(
                      s === "posted"
                        ? "statusPosted"
                        : s === "pending"
                          ? "statusPending"
                          : "statusCanceled",
                    )}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Notes */}
          <div className="mb-4">
            <label
              htmlFor="activity-notes"
              className="mb-2 block text-sm font-medium"
            >
              {t("notesLabel")}
            </label>
            <textarea
              id="activity-notes"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder={t("notesPlaceholder")}
              rows={2}
              className={inputClass}
            />
          </div>

          {error && (
            <p className="mb-4 text-sm text-destructive" role="alert">
              {error}
            </p>
          )}

          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md border border-input bg-background px-4 py-2 text-sm font-medium hover:bg-accent"
            >
              {t("cancel")}
            </button>
            <button
              type="submit"
              disabled={createMutation.isPending}
              className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              {createMutation.isPending ? t("creating") : t("create")}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}