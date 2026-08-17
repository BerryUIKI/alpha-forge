/**
 * Add Asset Dialog
 *
 * Modal dialog for creating a new asset/instrument (Phase 3.5 CRUD).
 * Submits a CreateAssetInput to the create_asset Tauri command via
 * the desktop API layer.
 *
 * @module features/portfolio/components/AddAssetDialog
 */

import { useState, useRef, useEffect } from "react";
import { X } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useCreateAsset } from "../hooks/useFinancialData";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";
import type {
  AssetKind,
  InstrumentType,
  QuoteMode,
  CreateAssetInput,
} from "@/types/financial";

const ASSET_KINDS: AssetKind[] = [
  "investment",
  "property",
  "vehicle",
  "collectible",
  "precious_metal",
  "private_equity",
  "liability",
  "other",
  "fx",
];

const INSTRUMENT_TYPES: InstrumentType[] = [
  "equity",
  "crypto",
  "fx",
  "option",
  "metal",
];

const QUOTE_MODES: QuoteMode[] = ["market", "manual"];

interface AddAssetDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess?: (assetId: string) => void;
}

export function AddAssetDialog({
  isOpen,
  onClose,
  onSuccess,
}: AddAssetDialogProps) {
  const { t } = useLocale();
  const createMutation = useCreateAsset();

  const [kind, setKind] = useState<AssetKind>("investment");
  const [name, setName] = useState("");
  const [displayCode, setDisplayCode] = useState("");
  const [instrumentType, setInstrumentType] = useState<InstrumentType>("equity");
  const [instrumentSymbol, setInstrumentSymbol] = useState("");
  const [exchangeMic, setExchangeMic] = useState("");
  const [quoteMode, setQuoteMode] = useState<QuoteMode>("market");
  const [quoteCcy, setQuoteCcy] = useState("USD");
  const [notes, setNotes] = useState("");
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
      setKind("investment");
      setName("");
      setDisplayCode("");
      setInstrumentType("equity");
      setInstrumentSymbol("");
      setExchangeMic("");
      setQuoteMode("market");
      setQuoteCcy("USD");
      setNotes("");
      setError("");
    }
  }, [isOpen]);

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
      setError(t("assetNameRequired"));
      return;
    }

    const input: CreateAssetInput = {
      kind,
      name: trimmedName,
      display_code: displayCode.trim() || null,
      notes: notes.trim() || null,
      is_active: true,
      quote_mode: quoteMode,
      quote_ccy: quoteCcy.trim().toUpperCase(),
      instrument_type: instrumentType,
      instrument_symbol:
        instrumentSymbol.trim().toUpperCase() || null,
      instrument_exchange_mic: exchangeMic.trim().toUpperCase() || null,
      provider_config: null,
    };

    try {
      const asset = await createMutation.mutateAsync(input);
      setName("");
      setError("");
      onSuccess?.(asset.id);
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : t("unableToCreateAsset"));
    }
  };

  const inputClass =
    "w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary";

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      role="dialog"
      aria-modal="true"
      aria-labelledby="add-asset-dialog-title"
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
          <h2 id="add-asset-dialog-title" className="text-lg font-semibold">
            {t("addAssetTitle")}
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
            <label htmlFor="asset-name" className="mb-2 block text-sm font-medium">
              {t("assetNameLabel")}
            </label>
            <input
              id="asset-name"
              type="text"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
                setError("");
              }}
              placeholder={t("assetNamePlaceholder")}
              className={inputClass}
              autoFocus
            />
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="mb-4">
              <label htmlFor="asset-kind" className="mb-2 block text-sm font-medium">
                {t("assetKindLabel")}
              </label>
              <select
                id="asset-kind"
                value={kind}
                onChange={(e) => setKind(e.target.value as AssetKind)}
                className={inputClass}
              >
                {ASSET_KINDS.map((k) => (
                  <option key={k} value={k}>
                    {k}
                  </option>
                ))}
              </select>
            </div>

            <div className="mb-4">
              <label htmlFor="display-code" className="mb-2 block text-sm font-medium">
                {t("displayCodeLabel")}
              </label>
              <input
                id="display-code"
                type="text"
                value={displayCode}
                onChange={(e) => setDisplayCode(e.target.value)}
                placeholder={t("displayCodePlaceholder")}
                className={inputClass}
              />
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="mb-4">
              <label htmlFor="instrument-type" className="mb-2 block text-sm font-medium">
                {t("instrumentTypeLabel")}
              </label>
              <select
                id="instrument-type"
                value={instrumentType}
                onChange={(e) => setInstrumentType(e.target.value as InstrumentType)}
                className={inputClass}
              >
                {INSTRUMENT_TYPES.map((it) => (
                  <option key={it} value={it}>
                    {t(`instrumentType${capitalize(it)}` as any)}
                  </option>
                ))}
              </select>
            </div>

            <div className="mb-4">
              <label htmlFor="instrument-symbol" className="mb-2 block text-sm font-medium">
                {t("instrumentSymbolLabel")}
              </label>
              <input
                id="instrument-symbol"
                type="text"
                value={instrumentSymbol}
                onChange={(e) => setInstrumentSymbol(e.target.value)}
                placeholder={t("instrumentSymbolPlaceholder")}
                className={inputClass}
              />
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div className="mb-4">
              <label htmlFor="exchange-mic" className="mb-2 block text-sm font-medium">
                {t("exchangeMicLabel")}
              </label>
              <input
                id="exchange-mic"
                type="text"
                value={exchangeMic}
                onChange={(e) => setExchangeMic(e.target.value)}
                placeholder={t("exchangeMicPlaceholder")}
                className={inputClass}
                maxLength={4}
              />
            </div>

            <div className="mb-4">
              <label htmlFor="quote-ccy" className="mb-2 block text-sm font-medium">
                {t("quoteCurrencyLabel")}
              </label>
              <input
                id="quote-ccy"
                type="text"
                value={quoteCcy}
                onChange={(e) => setQuoteCcy(e.target.value)}
                className={inputClass}
                maxLength={3}
              />
            </div>
          </div>

          <div className="mb-4">
            <label htmlFor="quote-mode" className="mb-2 block text-sm font-medium">
              {t("quoteModeLabel")}
            </label>
            <select
              id="quote-mode"
              value={quoteMode}
              onChange={(e) => setQuoteMode(e.target.value as QuoteMode)}
              className={inputClass}
            >
              {QUOTE_MODES.map((mode) => (
                <option key={mode} value={mode}>
                  {t(`quoteMode${capitalize(mode)}` as any)}
                </option>
              ))}
            </select>
          </div>

          <div className="mb-4">
            <label htmlFor="asset-notes" className="mb-2 block text-sm font-medium">
              {t("notesLabel")}
            </label>
            <textarea
              id="asset-notes"
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
              placeholder={t("notesPlaceholder")}
              rows={2}
              className={inputClass}
            />
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