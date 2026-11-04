// AI provider settings — stores the API key in the OS keychain via Rust.

import { useState } from "react";
import { KeyRound } from "lucide-react";
import { Badge } from "@investment-os/ui";
import { LoadingSpinner } from "@/components/common/LoadingSpinner";
import { ErrorState } from "@/components/common/ErrorState";
import { useToast } from "@/components/common/useToast";
import { getErrorMessage } from "@/lib/errors";
import {
  useClearProviderApiKey,
  useProviderStatus,
  useSetProviderApiKey,
} from "../hooks/useProvider";

export function ProviderSettings() {
  const { toast } = useToast();
  const statusQuery = useProviderStatus();
  const { saveApiKey, isSavingKey } = useSetProviderApiKey();
  const clearApiKey = useClearProviderApiKey();
  const [apiKey, setApiKey] = useState("");

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();

    const trimmed = apiKey.trim();
    if (!trimmed) {
      return;
    }

    try {
      await saveApiKey(trimmed);
      setApiKey("");
      toast({ title: "API key saved", variant: "success" });
    } catch (error) {
      toast({
        title: "Failed to save API key",
        description: getErrorMessage(error),
        variant: "destructive",
      });
    }
  };

  const handleClear = () => {
    clearApiKey.mutate(undefined, {
      onSuccess: () => {
        toast({ title: "API key removed", variant: "success" });
      },
      onError: (error) => {
        toast({
          title: "Failed to remove API key",
          description: getErrorMessage(error),
          variant: "destructive",
        });
      },
    });
  };

  if (statusQuery.isLoading) {
    return <LoadingSpinner className="p-8" />;
  }

  if (statusQuery.error) {
    return (
      <ErrorState
        message="Failed to load provider settings"
        onRetry={() => statusQuery.refetch()}
      />
    );
  }

  const status = statusQuery.data;
  const configured = status?.configured ?? false;

  return (
    <section className="rounded-lg border border-border bg-card p-6">
      <div className="mb-4 flex flex-wrap items-start justify-between gap-3">
        <div>
          <div className="flex items-center gap-2">
            <KeyRound className="h-5 w-5 text-primary" />
            <h2 className="text-lg font-semibold">AI Provider</h2>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">
            Agent tasks call {status?.provider ?? "the AI provider"} from the Rust backend. The key
            is stored in your operating system keychain and never leaves this computer.
          </p>
        </div>
        <Badge variant={configured ? "default" : "outline"}>
          {configured ? "Configured" : "Not configured"}
        </Badge>
      </div>

      {status && <p className="mb-4 text-xs text-muted-foreground">Model: {status.model}</p>}

      <form onSubmit={handleSubmit} className="flex flex-col gap-2 sm:flex-row">
        <label htmlFor="provider-api-key" className="sr-only">
          {status?.provider ?? "Provider"} API key
        </label>
        <input
          id="provider-api-key"
          type="password"
          autoComplete="off"
          spellCheck={false}
          value={apiKey}
          onChange={(event) => {
            setApiKey(event.target.value);
          }}
          placeholder={configured ? "Enter a replacement key" : "sk-..."}
          className="flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
        />
        <button
          type="submit"
          disabled={!apiKey.trim() || isSavingKey}
          className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground transition-colors hover:bg-primary/90 disabled:opacity-50"
        >
          {isSavingKey ? "Saving..." : "Save key"}
        </button>
      </form>

      {configured && (
        <div className="mt-4 flex flex-wrap items-center justify-between gap-3 border-t border-border pt-4">
          <p className="text-xs text-muted-foreground">
            Removing the key stops agent tasks until a new key is saved.
          </p>
          <button
            type="button"
            onClick={handleClear}
            disabled={clearApiKey.isPending}
            className="rounded-md border border-destructive/50 px-3 py-1.5 text-xs font-medium text-destructive transition-colors hover:bg-destructive/10 disabled:opacity-50"
          >
            {clearApiKey.isPending ? "Removing..." : "Remove key"}
          </button>
        </div>
      )}
    </section>
  );
}
