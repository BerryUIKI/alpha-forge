/**
 * Import Activities Dialog
 *
 * Modal dialog for uploading and importing broker activity statement CSVs
 * (supporting Generic CSV and Interactive Brokers Activity Statements).
 *
 * @module features/portfolio/components/ImportActivitiesDialog
 */

import { useState, useRef } from "react";
import { X, Upload, FileText, CheckCircle2, AlertCircle } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useImportActivitiesCsv } from "../hooks/useFinancialData";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";

interface ImportActivitiesDialogProps {
  isOpen: boolean;
  onClose: () => void;
  accountId: string;
  onSuccess?: () => void;
}

export function ImportActivitiesDialog({
  isOpen,
  onClose,
  accountId,
  onSuccess,
}: ImportActivitiesDialogProps) {
  const { t } = useLocale();
  const [format, setFormat] = useState<"GENERIC" | "IBKR">("GENERIC");
  const [csvText, setCsvText] = useState("");
  const [fileName, setFileName] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [isSuccess, setIsSuccess] = useState(false);

  const importMutation = useImportActivitiesCsv();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
  });

  useEscapeKey(onClose, isOpen);

  if (!isOpen) return null;

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setFileName(file.name);
    setErrorMsg(null);
    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      setCsvText(text);
    };
    reader.onerror = () => {
      setErrorMsg("Failed to read file.");
    };
    reader.readAsText(file);
  };

  const handleImport = async () => {
    if (!csvText.trim()) {
      setErrorMsg("Please select or paste a CSV statement.");
      return;
    }

    setErrorMsg(null);
    try {
      await importMutation.mutateAsync({
        accountId,
        format,
        csvText,
      });
      setIsSuccess(true);
      if (onSuccess) {
        onSuccess();
      }
      setTimeout(() => {
        setIsSuccess(false);
        setCsvText("");
        setFileName(null);
        onClose();
      }, 1200);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMsg(msg || "Failed to import statement");
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm animate-in fade-in">
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="import-activities-title"
        className="relative w-full max-w-lg rounded-xl border bg-card p-6 text-card-foreground shadow-2xl transition-all"
      >
        <button
          type="button"
          onClick={onClose}
          className="absolute right-4 top-4 rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
          aria-label="Close"
        >
          <X className="h-4 w-4" />
        </button>

        <h2 id="import-activities-title" className="text-lg font-semibold tracking-tight">
          {t("importActivitiesTitle")}
        </h2>
        <p className="mt-1 text-xs text-muted-foreground">
          {t("importActivitiesDescription")}
        </p>

        <div className="mt-4 space-y-4">
          <div>
            <label className="block text-xs font-medium text-foreground mb-1.5">
              Statement Format
            </label>
            <select
              value={format}
              onChange={(e) => setFormat(e.target.value as "GENERIC" | "IBKR")}
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              <option value="GENERIC">Generic Broker CSV (date, type, symbol, qty, price...)</option>
              <option value="IBKR">Interactive Brokers (IBKR) Activity Statement</option>
            </select>
          </div>

          <div>
            <label className="block text-xs font-medium text-foreground mb-1.5">
              Upload CSV File
            </label>
            <input
              ref={fileInputRef}
              type="file"
              accept=".csv,text/csv"
              onChange={handleFileUpload}
              className="hidden"
            />
            <button
              type="button"
              onClick={() => fileInputRef.current?.click()}
              className="flex w-full flex-col items-center justify-center rounded-lg border-2 border-dashed border-muted p-5 text-center hover:border-accent hover:bg-muted/30 transition-colors"
            >
              {fileName ? (
                <div className="flex items-center gap-2 text-sm font-medium text-foreground">
                  <FileText className="h-4 w-4 text-primary" />
                  <span>{fileName}</span>
                </div>
              ) : (
                <>
                  <Upload className="h-6 w-6 text-muted-foreground mb-1.5" />
                  <span className="text-xs font-medium text-muted-foreground">
                    Click to browse or drop CSV statement file
                  </span>
                </>
              )}
            </button>
          </div>

          <div>
            <label className="block text-xs font-medium text-foreground mb-1.5">
              Or Paste CSV Raw Text
            </label>
            <textarea
              rows={4}
              value={csvText}
              onChange={(e) => setCsvText(e.target.value)}
              placeholder="date,type,symbol,quantity,price,amount,fee,currency,notes..."
              className="w-full rounded-md border border-input bg-background p-2 text-xs font-mono ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            />
          </div>

          {errorMsg && (
            <div className="flex items-center gap-2 rounded-md bg-destructive/10 p-2.5 text-xs text-destructive">
              <AlertCircle className="h-4 w-4 shrink-0" />
              <span>{errorMsg}</span>
            </div>
          )}

          {isSuccess && (
            <div className="flex items-center gap-2 rounded-md bg-green-500/10 p-2.5 text-xs text-green-600 dark:text-green-400">
              <CheckCircle2 className="h-4 w-4 shrink-0" />
              <span>Activities imported successfully!</span>
            </div>
          )}

          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md border border-input bg-background px-3 py-1.5 text-xs font-medium hover:bg-muted hover:text-foreground"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={handleImport}
              disabled={importMutation.isPending || !csvText.trim()}
              className="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
            >
              {importMutation.isPending ? "Importing…" : "Import Statement"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
