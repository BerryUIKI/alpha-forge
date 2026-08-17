/**
 * Analysis — Stub module
 *
 * TODO: Implement analysis panel components for portfolio feature.
 * These stubs prevent typecheck failures while the feature is incomplete.
 */

interface PanelProps {
  workspaceId: string;
}

export function AllocationPanel(_props: PanelProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Allocation analysis (coming soon)</div>;
}

export function ConcentrationPanel(_props: PanelProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Concentration analysis (coming soon)</div>;
}

export function ThemeExposurePanel(_props: PanelProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Theme exposure analysis (coming soon)</div>;
}

export function AlignmentReviewPanel(_props: PanelProps) {
  return <div className="rounded-lg border border-border p-4 text-sm text-muted-foreground">Alignment review (coming soon)</div>;
}