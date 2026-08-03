import { useEffect, useState } from "react";
import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useWorkspaces } from "@/features/workspace/hooks/useWorkspaces";
import { useKnowledgeEntities } from "@/features/thesis/hooks/useKnowledgeGraph";
import { useLocale } from "@/lib/i18n/useLocale";
import type { PortfolioAccount } from "@/lib/desktop-api/portfolio";
import { useCreatePortfolioAccount, useCreatePortfolioPosition, useImportPortfolioTransactions, useLinkPortfolioTheme, usePortfolioAccounts, usePortfolioAllocation, usePortfolioConcentrationRisks, usePortfolioPositions, usePortfolioReview, usePortfolioThemeExposure, usePortfolioThesisAlignment, usePortfolioTransactions } from "../hooks/usePortfolio";

export function PortfolioDashboard() {
  const { t } = useLocale();
  const workspaces = useWorkspaces();
  const [workspaceId, setWorkspaceId] = useState("");
  const [selectedAccountId, setSelectedAccountId] = useState("");
  const accounts = usePortfolioAccounts(workspaceId);
  const selectedAccount = accounts.data?.find((account) => account.id === selectedAccountId);

  useEffect(() => { if (!workspaceId && workspaces.data?.[0]) setWorkspaceId(workspaces.data[0].id); }, [workspaceId, workspaces.data]);
  useEffect(() => { if (!selectedAccountId && accounts.data?.[0]) setSelectedAccountId(accounts.data[0].id); }, [selectedAccountId, accounts.data]);

  if (workspaces.isLoading) return <LoadingSpinner className="p-8" />;
  if (workspaces.error) return <ErrorState message={t("failedToLoadWorkspaces")} onRetry={() => workspaces.refetch()} />;
  if (!workspaces.data?.length) return <EmptyState title={t("createWorkspaceFirst")} description={t("createWorkspaceFirstDescription")} />;

  return (
    <div className="space-y-6">
      <p className="text-muted-foreground">{t("portfolioDescription")}</p>
      <label className="block max-w-sm text-sm font-medium">
        {t("workspaceLabel")}
        <select value={workspaceId} onChange={(event) => { setWorkspaceId(event.target.value); setSelectedAccountId(""); }} className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm">
          {workspaces.data.map((workspace) => <option key={workspace.id} value={workspace.id}>{workspace.name}</option>)}
        </select>
      </label>
      <div className="grid gap-6 lg:grid-cols-[minmax(0,0.8fr)_minmax(0,1.2fr)]">
        <div className="space-y-4">
          <CreateAccountForm workspaceId={workspaceId} onCreated={setSelectedAccountId} />
          <AccountList query={accounts} selectedAccountId={selectedAccountId} onSelect={(account) => setSelectedAccountId(account.id)} />
        </div>
        <div>
          {selectedAccount ? <PositionPanel account={selectedAccount} /> : <EmptyState title={t("selectAnAccount")} description={t("selectAnAccountDescription")} />}
        </div>
      </div>
      <div className="grid gap-6 lg:grid-cols-2">
        <AllocationPanel workspaceId={workspaceId} />
        <ConcentrationPanel workspaceId={workspaceId} />
      </div>
      <ThemeExposurePanel workspaceId={workspaceId} />
      <AlignmentReviewPanel workspaceId={workspaceId} />
    </div>
  );
}

