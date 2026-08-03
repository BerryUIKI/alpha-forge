import { useLocale } from "@/lib/i18n/useLocale";

export function ArtifactsPage() {
  const { t } = useLocale();
  
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">{t("artifactsTitle")}</h1>
    </div>
  );
}
