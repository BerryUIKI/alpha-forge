import * as ToastPrimitive from "@radix-ui/react-toast";
import { AlertCircle, CheckCircle2, Info, X } from "lucide-react";
import { useCallback, useMemo, useState, type ReactNode } from "react";

import { ToastContext, type ToastInput, type ToastVariant } from "./toast-context";

interface ToastItem extends ToastInput {
  id: string;
}

const variantClasses: Record<ToastVariant, string> = {
  default: "border-border",
  success: "border-green-600/40",
  destructive: "border-destructive/50",
};

const variantIcons: Record<ToastVariant, ReactNode> = {
  default: <Info className="h-5 w-5 text-muted-foreground" />,
  success: <CheckCircle2 className="h-5 w-5 text-green-600" />,
  destructive: <AlertCircle className="h-5 w-5 text-destructive" />,
};

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  const dismiss = useCallback((id: string) => {
    setToasts((current) => current.filter((item) => item.id !== id));
  }, []);

  const toast = useCallback((input: ToastInput) => {
    const id =
      typeof crypto !== "undefined" && "randomUUID" in crypto
        ? crypto.randomUUID()
        : `${Date.now()}-${Math.random().toString(16).slice(2)}`;

    setToasts((current) => [...current, { id, variant: "default", ...input }]);
    return id;
  }, []);

  const value = useMemo(() => ({ toast, dismiss }), [toast, dismiss]);

  return (
    <ToastPrimitive.Provider swipeDirection="right">
      <ToastContext.Provider value={value}>
        {children}
        {toasts.map((item) => {
          const variant = item.variant ?? "default";

          return (
            <ToastPrimitive.Root
              key={item.id}
              open
              duration={item.duration ?? 5000}
              onOpenChange={(open) => {
                if (!open) {
                  dismiss(item.id);
                }
              }}
              className={`pointer-events-auto flex w-full items-start gap-3 rounded-lg border bg-card p-4 text-card-foreground shadow-lg ${variantClasses[variant]}`}
            >
              <div className="mt-0.5 shrink-0">{variantIcons[variant]}</div>
              <div className="min-w-0 flex-1">
                <ToastPrimitive.Title className="text-sm font-semibold">
                  {item.title}
                </ToastPrimitive.Title>
                {item.description && (
                  <ToastPrimitive.Description className="mt-1 text-sm text-muted-foreground">
                    {item.description}
                  </ToastPrimitive.Description>
                )}
              </div>
              <ToastPrimitive.Close
                aria-label="Close notification"
                className="shrink-0 rounded-md p-1 text-muted-foreground hover:bg-accent hover:text-accent-foreground"
              >
                <X className="h-4 w-4" />
              </ToastPrimitive.Close>
            </ToastPrimitive.Root>
          );
        })}
        <ToastPrimitive.Viewport className="fixed bottom-0 right-0 z-[100] flex max-h-screen w-full max-w-sm flex-col gap-2 p-4 outline-none" />
      </ToastContext.Provider>
    </ToastPrimitive.Provider>
  );
}
