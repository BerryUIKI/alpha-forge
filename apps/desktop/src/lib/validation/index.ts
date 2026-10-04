/**
 * Central Zod validation schemas module.
 *
 * Re-exports domain and API validation schemas across AlphaForge modules.
 *
 * @module lib/validation
 */

export {
  WorkspaceSchema,
} from "../desktop-api/workspace";

export {
  InvestmentThesisSchema,
  ThesisEvidenceSchema,
  ThesisStatusSchema,
  EvidenceDirectionSchema,
} from "../desktop-api/thesis";

export {
  AssetSchema,
  QuoteSchema,
  ActivitySchema,
  HoldingSchema,
  HoldingsSummarySchema,
  DailyAccountValuationSchema,
  PerformancePointSchema,
  PerformanceSummarySchema,
} from "../desktop-api/financial";
