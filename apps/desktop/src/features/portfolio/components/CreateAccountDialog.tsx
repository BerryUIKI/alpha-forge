/**
 * Create Account Dialog
 *
 * Modal dialog for creating a new financial account (Phase 3.5 CRUD).
 * Submits a CreateAccountInput to the create_financial_account Tauri
 * command via the desktop API layer.
 *
 * @module features/portfolio/components/CreateAccountDialog
 */

import { useState, useRef, useEffect } from "react";
import { X } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useCreateFinancialAccount } from "../hooks/useFinancialData";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";
import type {
  AccountType,
  TrackingMode,
  CreateAccountInput,
} from "@/types/financial";

const ACCOUNT_TYPES: AccountType[] = [
  "securities",
  "cash",
  "credit_card",
  "cryptocurrency",
];

const TRACKING_MODES: TrackingMode[] = [
  "not_set",
  "transactions",
  "holdings",
];

interface CreateAccountDialogProps {
  isOpen: boolean;
  onClose: () => void;
  workspaceId: string;
  /** Base currency for new accounts, usually the workspace currency. */
  defaultCurrency?: string;
  onSuccess?: (accountId: string) => void;
}

export function CreateAccountDialog({
  isOpen,
  onClose,
  workspaceId,
  defaultCurrency = "USD",
  onSuccess,
}: CreateAccountDialogProps) {
  const { t } = useLocale();
  const createMutation = useCreateFinancialAccount();

  const [name, setName] = useState("");
  const [accountType, setAccountType] = useState<AccountType>("securities");
  const [groupName, setGroupName] = useState("");
  const [currency, setCurrency] = useState(defaultCurrency);
  const [accountNumber, setAccountNumber] = useState("");
  const [trackingMode, setTrackingMode] = useState<TrackingMode>("transactions");
  const [error, setError] = useState("");
  const triggerRef = useRef<HTMLElement | null>(null);

  useEffect(() => {
    if (isOpen) {
      triggerRef.current = document.activeElement as HTMLElement;
    }
  }, [isOpen]);

  // Reset form each time the dialog opens
  useEffect(() => {
    if (isOpen) {
      setName("");
      setAccountType("securities");
      setGroupName("");
      setCurrency(defaultCurrency);
      setAccountNumber("");
      setTrackingMode("transactions");
      setError("");
    }
  }, [isOpen, defaultCurrency]);

  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
    returnFocus: triggerRef.current,
  });

  useEscapeKey(onClose, isOpen);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const trimmedName = name.trim();
    if (!trimmedName) {
      setError(t("accountNameRequired"));
      return;
    }

    const input: CreateAccountInput = {
      workspace_id: workspaceId,
      name: trimmedName,
      account_type: accountType,
      group_name: groupName.trim() || null,
      currency: currency.trim().toUpperCase(),
      is_default: false,
      platform_id: null,
      account_number: accountNumber.trim() || null,
      tracking_mode: trackingMode,
    };

    try {
      const account = await createMutation.mutateAsync(input);
      setName("");
      setError("");
      onSuccess?.(account.id);
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : t("unableToCreateAccount"));
    }
  };

  const inputClass =
    "w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary";

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      role="dialog"
      aria-modal="true"
      aria-labelledby="create-account-dialog-title"
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onClose();
        }
      }}
    >
      <div
        ref={containerRef}
        className="w-full max-w-md rounded-lg bg-card p-6 shadow-xl"
      >
        <div className="mb-4 flex items-center justify-between">
          <h2 id="create-account-dialog-title" className="text-lg font-semibold">
            {t("createAccountTitle")}
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
          <div className="mb-4">
            <label htmlFor="account-name" className="mb-2 block text-sm font-medium">
              {t("accountNameLabel")}
            </label>
            <input
              id="account-name"
              type="text"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
                setError("");
              }}
              placeholder={t("accountNamePlaceholder")}
              className={inputClass}
              autoFocus
            />
          </div>

          <div className="mb-4">
            <label htmlFor="account-type" className="mb-2 block text-sm font-medium">
              {t("accountTypeLabel")}
            </label>
            <select
              id="account-type"
              value={accountType}
              onChange={(e) => setAccountType(e.target.value as AccountType)}
              className={inputClass}
            >
              {ACCOUNT_TYPES.map((type) => (
                <option key={type} value={type}>
                  {t(`accountType${capitalize(type)}` as any)}
                </option>
              ))}
            </select>
          </div>

          <div className="mb-4">
            <label htmlFor="account-group" className="mb-2 block text-sm font-medium">
              {t("accountGroupNameLabel")}
            </label>
            <input
              id="account-group"
              type="text"
              value={groupName}
              onChange={(e) => setGroupName(e.target.value)}
              placeholder={t("accountGroupNamePlaceholder")}
              className={inputClass}
            />
          </div>

          <div className="mb-4">
            <label htmlFor="account-currency" className="mb-2 block text-sm font-medium">
              {t("accountCurrencyLabel")}
            </label>
            <input
              id="account-currency"
              type="text"
              value={currency}
              onChange={(e) => setCurrency(e.target.value)}
              className={inputClass}
              maxLength={3}
            />
          </div>

          <div className="mb-4">
            <label htmlFor="account-number" className="mb-2 block text-sm font-medium">
              {t("accountNumberLabel")}
            </label>
            <input
              id="account-number"
              type="text"
              value={accountNumber}
              onChange={(e) => setAccountNumber(e.target.value)}
              placeholder={t("accountNumberPlaceholder")}
              className={inputClass}
            />
          </div>

          <div className="mb-4">
            <label htmlFor="tracking-mode" className="mb-2 block text-sm font-medium">
              {t("trackingModeLabel")}
            </label>
            <select
              id="tracking-mode"
              value={trackingMode}
              onChange={(e) => setTrackingMode(e.target.value as TrackingMode)}
              className={inputClass}
            >
              {TRACKING_MODES.map((mode) => (
                <option key={mode} value={mode}>
                  {mode === "not_set"
                    ? t("cancel" as any)
                    : t(`trackingMode${capitalize(mode)}` as any)}
                </option>
              ))}
            </select>
          </div>

          {error && <p className="mb-4 text-sm text-destructive" role="alert">{error}</p>}

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

function capitalize(value: string): string {
  return value.charAt(0).toUpperCase() + value.slice(1);
}