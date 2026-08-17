/**
 * ThesesPage
 *
 * Alias for JournalPage — serves as the Investment Thesis management interface.
 * Theses are represented as Thesis objects with evidence tracking.
 *
 * @version GUI-M1
 */

import { ThesisDashboard } from "@/features/thesis";
import { useLocale } from "@/lib/i18n/useLocale";

export function ThesesPage() {
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