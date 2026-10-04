import { useEffect, useRef } from "react";
import { AlertTriangle, X, Loader2 } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { useFocusTrap, useEscapeKey } from "@/lib/hooks";

export interface ConfirmDialogProps {
  isOpen: boolean;
  title?: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: "danger" | "default";
  isConfirming?: boolean;
  onConfirm: () => void | Promise<void>;
  onClose: () => void;
}

export function ConfirmDialog({
  isOpen,
  title,
  message,
  confirmLabel,
  cancelLabel,
  variant = "danger",
  isConfirming = false,
  onConfirm,
  onClose,
}: ConfirmDialogProps) {
  const { t } = useLocale();
  const triggerRef = useRef<HTMLElement | null>(null);

  // Store triggering element to restore focus upon dialog close
  useEffect(() => {
    if (isOpen) {
      triggerRef.current = document.activeElement as HTMLElement;
    }
  }, [isOpen]);

  const containerRef = useFocusTrap<HTMLDivElement>({
    enabled: isOpen,
    returnFocus: triggerRef.current,
  });

  useEscapeKey(onClose, isOpen && !isConfirming);

  if (!isOpen) return null;

  const displayTitle =
    title ?? (variant === "danger" ? t("deleteConfirmTitle") : t("confirmActionTitle"));
  const displayConfirmLabel = confirmLabel ?? (variant === "danger" ? t("delete") : t("confirm"));
  const displayCancelLabel = cancelLabel ?? t("cancel");

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-150"
      role="alertdialog"
      aria-modal="true"
      aria-labelledby="confirm-dialog-title"
      aria-describedby="confirm-dialog-description"
      onClick={(e) => {
        if (!isConfirming && e.target === e.currentTarget) {
          onClose();
        }
      }}
    >
      <div
        ref={containerRef}
        className="w-full max-w-md rounded-xl border border-white/10 bg-[#1c1e28] p-6 shadow-2xl text-white animate-in zoom-in-95 duration-150"
      >
        <div className="flex items-start justify-between gap-3 mb-4">
          <div className="flex items-center gap-3">
            {variant === "danger" && (
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-red-500/10 text-red-400 border border-red-500/20">
                <AlertTriangle className="h-5 w-5" />
              </div>
            )}
            <h2 id="confirm-dialog-title" className="text-base font-semibold leading-6 text-white">
              {displayTitle}
            </h2>
          </div>
          <button
            type="button"
            onClick={onClose}
            disabled={isConfirming}
            className="rounded-lg p-1.5 text-neutral-400 hover:text-white hover:bg-white/10 transition-colors disabled:opacity-50"
            aria-label="Close"
          >
            <X className="h-4 w-4" />
          </button>
        </div>

        <p id="confirm-dialog-description" className="text-sm text-neutral-300 leading-relaxed mb-6">
          {message}
        </p>

        <div className="flex justify-end items-center gap-3">
          <button
            type="button"
            onClick={onClose}
            disabled={isConfirming}
            className="rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-neutral-300 hover:bg-white/10 hover:text-white transition-colors disabled:opacity-50"
          >
            {displayCancelLabel}
          </button>
          <button
            type="button"
            onClick={() => void onConfirm()}
            disabled={isConfirming}
            className={`inline-flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium text-white transition-colors shadow-sm disabled:opacity-50 ${
              variant === "danger"
                ? "bg-red-600 hover:bg-red-500 focus:ring-2 focus:ring-red-500/40"
                : "bg-cyan-600 hover:bg-cyan-500 focus:ring-2 focus:ring-cyan-500/40"
            }`}
          >
            {isConfirming && <Loader2 className="h-4 w-4 animate-spin" />}
            {displayConfirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
