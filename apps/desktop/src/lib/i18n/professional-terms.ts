import { useCallback, useEffect, useState } from "react";
import { z } from "zod";
import { desktopApi } from "@/lib/desktop-api";
import { useLocale } from "./useLocale";

export const PROFESSIONAL_TERMS_ENABLED_KEY = "app.i18n.professionalTermsEnabled";
export const PROFESSIONAL_TERMS_OVERRIDES_KEY = "app.i18n.professionalTermOverrides";
const PROFESSIONAL_TERMS_EVENT = "alphaforge:professional-terms-changed";

export const DEFAULT_PROFESSIONAL_TERMS = {
  company: "公司",
  industry: "行业",
  technology: "技术",
  macro_theme: "宏观主题",
} as const;

export type ProfessionalTerm = keyof typeof DEFAULT_PROFESSIONAL_TERMS;
type TermOverrides = Partial<Record<ProfessionalTerm, string>>;

const OverridesSchema = z.record(z.enum(["company", "industry", "technology", "macro_theme"]), z.string().trim().min(1).max(80));

function parseOverrides(value: string | null): TermOverrides {
  if (!value) return {};
  try {
    const parsed = OverridesSchema.safeParse(JSON.parse(value));
    return parsed.success ? parsed.data : {};
  } catch {
    return {};
  }
}

export function useProfessionalTerms() {
  const { locale } = useLocale();
  const [enabled, setEnabledState] = useState(false);
  const [overrides, setOverridesState] = useState<TermOverrides>({});

  const reload = useCallback(() => {
    void Promise.all([
      desktopApi.settings.getSetting(PROFESSIONAL_TERMS_ENABLED_KEY),
      desktopApi.settings.getSetting(PROFESSIONAL_TERMS_OVERRIDES_KEY),
    ]).then(([storedEnabled, storedOverrides]) => {
      setEnabledState(storedEnabled === "true");
      setOverridesState(parseOverrides(storedOverrides));
    }).catch(() => undefined);
  }, []);

  useEffect(() => {
    reload();
    window.addEventListener(PROFESSIONAL_TERMS_EVENT, reload);
    return () => window.removeEventListener(PROFESSIONAL_TERMS_EVENT, reload);
  }, [reload]);

  const setEnabled = async (nextEnabled: boolean) => {
    setEnabledState(nextEnabled);
    await desktopApi.settings.setSetting(PROFESSIONAL_TERMS_ENABLED_KEY, String(nextEnabled));
    window.dispatchEvent(new Event(PROFESSIONAL_TERMS_EVENT));
  };

  const setOverride = async (term: ProfessionalTerm, value: string) => {
    const next = { ...overrides, [term]: value.trim() || DEFAULT_PROFESSIONAL_TERMS[term] };
    setOverridesState(next);
    await desktopApi.settings.setSetting(PROFESSIONAL_TERMS_OVERRIDES_KEY, JSON.stringify(next));
    window.dispatchEvent(new Event(PROFESSIONAL_TERMS_EVENT));
  };

  const label = (term: ProfessionalTerm, fallback: string) => {
    if (locale !== "zh-CN" || !enabled) return fallback;
    return overrides[term] ?? DEFAULT_PROFESSIONAL_TERMS[term];
  };

  return { enabled, overrides, label, setEnabled, setOverride };
}
