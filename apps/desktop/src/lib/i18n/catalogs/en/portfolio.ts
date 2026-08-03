/**
 * English portfolio messages catalog.
 */

export const portfolio = {
  // Page header
  portfolioTitle: "Portfolio",
  portfolioDescription: "Track holdings for review and risk analysis. This workspace never executes trades.",
  
  // Workspace
  workspace: "Workspace",
  
  // Account form
  newAccount: "New account",
  newAccountDescription: "Add a source of holdings for tracking only.",
  accountName: "Account name",
  accountNamePlaceholder: "Primary brokerage",
  accountType: "Account type",
  accountTypeBrokerage: "Brokerage",
  accountTypeRetirement: "Retirement",
  accountTypeCash: "Cash",
  accountTypeOther: "Other",
  accountCurrency: "Account currency",
  creating: "Creating…",
  addAccount: "Add account",
  accountNameRequired: "Account name is required.",
  unableToCreateAccount: "Unable to create the account.",
  
  // Account list
  accounts: "Accounts",
  noAccountsYet: "No accounts yet",
  noAccountsDescription: "Add an account to start tracking holdings.",
  
  // Positions
  symbol: "Symbol",
  quantity: "Quantity",
  costBasis: "Cost basis",
  addHolding: "Add holding",
  adding: "Adding…",
  noHoldingsYet: "No holdings yet",
  noHoldingsDescription: "Record a holding manually, or import your transaction history below.",
  failedToLoadHoldings: "Failed to load holdings.",
  holdingValidationError: "Enter a symbol, a non-zero quantity, and an optional numeric cost basis.",
  unableToAddHolding: "Unable to add the holding.",
  
  // Transactions import
  importTransactionHistory: "Import transaction history",
  importDescription: "Paste CSV with exactly: symbol, transaction_type (buy/sell), quantity, price, executed_at (RFC 3339). Importing records history only; it never sends a trade.",
  transactionCsv: "Transaction CSV",
  csvPlaceholder: "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z",
  importTransactions: "Import transactions",
  importing: "Importing…",
  unableToImportTransactions: "Unable to import transactions.",
  importedTransactions: "Imported transactions",
  noTransactionHistory: "No transaction history has been imported.",
  
  // Allocation
  costBasisAllocation: "Cost-basis allocation",
  allocationDescription: "Exposure uses recorded cost basis, not live market prices. Positions without a cost basis contribute zero until priced.",
  addHoldingsPrompt: "Add holdings with a cost basis to see allocation and concentration.",
  recordedCost: "Recorded cost",
  
  // Concentration
  concentrationReview: "Concentration review",
  concentrationDescription: "A rules-based signal: moderate at 10% and high at 25% of recorded cost allocation. It is not investment advice.",
  noConcentrationRisks: "No positions currently exceed the review thresholds.",
  moderate: "Moderate",
  high: "High",
  
  // Theme exposure
  themeExposure: "Theme exposure",
  themeExposureDescription: "Link a held symbol to an existing knowledge entity. Exposure is based only on your explicit links and recorded cost basis.",
  knowledgeEntity: "Knowledge entity…",
  linkTheme: "Link theme",
  noThemeLinks: "No theme links yet.",
  failedToLoadThemeExposure: "Failed to load theme exposure.",
  unableToLinkTheme: "Unable to link theme.",
  
  // Alignment review
  thesisAlignmentReview: "Thesis alignment and review",
  alignmentDescription: "Matches symbols only when their ticker appears in workspace thesis content. Review results are informational, not recommendations.",
  noThesisAlignment: "No held symbols currently match a workspace thesis.",
  generatePortfolioReview: "Generate portfolio review",
  reviewing: "Reviewing…",
  unableToGenerateReview: "Unable to generate review.",
  reviewGenerated: "Review generated",
  unalignedSymbols: "Unaligned symbols",
  concentrationSignals: "Concentration signals",
  
  // Select account
  selectAccount: "Select an account",
  selectAccountDescription: "Choose an account to review and record its holdings.",
  
  // Errors
  failedToLoadWorkspaces: "Failed to load workspaces.",
  createWorkspaceFirst: "Create a workspace first",
  createWorkspaceFirstDescription: "Portfolio accounts are organized within a workspace.",
  failedToCalculateAllocation: "Failed to calculate allocation.",
  failedToAnalyzeConcentration: "Failed to analyze concentration.",
  failedToCheckThesisAlignment: "Failed to check thesis alignment.",
  failedToLoadTransactions: "Failed to load transactions.",
  failedToLoadPortfolioAccounts: "Failed to load portfolio accounts.",
} as const;

export type PortfolioKey = keyof typeof portfolio;