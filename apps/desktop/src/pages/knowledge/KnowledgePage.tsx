/**
 * KnowledgePage
 *
 * Placeholder page for the Knowledge Graph feature.
 * Will display interconnected research entities, relationships, and insights.
 *
 * @version GUI-M1
 */

import { useLocale } from "@/lib/i18n/useLocale";
import { EmptyState } from "@/components/common/EmptyState";
import { BookOpen } from "lucide-react";

export function KnowledgePage() {
  const { t } = useLocale();

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">{t("knowledgeGraph")}</h1>
      <p className="mt-2 text-sm text-muted-foreground">
        {t("knowledgeGraphDescription")}
      </p>
      <div className="mt-12">
        <EmptyState
          icon={<BookOpen className="h-8 w-8 text-muted-foreground" />}
          title={t("knowledgeGraph")}
          description="Research connections and insights will appear here once you start building your thesis network."
        />
      </div>
    </div>
  );
}