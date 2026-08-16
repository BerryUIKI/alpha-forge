/**
 * AccountManagement — Stub module
 *
 * TODO: Implement account management components for portfolio feature.
 * These stubs prevent typecheck failures while the feature is incomplete.
 */

import type { PortfolioAccount } from "@/lib/desktop-api/portfolio";

interface CreateAccountFormProps {
  workspaceId: string;
  onCreated: (id: string) => void;
}

export function CreateAccountForm(_props: CreateAccountFormProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Create Account form (coming soon)</div>;
}

interface AccountListProps {
  workspaceId: string;
  selectedAccountId: string;
  onSelect: (account: PortfolioAccount) => void;
}

export function AccountList(_props: AccountListProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Account list (coming soon)</div>;
}

interface PositionPanelProps {
  account: PortfolioAccount;
}

export function PositionPanel(_props: PositionPanelProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Position panel (coming soon)</div>;
}