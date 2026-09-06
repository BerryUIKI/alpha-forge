import { FileText, ArrowUpRight, ArrowDownRight } from "lucide-react";
import { useLocale } from "@/lib/i18n/useLocale";
import { formatMessage } from "@/lib/i18n/locale";

interface FilingItem {
  id: string;
  ticker: string;
  formType: string;
  timeAgo: string;
  summary: string;
  linkedThesis: string;
  impactType: "positive" | "contra";
  confidenceDelta: number;
}

const DEMO_FILINGS: FilingItem[] = [
  {
    id: "filing-1",
    ticker: "NVDA",
    formType: "SEC Form 10-Q",
    timeAgo: "2h",
    summary:
      "Data Center revenue grew 154% YoY with customer purchase obligations expanding 28% QoQ, confirming strong enterprise rack visibility.",
    linkedThesis: "Blackwell Rack Ramp Ahead of Consensus",
    impactType: "positive",
    confidenceDelta: 4,
  },
  {
    id: "filing-2",
    ticker: "ASML",
    formType: "Q2 Earnings Transcript",
    timeAgo: "1d",
    summary:
      "Management confirmed selective High-NA EUV tool deliveries deferred into early 2026, though total order backlog remains solid at €36B.",
    linkedThesis: "EUV Lithography Order Inflection",
    impactType: "contra",
    confidenceDelta: 3,
  },
];

export function SecFilingFeed() {
  const { t } = useLocale();

  return (
    <div className="rounded-xl border border-white/10 bg-[#1c1e28] p-5 shadow-sm">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <FileText className="h-4 w-4 text-cyan-400" />
          <h3 className="text-sm font-semibold text-white tracking-tight">
            {t("secFilingFeed")}
          </h3>
        </div>
        <span className="rounded bg-cyan-500/10 px-2 py-0.5 text-[10px] font-medium text-cyan-300 border border-cyan-500/20">
          {t("secAutomatedParsing")}
        </span>
      </div>

      <div className="space-y-3">
        {DEMO_FILINGS.map((item) => (
          <div
            key={item.id}
            className="rounded-lg border border-white/5 bg-black/20 p-3 text-xs space-y-2 hover:border-white/10 transition-colors"
          >
            {/* Header */}
            <div className="flex items-center justify-between">
              <span className="font-semibold text-indigo-300 flex items-center gap-1.5">
                <span className="font-mono text-white font-bold">{item.ticker}</span>
                <span className="text-neutral-500">·</span>
                <span>{item.formType}</span>
              </span>
              <span className="text-[10px] text-neutral-400 font-mono">
                {formatMessage(t("hoursAgo"), { hours: item.timeAgo })}
              </span>
            </div>

            {/* Extracted snippet */}
            <p className="text-neutral-300 text-[11px] leading-relaxed">
              {item.summary}
            </p>

            {/* Linked Thesis & Conviction Delta */}
            <div className="flex flex-wrap items-center justify-between pt-1 border-t border-white/5 text-[10px]">
              <span className="text-neutral-400">
                {formatMessage(t("linkedThesis"), { title: item.linkedThesis })}
              </span>
              <div
                className={`flex items-center gap-1 font-medium ${
                  item.impactType === "positive"
                    ? "text-emerald-400"
                    : "text-rose-400"
                }`}
              >
                {item.impactType === "positive" ? (
                  <>
                    <ArrowUpRight className="h-3 w-3" />
                    <span>
                      {formatMessage(t("evidencePositive"), {
                        delta: String(item.confidenceDelta),
                      })}
                    </span>
                  </>
                ) : (
                  <>
                    <ArrowDownRight className="h-3 w-3" />
                    <span>
                      {formatMessage(t("evidenceContra"), {
                        delta: String(item.confidenceDelta),
                      })}
                    </span>
                  </>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
