import { useState, type FormEvent } from "react";
import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router-dom";

import { EmptyState, ErrorState, LoadingSpinner } from "@/components/common";
import { useCreateCompanyComparisonArtifact, useStartViewingArtifact } from "@/features/artifacts";
import { desktopApi } from "@/lib/desktop-api";
import {
  COMPANY_COMPARISON_DIMENSIONS,
  COMPANY_COMPARISON_PLUGIN_ID,
  CompanyComparisonPayloadSchema,
  type CompanyComparisonDimension,
} from "@/lib/desktop-api/plugins";
import { useLocale } from "@/lib/i18n/useLocale";

export function CompanyComparisonArtifactForm({
  workspaceId,
  onArtifactCreated,
}: {
  workspaceId: string;
  onArtifactCreated: (artifactId: string) => void;
}) {
  const { t } = useLocale();
  const pluginsQuery = useQuery({
    queryKey: ["internalPlugins"],
    queryFn: desktopApi.plugins.listPlugins,
  });
  const createArtifact = useCreateCompanyComparisonArtifact();
  const openArtifact = useStartViewingArtifact();
  const [companies, setCompanies] = useState<Array<{ ticker: string; value: string }>>([
    { ticker: "", value: "" },
    { ticker: "", value: "" },
  ]);
  const [dimension, setDimension] = useState<CompanyComparisonDimension>("revenue");
  const [validationError, setValidationError] = useState(false);

  const plugin = pluginsQuery.data?.find(
    ({ manifest }) => manifest.id === COMPANY_COMPARISON_PLUGIN_ID,
  );

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    createArtifact.reset();
    openArtifact.reset();

    if (companies.some(({ value }) => !value.trim())) {
      setValidationError(true);
      return;
    }
    const parsed = CompanyComparisonPayloadSchema.safeParse({
      companies: companies.map(({ ticker, value }) => ({
        ticker,
        name: ticker.trim().toUpperCase(),
        metrics: { [dimension]: Number(value) },
      })),
      comparisonDimensions: [dimension],
    });
    if (!parsed.success) {
      setValidationError(true);
      return;
    }
    setValidationError(false);

    try {
      const artifact = await createArtifact.mutateAsync({ workspaceId, input: parsed.data });
      onArtifactCreated(artifact.id);
      await openArtifact.mutateAsync(artifact.id).catch(() => undefined);
    } catch {
      // React Query exposes the localized create error below.
    }
  };

  const retryOpen = () => {
    if (createArtifact.data) {
      void openArtifact.mutateAsync(createArtifact.data.id).catch(() => undefined);
    }
  };

  const updateCompany = (index: number, field: "ticker" | "value", value: string) => {
    setCompanies((current) =>
      current.map((company, companyIndex) =>
        companyIndex === index ? { ...company, [field]: value } : company,
      ),
    );
  };

  return (
    <section
      className="border-b border-border bg-muted/20 px-6 py-4"
      aria-labelledby="comparison-title"
    >
      <h2 id="comparison-title" className="font-semibold">
        {t("createCompanyComparison")}
      </h2>
      <p className="mt-1 text-sm text-muted-foreground">
        {t("createCompanyComparisonDescription")}
      </p>

      {pluginsQuery.isLoading && (
        <div className="mt-4 flex items-center gap-2 text-sm" role="status">
          <LoadingSpinner size="sm" />
          {t("loadingCompanyComparisonPlugin")}
        </div>
      )}
      {pluginsQuery.isError && (
        <ErrorState
          message={t("failedToLoadCompanyComparisonPlugin")}
          retryLabel={t("retry")}
          onRetry={() => void pluginsQuery.refetch()}
        />
      )}
      {pluginsQuery.isSuccess && (!plugin || !plugin.enabled) && (
        <EmptyState
          title={t("companyComparisonPluginDisabled")}
          description={t("companyComparisonPluginDisabledDescription")}
          action={
            <Link
              className="rounded-md border border-input px-3 py-2 text-sm font-medium"
              to="/settings#internal-plugins"
            >
              {t("manageInternalPlugins")}
            </Link>
          }
        />
      )}
      {plugin?.enabled && (
        <form className="mt-4 grid gap-3 lg:grid-cols-3" onSubmit={handleSubmit}>
          {companies.map((company, index) => (
            <fieldset key={index} className="grid grid-cols-2 gap-3">
              <label className="text-sm font-medium">
                {t(index === 0 ? "firstCompanyTicker" : "secondCompanyTicker")}
                <input
                  value={company.ticker}
                  onChange={(event) => updateCompany(index, "ticker", event.target.value)}
                  placeholder={index === 0 ? "AAPL" : "MSFT"}
                  className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2"
                />
              </label>
              <label className="text-sm font-medium">
                {t(index === 0 ? "firstCompanyMetric" : "secondCompanyMetric")}
                <input
                  value={company.value}
                  onChange={(event) => updateCompany(index, "value", event.target.value)}
                  placeholder={index === 0 ? "100" : "120"}
                  inputMode="decimal"
                  className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2"
                />
              </label>
            </fieldset>
          ))}
          <label className="text-sm font-medium">
            {t("comparisonDimension")}
            <select
              value={dimension}
              onChange={(event) => {
                if (isComparisonDimension(event.target.value)) {
                  setDimension(event.target.value);
                }
              }}
              className="mt-1 w-full rounded-md border border-input bg-background px-3 py-2"
            >
              <option value="revenue">{t("comparisonDimensionRevenue")}</option>
              <option value="market_cap">{t("comparisonDimensionMarketCap")}</option>
              <option value="pe_ratio">{t("comparisonDimensionPeRatio")}</option>
            </select>
          </label>
          <div className="lg:col-span-3">
            <button
              type="submit"
              disabled={createArtifact.isPending || openArtifact.isPending}
              className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground disabled:opacity-50"
            >
              {createArtifact.isPending || openArtifact.isPending
                ? t("creatingCompanyComparison")
                : t("createAndOpenCompanyComparison")}
            </button>
          </div>
          {validationError && (
            <p className="text-sm text-destructive lg:col-span-3" role="alert">
              {t("invalidCompanyComparison")}
            </p>
          )}
          {createArtifact.isError && (
            <p className="text-sm text-destructive lg:col-span-3" role="alert">
              {t("failedToCreateCompanyComparison")}
            </p>
          )}
          {openArtifact.isError && createArtifact.data && (
            <div
              className="flex items-center gap-3 text-sm text-destructive lg:col-span-3"
              role="alert"
            >
              <span>{t("companyComparisonCreatedOpenFailed")}</span>
              <button type="button" className="underline" onClick={retryOpen}>
                {t("retryOpenArtifact")}
              </button>
            </div>
          )}
        </form>
      )}
    </section>
  );
}

function isComparisonDimension(value: string): value is CompanyComparisonDimension {
  return COMPANY_COMPARISON_DIMENSIONS.some((dimension) => dimension === value);
}
