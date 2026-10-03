/**
 * Import Activities Dialog
 *
 * Modal dialog for uploading and importing broker activity statement CSVs
 * (supporting Generic CSV and Interactive Brokers Activity Statements).
 *
 * Features auditable import run summaries with row-level metrics (created,
 * skipped duplicates, failed) and explicit user dismissal.
 *
 * @module features/portfolio/components/ImportActivitiesDialog
 */

import { useState, useRef } from "react";
import { X, Upload, FileText, CheckCircle2, AlertCircle, AlertTriangle } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useImportActivitiesCsv } from "../hooks/useFinancialData";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";
import type { ImportRun } from "@/lib/desktop-api/financial";

interface ImportActivitiesDialogProps {
  isOpen: boolean;
  onClose: () => void;
  accountId: string;
  onSuccess?: () => void;
}

interface ParsedSummary {
  total_rows: number;
  created_count: number;
  skipped_count: number;
  failed_count: number;
  errors?: string[];
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
  const [importResult, setImportResult] = useState<ImportRun | null>(null);

  const importMutation = useImportActivitiesCsv();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
  });

  const handleClose = () => {
    setImportResult(null);
    setCsvText("");
    setFileName(null);
    setErrorMsg(null);
    onClose();
  };

  useEscapeKey(handleClose, isOpen);

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
      const run = await importMutation.mutateAsync({
        accountId,
        format,
        csvText,
      });
      setImportResult(run);
      if (onSuccess) {
        onSuccess();
      }
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      setErrorMsg(msg || "Failed to import statement");
    }
  };

  let summaryData: ParsedSummary | null = null;
  if (importResult?.summary) {
    try {
      summaryData = JSON.parse(importResult.summary) as ParsedSummary;
    } catch {
      summaryData = null;
    }
  }

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
          onClick={handleClose}
          className="absolute right-4 top-4 rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
          aria-label="Close"
        >
          <X className="h-4 w-4" />
        </button>

        <h2 id="import-activities-title" className="text-lg font-semibold tracking-tight">
          {importResult ? "Import Summary" : t("importActivitiesTitle")}
        </h2>
        <p className="mt-1 text-xs text-muted-foreground">
          {importResult
            ? "Review the applied, skipped, and recorded activities from this statement."
            : t("importActivitiesDescription")}
        </p>

        {importResult ? (
          <div className="mt-4 space-y-4">
            {/* Status Banner */}
            {importResult.status === "COMPLETED" && (
              <div className="flex items-center gap-2.5 rounded-lg border border-green-500/20 bg-green-500/10 p-3 text-xs text-green-600 dark:text-green-400">
                <CheckCircle2 className="h-4 w-4 shrink-0" />
                <span className="font-medium">Statement imported successfully!</span>
              </div>
            )}
            {importResult.status === "PARTIAL" && (
              <div className="flex items-center gap-2.5 rounded-lg border border-amber-500/20 bg-amber-500/10 p-3 text-xs text-amber-600 dark:text-amber-400">
                <AlertTriangle className="h-4 w-4 shrink-0" />
                <span className="font-medium">Statement imported with partial warnings.</span>
              </div>
            )}
            {importResult.status === "FAILED" && (
              <div className="flex items-center gap-2.5 rounded-lg border border-destructive/20 bg-destructive/10 p-3 text-xs text-destructive">
                <AlertCircle className="h-4 w-4 shrink-0" />
                <span className="font-medium">Import completed with errors.</span>
              </div>
            )}

            {/* Metrics Grid */}
            <div className="grid grid-cols-4 gap-2.5 py-1">
              <div className="rounded-lg border bg-muted/40 p-2.5 text-center">
                <div className="text-[10px] uppercase tracking-wider text-muted-foreground">
                  Total Rows
                </div>
                <div className="mt-1 text-base font-semibold text-foreground">
                  {summaryData?.total_rows ?? 0}
                </div>
              </div>
              <div className="rounded-lg border bg-muted/40 p-2.5 text-center">
                <div className="text-[10px] uppercase tracking-wider text-muted-foreground">
                  Created
                </div>
                <div className="mt-1 text-base font-semibold text-green-600 dark:text-green-400">
                  {summaryData?.created_count ?? 0}
                </div>
              </div>
              <div className="rounded-lg border bg-muted/40 p-2.5 text-center">
                <div className="text-[10px] uppercase tracking-wider text-muted-foreground">
                  Skipped
                </div>
                <div className="mt-1 text-base font-semibold text-blue-600 dark:text-blue-400">
                  {summaryData?.skipped_count ?? 0}
                </div>
              </div>
              <div className="rounded-lg border bg-muted/40 p-2.5 text-center">
                <div className="text-[10px] uppercase tracking-wider text-muted-foreground">
                  Failed
                </div>
                <div
                  className={`mt-1 text-base font-semibold ${
                    (summaryData?.failed_count ?? 0) > 0
                      ? "text-destructive"
                      : "text-muted-foreground"
                  }`}
                >
                  {summaryData?.failed_count ?? 0}
                </div>
              </div>
            </div>

            {/* Errors / Warnings List */}
            {summaryData?.errors && summaryData.errors.length > 0 && (
              <div className="max-h-32 overflow-y-auto rounded-md border border-destructive/20 bg-destructive/5 p-2.5 text-xs text-destructive">
                <div className="font-medium mb-1">Row Errors:</div>
                <ul className="list-inside list-disc space-y-0.5">
                  {summaryData.errors.map((err, i) => (
                    <li key={i}>{err}</li>
                  ))}
                </ul>
              </div>
            )}

            {importResult.warnings && !summaryData?.errors?.length && (
              <div className="rounded-md border border-amber-500/20 bg-amber-500/5 p-2.5 text-xs text-amber-600 dark:text-amber-400">
                {importResult.warnings}
              </div>
            )}

            {/* Footer Actions */}
            <div className="flex justify-end gap-2 pt-2">
              <button
                type="button"
                onClick={() => {
                  setImportResult(null);
                  setCsvText("");
                  setFileName(null);
                  setErrorMsg(null);
                }}
                className="rounded-md border border-input bg-background px-3 py-1.5 text-xs font-medium hover:bg-muted hover:text-foreground"
              >
                Import Another
              </button>
              <button
                type="button"
                onClick={handleClose}
                className="rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90"
              >
                Done
              </button>
            </div>
          </div>
        ) : (
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
                <option value="GENERIC">
                  Generic Broker CSV (date, type, symbol, qty, price...)
                </option>
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

            <div className="flex justify-end gap-2 pt-2">
              <button
                type="button"
                onClick={handleClose}
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
        )}
      </div>
    </div>
  );
}
