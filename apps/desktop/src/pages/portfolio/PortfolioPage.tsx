import { PortfolioDashboard } from "@/features/portfolio/components/PortfolioDashboard";
import { useLocale } from "@/lib/i18n/useLocale";

export function PortfolioPage() { 
  const { t } = useLocale();
  return <div className="space-y-6 p-6"><h1 className="text-2xl font-bold">{t("portfolioTitle")}</h1><PortfolioDashboard /></div>; 
}
