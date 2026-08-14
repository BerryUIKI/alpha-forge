-- Financial domain schema (Phase 1 storage) — daily valuation uniqueness.
--
-- `daily_account_valuation` is a per-account daily time series; exactly one
-- row per (account_id, valuation_date), matching the Wealthfolio read model
-- (primary key on (account_id, date)). The unique index was deferred to this
-- append-only migration so repositories can upsert with `ON CONFLICT`.
-- 0019 created the table; this adds the integrity constraint without touching
-- that (already authored) migration.

CREATE UNIQUE INDEX IF NOT EXISTS ux_daily_account_valuation_account_date
    ON daily_account_valuation(account_id, valuation_date);
