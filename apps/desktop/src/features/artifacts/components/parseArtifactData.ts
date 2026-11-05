// Parsing for artifact payloads.
//
// The Rust backend validates payloads before storing them, but the UI still
// checks shapes so an unexpected payload renders a fallback instead of throwing.

export type ArtifactCellValue = string | number | boolean | null;

export interface ComparisonColumn {
  key: string;
  label: string;
}

export interface ComparisonRow {
  label: string;
  values: Record<string, ArtifactCellValue>;
}

export interface ComparisonTableData {
  columns: ComparisonColumn[];
  rows: ComparisonRow[];
}

export interface TimelineEvent {
  date: string;
  title: string;
  description: string | null;
}

export interface TimelineData {
  events: TimelineEvent[];
}

export type ParsedArtifactData =
  | { kind: "comparison-table"; data: ComparisonTableData }
  | { kind: "timeline"; data: TimelineData }
  | { kind: "unsupported"; reason: string };

export const COMPARISON_TABLE_TYPE = "comparison-table";
export const TIMELINE_TYPE = "timeline";

export function parseArtifactData(artifactType: string, json: string): ParsedArtifactData {
  let value: unknown;

  try {
    value = JSON.parse(json);
  } catch {
    return { kind: "unsupported", reason: "Artifact data is not valid JSON." };
  }

  if (artifactType === COMPARISON_TABLE_TYPE) {
    const data = parseComparisonTable(value);
    return data
      ? { kind: COMPARISON_TABLE_TYPE, data }
      : { kind: "unsupported", reason: "Comparison table data has an unexpected shape." };
  }

  if (artifactType === TIMELINE_TYPE) {
    const data = parseTimeline(value);
    return data
      ? { kind: TIMELINE_TYPE, data }
      : { kind: "unsupported", reason: "Timeline data has an unexpected shape." };
  }

  return { kind: "unsupported", reason: `Unsupported artifact type "${artifactType}".` };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseComparisonTable(value: unknown): ComparisonTableData | null {
  if (!isRecord(value) || !Array.isArray(value.columns) || !Array.isArray(value.rows)) {
    return null;
  }

  const columns: ComparisonColumn[] = [];
  for (const column of value.columns) {
    if (!isRecord(column) || typeof column.key !== "string" || typeof column.label !== "string") {
      return null;
    }
    columns.push({ key: column.key, label: column.label });
  }

  const rows: ComparisonRow[] = [];
  for (const row of value.rows) {
    if (!isRecord(row) || typeof row.label !== "string") {
      return null;
    }

    const values: Record<string, ArtifactCellValue> = {};
    if (row.values !== undefined) {
      if (!isRecord(row.values)) {
        return null;
      }
      for (const [key, cell] of Object.entries(row.values)) {
        if (
          cell !== null &&
          typeof cell !== "string" &&
          typeof cell !== "number" &&
          typeof cell !== "boolean"
        ) {
          return null;
        }
        values[key] = cell;
      }
    }

    rows.push({ label: row.label, values });
  }

  if (columns.length === 0 || rows.length === 0) {
    return null;
  }

  return { columns, rows };
}

function parseTimeline(value: unknown): TimelineData | null {
  if (!isRecord(value) || !Array.isArray(value.events)) {
    return null;
  }

  const events: TimelineEvent[] = [];
  for (const event of value.events) {
    if (!isRecord(event) || typeof event.date !== "string" || typeof event.title !== "string") {
      return null;
    }

    const description =
      typeof event.description === "string" && event.description.trim() ? event.description : null;

    events.push({ date: event.date, title: event.title, description });
  }

  if (events.length === 0) {
    return null;
  }

  return { events };
}
