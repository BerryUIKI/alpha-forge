import { ThesisDashboard } from "@/features/thesis";
import { useLocale } from "@/lib/i18n/useLocale";

export function JournalPage() {
  const { t } = useLocale();
  
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">{t("journalTitle")}</h1>
      <div className="mt-6">
        <ThesisDashboard />
      </div>
    </div>
  );
}
