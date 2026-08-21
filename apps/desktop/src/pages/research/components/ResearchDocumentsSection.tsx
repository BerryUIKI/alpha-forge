/**
 * Research Documents Section
 *
 * Document management (create / import PDF / import web page) and report
 * creation for the selected research project.
 *
 * @module pages/research/components/ResearchDocumentsSection
 */

import { Trash2 } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import type {
  ResearchDocument,
  ResearchReport,
} from "@/lib/desktop-api/research";

export type ReportType = "analysis" | "summary" | "thesis" | "recommendation";

interface ResearchDocumentsSectionProps {
  documents: ResearchDocument[] | undefined;
  documentTitle: string;
  onDocumentTitleChange: (value: string) => void;
  createDocumentPending: boolean;
  onCreateDocument: () => void;
  onSelectDocument: (id: string) => void;
  importPdfPending: boolean;
  onImportPdf: () => void;
  webPageUrl: string;
  onWebPageUrlChange: (value: string) => void;
  importWebPagePending: boolean;
  onImportWebPage: () => void;
  reports: ResearchReport[] | undefined;
  reportTitle: string;
  onReportTitleChange: (value: string) => void;
  reportContent: string;
  onReportContentChange: (value: string) => void;
  reportType: ReportType;
  onReportTypeChange: (value: ReportType) => void;
  createReportPending: boolean;
  onCreateReport: () => void;
  deleteDocument: (id: string) => void;
  deleteReport: (id: string) => void;
}

/**
 * Two-column section: document management on the left, report creation and
 * listing on the right.
 */
export function ResearchDocumentsSection({
  documents,
  documentTitle,
  onDocumentTitleChange,
  createDocumentPending,
  onCreateDocument,
  onSelectDocument,
  importPdfPending,
  onImportPdf,
  webPageUrl,
  onWebPageUrlChange,
  importWebPagePending,
  onImportWebPage,
  reports,
  reportTitle,
  onReportTitleChange,
  reportContent,
  onReportContentChange,
  reportType,
  onReportTypeChange,
  createReportPending,
  onCreateReport,
  deleteDocument,
  deleteReport,
}: ResearchDocumentsSectionProps) {
  const { t } = useLocale();

  return (
    <section className="grid gap-4 rounded-lg border p-4 lg:grid-cols-2">
      <div className="space-y-3">
        <h2 className="font-semibold">{t("documents")}</h2>
        <form
          className="flex gap-2"
          onSubmit={(event) => {
            event.preventDefault();
            onCreateDocument();
          }}
        >
          <input
            aria-label={t("documentTitle")}
            className="flex-1 rounded border bg-background p-2"
            placeholder={t("documentTitle")}
            value={documentTitle}
            onChange={(event) => onDocumentTitleChange(event.target.value)}
          />
          <button
            className="rounded border px-3"
            disabled={!documentTitle.trim() || createDocumentPending}
          >
            {t("add")}
          </button>
        </form>
        <button
          className="rounded border px-3 py-1 text-sm"
          onClick={onImportPdf}
          disabled={importPdfPending}
        >
          {importPdfPending ? t("importingPdf") : t("importPdf")}
        </button>
        <form
          className="flex gap-2"
          onSubmit={(event) => {
            event.preventDefault();
            onImportWebPage();
          }}
        >
          <input
            aria-label={t("webPageUrl")}
            className="flex-1 rounded border bg-background p-2"
            placeholder={t("webPageUrlPlaceholder")}
            value={webPageUrl}
            onChange={(event) => onWebPageUrlChange(event.target.value)}
          />
          <button
            className="rounded border px-3"
            disabled={!webPageUrl.trim() || importWebPagePending}
          >
            {importWebPagePending ? t("importing") : t("importWebPage")}
          </button>
        </form>
        <p className="text-xs text-muted-foreground">{t("importHint")}</p>
        <div className="flex flex-wrap gap-2">
          {documents?.map((item) => (
            <div key={item.id} className="flex items-center gap-1">
              <button
                className="rounded border px-3 py-1 text-sm"
                onClick={() => onSelectDocument(item.id)}
              >
                {item.title}
              </button>
              <button
                onClick={() => deleteDocument(item.id)}
                className="rounded p-1 hover:bg-accent"
                title={t("deleteDocument")}
              >
                <Trash2 className="h-3 w-3 text-destructive" />
              </button>
            </div>
          ))}
        </div>
      </div>

      <div className="space-y-3">
        <h2 className="font-semibold">{t("reports")}</h2>
        <form
          className="space-y-2"
          onSubmit={(event) => {
            event.preventDefault();
            onCreateReport();
          }}
        >
          <input
            aria-label={t("reportTitle")}
            className="w-full rounded border bg-background p-2"
            placeholder={t("reportTitle")}
            value={reportTitle}
            onChange={(event) => onReportTitleChange(event.target.value)}
          />
          <textarea
            aria-label={t("reportContent")}
            className="w-full rounded border bg-background p-2"
            placeholder={t("reportContent")}
            value={reportContent}
            onChange={(event) => onReportContentChange(event.target.value)}
          />
          <select
            aria-label={t("reportType")}
            className="rounded border bg-background p-2"
            value={reportType}
            onChange={(event) => onReportTypeChange(event.target.value as ReportType)}
          >
            <option value="analysis">{t("reportTypeAnalysis")}</option>
            <option value="summary">{t("reportTypeSummary")}</option>
            <option value="thesis">{t("reportTypeThesis")}</option>
            <option value="recommendation">
              {t("reportTypeRecommendation")}
            </option>
          </select>
          <button
            className="ml-2 rounded border px-3 py-1"
            disabled={
              !reportTitle.trim() ||
              !reportContent.trim() ||
              createReportPending
            }
          >
            {t("saveReport")}
          </button>
        </form>
        <ul className="space-y-2">
          {reports?.map((item) => (
            <li key={item.id} className="rounded bg-muted p-2 text-sm">
              <div className="flex items-start justify-between">
                <div>
                  <p className="font-medium">
                    {item.title}{" "}
                    <span className="font-normal text-muted-foreground">
                      ({item.reportType})
                    </span>
                  </p>
                  <p>{item.content}</p>
                </div>
                <button
                  onClick={() => deleteReport(item.id)}
                  className="rounded p-1 hover:bg-accent"
                  title={t("deleteReport")}
                >
                  <Trash2 className="h-4 w-4 text-destructive" />
                </button>
              </div>
            </li>
          ))}
        </ul>
      </div>
    </section>
  );
}