function AlignmentReviewPanel({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const alignment = usePortfolioThesisAlignment(workspaceId);
  const review = usePortfolioReview();
  const [error, setError] = useState("");

  return (
    <section className="rounded-lg border border-border bg-card p-5">
      <h2 className="text-lg font-semibold">{t("thesisAlignmentAndReview")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("alignmentDescription")}</p>
      {alignment.isLoading ? <LoadingSpinner className="p-4" /> :
       alignment.error ? <ErrorState message={t("failedToCheckAlignment")} onRetry={() => alignment.refetch()} /> :
       !alignment.data?.length ? <p className="mt-3 text-sm text-muted-foreground">{t("noAlignmentMatches")}</p> :
       <ul className="mt-3 space-y-1 text-sm">
         {alignment.data.map((item) => (
           <li key={`${item.symbol}-${item.thesis_id}`} className="rounded bg-muted px-3 py-2">
             <span className="font-medium">{item.symbol}</span> · {item.thesis_title} ({item.confidence}% confidence)
           </li>
         ))}
       </ul>}
      <button
        onClick={async () => {
          try {
            setError("");
            await review.mutateAsync(workspaceId);
          } catch (cause) {
            setError(cause instanceof Error ? cause.message : t("failedToCheckAlignment"));
          }
        }}
        disabled={review.isPending}
        className="mt-4 rounded-md border border-input px-3 py-2 text-sm hover:bg-accent disabled:opacity-50"
      >
        {review.isPending ? t("reviewing") : t("generatePortfolioReview")}
      </button>
      {error && <p className="mt-2 text-sm text-destructive">{error}</p>}
      {review.data && (
        <div className="mt-3 rounded bg-muted p-3 text-sm">
          <p className="font-medium">{t("reviewGenerated")} {new Date(review.data.generated_at).toLocaleString()}</p>
          <p className="mt-1">{t("unalignedSymbols")}: {review.data.unaligned_symbols.join(", ") || t("none")}</p>
          <p className="mt-1">{t("concentrationSignals")}: {review.data.concentration_risks.length}</p>
        </div>
      )}
    </section>
  );
}

function ThemeExposurePanel({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const entities = useKnowledgeEntities(workspaceId);
  const exposure = usePortfolioThemeExposure(workspaceId);
  const link = useLinkPortfolioTheme();
  const [symbol, setSymbol] = useState("");
  const [entityId, setEntityId] = useState("");
  const [error, setError] = useState("");

  async function submit(event: React.FormEvent) {
    event.preventDefault();
    try {
      await link.mutateAsync({ workspaceId, symbol, entityId });
      setSymbol("");
      setEntityId("");
      setError("");
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : t("unableToLinkTheme"));
    }
  }

  return (
    <section className="space-y-4 rounded-lg border border-border bg-card p-5">
      <div>
        <h2 className="text-lg font-semibold">{t("themeExposure")}</h2>
        <p className="text-sm text-muted-foreground">{t("themeExposureDescription")}</p>
      </div>
      <form onSubmit={submit} className="flex flex-wrap gap-2">
        <input
          aria-label={t("themeSymbolLabel")}
          value={symbol}
          onChange={(event) => setSymbol(event.target.value)}
          placeholder="Symbol"
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        />
        <select
          aria-label={t("knowledgeEntityLabel")}
          value={entityId}
          onChange={(event) => setEntityId(event.target.value)}
          className="min-w-48 rounded-md border border-input bg-background px-3 py-2 text-sm"
        >
          <option value="">{t("knowledgeEntityPlaceholder")}</option>
          {entities.data?.map((entity) => (
            <option key={entity.id} value={entity.id}>{entity.entity_type}: {entity.name}</option>
          ))}
        </select>
        <button disabled={link.isPending || !symbol.trim() || !entityId} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent disabled:opacity-50">
          {t("linkTheme")}
        </button>
      </form>
      {error && <p className="text-sm text-destructive">{error}</p>}
      {exposure.isLoading ? <LoadingSpinner className="p-4" /> :
       exposure.error ? <ErrorState message={t("failedToLoadThemeExposure")} onRetry={() => exposure.refetch()} /> :
       !exposure.data?.length ? <p className="text-sm text-muted-foreground">{t("noThemeLinks")}</p> :
       <ul className="space-y-2 text-sm">
         {exposure.data.map((item) => (
           <li key={item.entity_id} className="flex justify-between rounded bg-muted px-3 py-2">
             <span>{item.theme_name}</span>
             <span>{item.weight_percent.toFixed(1)}%</span>
           </li>
         ))}
       </ul>}
    </section>
  );
}

function ConcentrationPanel({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const risks = usePortfolioConcentrationRisks(workspaceId);

  if (risks.isLoading) return <LoadingSpinner className="p-6" />;
  if (risks.error) return <ErrorState message={t("failedToAnalyzeConcentration")} onRetry={() => risks.refetch()} />;

  return (
    <section className="rounded-lg border border-border bg-card p-5">
      <h2 className="text-lg font-semibold">{t("concentrationReview")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("concentrationDescription")}</p>
      {!risks.data?.length ? (
        <p className="mt-4 text-sm text-muted-foreground">{t("noConcentrationRisks")}</p>
      ) : (
        <ul className="mt-4 space-y-2">
          {risks.data.map((risk) => (
            <li key={risk.symbol} className={`rounded-md border p-3 text-sm ${risk.severity === "high" ? "border-destructive/40 bg-destructive/5" : "border-amber-500/40 bg-amber-500/5"}`}>
              <span className="font-medium">{risk.severity === "high" ? t("severityHigh") : t("severityModerate")}: </span>
              {risk.message}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

function AllocationPanel({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const allocation = usePortfolioAllocation(workspaceId);

  if (allocation.isLoading) return <LoadingSpinner className="p-6" />;
  if (allocation.error) return <ErrorState message={t("failedToCalculateAllocation")} onRetry={() => allocation.refetch()} />;

  return (
    <section className="rounded-lg border border-border bg-card p-5">
      <h2 className="text-lg font-semibold">{t("costBasisAllocation")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("allocationDescription")}</p>
      {!allocation.data?.length ? (
        <p className="mt-4 text-sm text-muted-foreground">{t("noAllocationData")}</p>
      ) : (
        <div className="mt-4 space-y-3">
          {allocation.data.map((item) => (
            <div key={item.symbol}>
              <div className="flex justify-between text-sm">
                <span className="font-medium">{item.symbol}</span>
                <span>{item.weight_percent.toFixed(1)}% · {item.account_count} account{item.account_count === 1 ? "" : "s"}</span>
              </div>
              <div className="mt-1 h-2 overflow-hidden rounded bg-muted">
                <div className="h-full bg-primary" style={{ width: `${item.weight_percent}%` }} />
              </div>
              <p className="mt-1 text-xs text-muted-foreground">{t("recordedCost")}: {item.allocated_cost.toLocaleString(undefined, { maximumFractionDigits: 2 })}</p>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}

function CreateAccountForm({ workspaceId, onCreated }: { workspaceId: string; onCreated: (id: string) => void }) {
  const { t } = useLocale();
  const createAccount = useCreatePortfolioAccount();
  const [name, setName] = useState("");
  const [accountType, setAccountType] = useState("brokerage");
  const [currency, setCurrency] = useState("USD");
  const [error, setError] = useState("");

  async function submit(event: React.FormEvent) {
    event.preventDefault();
    if (!name.trim()) return setError(t("accountNameRequired"));
    try {
      const account = await createAccount.mutateAsync({ workspaceId, name: name.trim(), accountType, currency: currency.trim().toUpperCase() });
      setName("");
      setError("");
      onCreated(account.id);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : t("unableToCreateAccount"));
    }
  }

  return (
    <form onSubmit={submit} className="space-y-3 rounded-lg border border-border bg-card p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("newAccount")}</h2>
        <p className="text-sm text-muted-foreground">{t("newAccountDescription")}</p>
      </div>
      <input
        aria-label={t("accountNameLabel")}
        value={name}
        onChange={(event) => setName(event.target.value)}
        placeholder={t("accountNamePlaceholder")}
        className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
      />
      <div className="grid grid-cols-2 gap-2">
        <select
          aria-label={t("accountTypeLabel")}
          value={accountType}
          onChange={(event) => setAccountType(event.target.value)}
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        >
          <option value="brokerage">{t("accountTypeBrokerage")}</option>
          <option value="retirement">{t("accountTypeRetirement")}</option>
          <option value="cash">{t("accountTypeCash")}</option>
          <option value="other">{t("accountTypeOther")}</option>
        </select>
        <input
          aria-label={t("accountCurrencyLabel")}
          value={currency}
          maxLength={3}
          onChange={(event) => setCurrency(event.target.value)}
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        />
      </div>
      {error && <p className="text-sm text-destructive">{error}</p>}
      <button disabled={createAccount.isPending} className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50">
        {createAccount.isPending ? t("creatingAccount") : t("addAccount")}
      </button>
    </form>
  );
}

function AccountList({ query, selectedAccountId, onSelect }: { query: ReturnType<typeof usePortfolioAccounts>; selectedAccountId: string; onSelect: (account: PortfolioAccount) => void }) {
  const { t } = useLocale();

  if (query.isLoading) return <LoadingSpinner className="p-6" />;
  if (query.error) return <ErrorState message={t("failedToLoadAccounts")} onRetry={() => query.refetch()} />;
  if (!query.data?.length) return <EmptyState title={t("noAccountsYet")} description={t("noAccountsDescription")} />;

  return (
    <section className="space-y-2">
      <h2 className="font-semibold">{t("accounts")}</h2>
      {query.data.map((account) => (
        <button
          key={account.id}
          onClick={() => onSelect(account)}
          className={`w-full rounded-lg border p-3 text-left ${selectedAccountId === account.id ? "border-primary bg-accent" : "border-border bg-card hover:bg-accent"}`}
        >
          <div className="flex justify-between gap-2">
            <span className="font-medium">{account.name}</span>
            <span className="text-sm text-muted-foreground">{account.currency}</span>
          </div>
          <p className="mt-1 text-sm capitalize text-muted-foreground">{account.account_type}</p>
        </button>
      ))}
    </section>
  );
}

function PositionPanel({ account }: { account: PortfolioAccount }) {
  const { t } = useLocale();
  const positions = usePortfolioPositions(account.id);
  const transactions = usePortfolioTransactions(account.id);
  const createPosition = useCreatePortfolioPosition();
  const importTransactions = useImportPortfolioTransactions();
  const [symbol, setSymbol] = useState("");
  const [quantity, setQuantity] = useState("");
  const [costBasis, setCostBasis] = useState("");
  const [error, setError] = useState("");

  async function submit(event: React.FormEvent) {
    event.preventDefault();
    const parsedQuantity = Number(quantity);
    const parsedCostBasis = costBasis ? Number(costBasis) : undefined;
    if (!symbol.trim() || !Number.isFinite(parsedQuantity) || parsedQuantity === 0 || (parsedCostBasis !== undefined && !Number.isFinite(parsedCostBasis))) {
      return setError(t("invalidHoldingInput"));
    }
    try {
      await createPosition.mutateAsync({ accountId: account.id, symbol: symbol.trim().toUpperCase(), quantity: parsedQuantity, costBasis: parsedCostBasis });
      setSymbol("");
      setQuantity("");
      setCostBasis("");
      setError("");
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : t("unableToAddHolding"));
    }
  }

  const [csvText, setCsvText] = useState("");
  const [importError, setImportError] = useState("");

  async function importCsv(event: React.FormEvent) {
    event.preventDefault();
    try {
      await importTransactions.mutateAsync({ accountId: account.id, csvText });
      setCsvText("");
      setImportError("");
    } catch (cause) {
      setImportError(cause instanceof Error ? cause.message : t("unableToImportTransactions"));
    }
  }

  return (
    <section className="space-y-5 rounded-lg border border-border bg-card p-5">
      <div>
        <p className="text-sm text-muted-foreground">{account.account_type} · {account.currency}</p>
        <h2 className="text-xl font-semibold">{account.name}</h2>
      </div>
      <form onSubmit={submit} className="grid gap-2 sm:grid-cols-4">
        <input
          aria-label={t("symbolLabel")}
          value={symbol}
          onChange={(event) => setSymbol(event.target.value)}
          placeholder="Symbol"
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        />
        <input
          aria-label={t("quantityLabel")}
          value={quantity}
          onChange={(event) => setQuantity(event.target.value)}
          placeholder="Quantity"
          inputMode="decimal"
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        />
        <input
          aria-label={t("costBasisLabel")}
          value={costBasis}
          onChange={(event) => setCostBasis(event.target.value)}
          placeholder="Cost basis"
          inputMode="decimal"
          className="rounded-md border border-input bg-background px-3 py-2 text-sm"
        />
        <button disabled={createPosition.isPending} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent disabled:opacity-50">
          {createPosition.isPending ? t("addingHolding") : t("addHolding")}
        </button>
      </form>
      {error && <p className="text-sm text-destructive">{error}</p>}
      {positions.isLoading ? <LoadingSpinner className="p-6" /> :
       positions.error ? <ErrorState message={t("failedToLoadHoldings")} onRetry={() => positions.refetch()} /> :
       !positions.data?.length ? <EmptyState title={t("noHoldingsYet")} description={t("noHoldingsDescription")} /> :
       <div className="overflow-x-auto">
         <table className="w-full text-left text-sm">
           <thead className="border-b border-border text-muted-foreground">
             <tr>
               <th className="p-2">Symbol</th>
               <th className="p-2">Quantity</th>
               <th className="p-2">Cost basis</th>
             </tr>
           </thead>
           <tbody>
             {positions.data.map((position) => (
               <tr key={position.id} className="border-b border-border">
                 <td className="p-2 font-medium">{position.symbol}</td>
                 <td className="p-2">{position.quantity}</td>
                 <td className="p-2">{position.cost_basis == null ? "—" : position.cost_basis.toLocaleString(undefined, { style: "currency", currency: account.currency })}</td>
               </tr>
             ))}
           </tbody>
         </table>
       </div>}
      <form onSubmit={importCsv} className="space-y-2 border-t border-border pt-5">
        <div>
          <h3 className="font-semibold">{t("importTransactionHistory")}</h3>
          <p className="text-sm text-muted-foreground">{t("importDescription")}</p>
        </div>
        <textarea
          aria-label={t("transactionCsvLabel")}
          value={csvText}
          onChange={(event) => setCsvText(event.target.value)}
          rows={5}
          placeholder={t("transactionCsvPlaceholder")}
          className="w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-sm"
        />
        {importError && <p className="text-sm text-destructive">{importError}</p>}
        <button disabled={importTransactions.isPending || !csvText.trim()} className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent disabled:opacity-50">
          {importTransactions.isPending ? t("importingTransactions") : t("importTransactions")}
        </button>
      </form>
      <div className="border-t border-border pt-5">
        <h3 className="font-semibold">{t("importedTransactions")}</h3>
        {transactions.isLoading ? <LoadingSpinner className="p-4" /> :
         transactions.error ? <ErrorState message={t("failedToLoadTransactions")} onRetry={() => transactions.refetch()} /> :
         !transactions.data?.length ? <p className="mt-2 text-sm text-muted-foreground">{t("noTransactionsImported")}</p> :
         <ul className="mt-2 space-y-1 text-sm">
           {transactions.data.map((transaction) => (
             <li key={transaction.id} className="flex justify-between rounded bg-muted px-3 py-2">
               <span>{transaction.symbol} · {transaction.transaction_type} · {transaction.quantity} @ {transaction.price}</span>
               <time className="text-muted-foreground">{new Date(transaction.executed_at).toLocaleDateString()}</time>
             </li>
           ))}
         </ul>}
      </div>
    </section>
  );
}