// Research document labels.

import type { ResearchDocumentKind } from "@/lib/desktop-api";

const kindLabels: Record<ResearchDocumentKind, string> = {
  note: "Note",
  web: "Web",
  pdf: "PDF",
  agent: "Agent",
};

export function researchKindLabel(kind: ResearchDocumentKind | string): string {
  return kindLabels[kind as ResearchDocumentKind] ?? kind;
}
