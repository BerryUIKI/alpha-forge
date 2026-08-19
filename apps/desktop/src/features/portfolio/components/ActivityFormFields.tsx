/**
 * Add Activity Dialog — Form Fields
 *
 * Presentational field set for the Add Activity dialog. Owns no state;
 * receives field values and change handlers from the container dialog.
 *
 * @module features/portfolio/components/ActivityFormFields
 */

import { useLocale } from "@/lib/i18n/useLocale";
import type { MessageKey } from "@/lib/i18n/locale";
import type {
  Asset,
  ActivityType,
  ActivityStatus,
} from "@/types/financial";

const ACTIVITY_TYPES: { value: ActivityType; label: MessageKey }[] = [
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

interface ActivityFormFieldsProps {
  activityType: ActivityType;
  onActivityTypeChange: (value: ActivityType) => void;
  needsAsset: boolean;
  assetId: string;
  onAssetIdChange: (value: string) => void;
  assets: Asset[] | undefined;
  activityDate: string;
  onActivityDateChange: (value: string) => void;
  settlementDate: string;
  onSettlementDateChange: (value: string) => void;
  quantity: string;
  onQuantityChange: (value: string) => void;
  unitPrice: string;
  onUnitPriceChange: (value: string) => void;
  amount: string;
  onAmountChange: (value: string) => void;
  fee: string;
  onFeeChange: (value: string) => void;
  tax: string;
  onTaxChange: (value: string) => void;
  currency: string;
  onCurrencyChange: (value: string) => void;
  status: ActivityStatus;
  onStatusChange: (value: ActivityStatus) => void;
  notes: string;
  onNotesChange: (value: string) => void;
}

/**
 * All editable fields of the Add Activity dialog.
 */
export function ActivityFormFields({
  activityType,
  onActivityTypeChange,
  needsAsset,
  assetId,
  onAssetIdChange,
  assets,
  activityDate,
  onActivityDateChange,
  settlementDate,
  onSettlementDateChange,
  quantity,
  onQuantityChange,
  unitPrice,
  onUnitPriceChange,
  amount,
  onAmountChange,
  fee,
  onFeeChange,
  tax,
  onTaxChange,
  currency,
  onCurrencyChange,
  status,
  onStatusChange,
  notes,
  onNotesChange,
}: ActivityFormFieldsProps) {
  const { t } = useLocale();

  const inputClass =
    "w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary";

  return (
    <>
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
          onChange={(e) => onActivityTypeChange(e.target.value as ActivityType)}
          className={inputClass}
        >
          {ACTIVITY_TYPES.map((at) => (
            <option key={at.value} value={at.value}>
              {t(at.label)}
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
            onChange={(e) => onAssetIdChange(e.target.value)}
            className={inputClass}
          >
            <option value="">
              -- {t("selectAsset")} --
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
            onChange={(e) => onActivityDateChange(e.target.value)}
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
            onChange={(e) => onSettlementDateChange(e.target.value)}
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
              onChange={(e) => onQuantityChange(e.target.value)}
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
              onChange={(e) => onUnitPriceChange(e.target.value)}
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
            onChange={(e) => onAmountChange(e.target.value)}
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
            onChange={(e) => onFeeChange(e.target.value)}
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
            onChange={(e) => onTaxChange(e.target.value)}
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
            onChange={(e) => onCurrencyChange(e.target.value)}
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
            onChange={(e) => onStatusChange(e.target.value as ActivityStatus)}
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
          onChange={(e) => onNotesChange(e.target.value)}
          placeholder={t("notesPlaceholder")}
          rows={2}
          className={inputClass}
        />
      </div>
    </>
  );
}
