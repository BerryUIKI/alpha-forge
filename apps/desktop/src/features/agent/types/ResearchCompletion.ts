import { z } from "zod";

export const ResearchCompletionSchema = z.object({
  summary: z.string().min(1),
  claims: z.array(z.string()),
  evidence: z.array(z.string()),
  risks: z.array(z.string()),
  confidence: z.number().min(0).max(100),
});

export type ResearchCompletion = z.infer<typeof ResearchCompletionSchema>;

export function parseResearchCompletion(payload: string | null): ResearchCompletion | null {
  if (!payload) return null;
  try {
    const parsedJson = JSON.parse(payload);
    const result = ResearchCompletionSchema.safeParse(parsedJson);
    if (result.success) {
      return result.data;
    }
    return null;
  } catch {
    return null;
  }
}
