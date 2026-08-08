/**
 * Journal Page
 *
 * Serves as the Investment Thesis management interface.
 * Journal entries are represented as Thesis objects with evidence tracking.
 * This page provides a unified view for managing investment theses,
 * tracking confidence changes, and maintaining evidence records.
 */
import { ThesisDashboard } from "@/features/thesis";
import { useLocale } from "@/lib/i18n/useLocale";

export function JournalPage() {
  const { t } = useLocale();
  
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">{t("thesisManagement")}</h1>
      <p className="mt-1 text-sm text-muted-foreground">
        {t("thesisManagementDescription")}
      </p>
      <div className="mt-6">
        <ThesisDashboard />
      </div>
    </div>
  );
}
