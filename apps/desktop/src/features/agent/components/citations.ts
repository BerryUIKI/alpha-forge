// Citation parsing for stored task output.

export interface OutputCitation {
  kind: "document" | "task";
  id: string;
  title: string;
}

const MAX_CITATIONS = 20;

export function parseCitations(output: string | null): OutputCitation[] {
  if (!output) {
    return [];
  }

  let value: unknown;
  try {
    value = JSON.parse(output);
  } catch {
    return [];
  }

  if (typeof value !== "object" || value === null) {
    return [];
  }

  const citations = (value as { citations?: unknown }).citations;
  if (!Array.isArray(citations)) {
    return [];
  }

  return citations.filter(isCitation).slice(0, MAX_CITATIONS);
}

function isCitation(value: unknown): value is OutputCitation {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const citation = value as Record<string, unknown>;
  return (
    (citation.kind === "document" || citation.kind === "task") &&
    typeof citation.id === "string" &&
    citation.id.trim().length > 0 &&
    typeof citation.title === "string" &&
    citation.title.trim().length > 0
  );
}
