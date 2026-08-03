/**
 * English portfolio messages catalog.
 */

export const portfolio = {
  // Page header
  portfolioTitle: "Portfolio",
  portfolioDescription: "Track holdings for review and risk analysis. This workspace never executes trades.",

  // Workspace selection
  workspaceLabel: "Workspace",

  // Create account form
  newAccount: "New account",
  newAccountDescription: "Add a source of holdings for tracking only.",
  accountNameLabel: "Account name",
  accountNamePlaceholder: "Primary brokerage",
  accountTypeLabel: "Account type",
  accountCurrencyLabel: "Account currency",
  accountTypeBrokerage: "Brokerage",
  accountTypeRetirement: "Retirement",
  accountTypeCash: "Cash",
  accountTypeOther: "Other",
  creatingAccount: "Creating…",
  addAccount: "Add account",
  accountNameRequired: "Account name is required.",
  unableToCreateAccount: "Unable to create the account.",

  // Account list
  accounts: "Accounts",
  noAccountsYet: "No accounts yet",
  noAccountsDescription: "Add an account to start tracking holdings.",
  failedToLoadAccounts: "Failed to load portfolio accounts.",

  // Position panel
  symbolLabel: "Symbol",
  quantityLabel: "Quantity",
  costBasisLabel: "Cost basis",
  addHolding: "Add holding",
  addingHolding: "Adding…",
  noHoldingsYet: "No holdings yet",
  noHoldingsDescription: "Record a holding manually, or import your transaction history below.",
  failedToLoadHoldings: "Failed to load holdings.",
  invalidHoldingInput: "Enter a symbol, a non-zero quantity, and an optional numeric cost basis.",
  unableToAddHolding: "Unable to add the holding.",

  // Transaction import
  importTransactionHistory: "Import transaction history",
  importDescription: "Paste CSV with exactly: symbol, transaction_type (buy/sell), quantity, price, executed_at (RFC 3339). Importing records history only; it never sends a trade.",
  transactionCsvLabel: "Transaction CSV",
  transactionCsvPlaceholder: "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z",
  importingTransactions: "Importing…",
  importTransactions: "Import transactions",
  unableToImportTransactions: "Unable to import transactions.",

  // Imported transactions
  importedTransactions: "Imported transactions",
  noTransactionsImported: "No transaction history has been imported.",
  failedToLoadTransactions: "Failed to load transactions.",

  // Allocation panel
  costBasisAllocation: "Cost-basis allocation",
  allocationDescription: "Exposure uses recorded cost basis, not live market prices. Positions without a cost basis contribute zero until priced.",
  noAllocationData: "Add holdings with a cost basis to see allocation and concentration.",
  failedToCalculateAllocation: "Failed to calculate allocation.",
  recordedCost: "Recorded cost",

  // Concentration panel
  concentrationReview: "Concentration review",
  concentrationDescription: "A rules-based signal: moderate at 10% and high at 25% of recorded cost allocation. It is not investment advice.",
  noConcentrationRisks: "No positions currently exceed the review thresholds.",
  failedToAnalyzeConcentration: "Failed to analyze concentration.",
  severityModerate: "moderate",
  severityHigh: "high",

  // Theme exposure panel
  themeExposure: "Theme exposure",
  themeExposureDescription: "Link a held symbol to an existing knowledge entity. Exposure is based only on your explicit links and recorded cost basis.",
  themeSymbolLabel: "Theme symbol",
  knowledgeEntityLabel: "Knowledge entity",
  knowledgeEntityPlaceholder: "Knowledge entity…",
  linkTheme: "Link theme",
  noThemeLinks: "No theme links yet.",
  failedToLoadThemeExposure: "Failed to load theme exposure.",
  unableToLinkTheme: "Unable to link theme.",

  // Alignment review panel
  thesisAlignmentAndReview: "Thesis alignment and review",
  alignmentDescription: "Matches symbols only when their ticker appears in workspace thesis content. Review results are informational, not recommendations.",
  noAlignmentMatches: "No held symbols currently match a workspace thesis.",
  failedToCheckAlignment: "Failed to check thesis alignment.",
  reviewing: "Reviewing…",
  generatePortfolioReview: "Generate portfolio review",
  reviewGenerated: "Review generated",
  unalignedSymbols: "Unaligned symbols",
  concentrationSignals: "Concentration signals",
  none: "None",

  // Common
  selectAnAccount: "Select an account",
  selectAnAccountDescription: "Choose an account to review and record its holdings.",
  failedToLoadWorkspaces: "Failed to load workspaces.",
  createWorkspaceFirst: "Create a workspace first",
  createWorkspaceFirstDescription: "Portfolio accounts are organized within a workspace.",
} as const;

export type PortfolioKey = keyof typeof portfolio;