import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { OfflineState } from "./OfflineState";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

function renderOfflineState(locale: Locale = "zh-CN") {
  const setLocale = vi.fn();
  const t = vi.fn((key: string) => {
    const messages: Record<string, string> = {
      offline: "你已离线",
      offlineDescription: "请检查你的网络连接后重试。",
      retry: "重试",
    };
    return messages[key] || key;
  });

  return render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <OfflineState />
    </LocaleContext.Provider>,
  );
}

describe("OfflineState", () => {
  it("renders offline message", () => {
    renderOfflineState();

    expect(screen.getByRole("alert")).toBeInTheDocument();
    expect(screen.getByRole("alert")).toHaveAttribute("aria-label", "你已离线");
  });

  it("renders retry button when onRetry provided", () => {
    const onRetry = vi.fn();
    const setLocale = vi.fn();
    const t = vi.fn((key: string) => {
      const messages: Record<string, string> = {
        offline: "You are offline",
        offlineDescription: "Please check your internet connection and try again.",
        retry: "Try Again",
      };
      return messages[key] || key;
    });

    render(
      <LocaleContext.Provider value={{ locale: "en", setLocale, t }}>
        <OfflineState onRetry={onRetry} />
      </LocaleContext.Provider>,
    );

    const button = screen.getByRole("button", { name: "Try Again" });
    fireEvent.click(button);
    expect(onRetry).toHaveBeenCalledOnce();
  });

  it("hides retry button when onRetry not provided", () => {
    renderOfflineState();

    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });

  it("has correct aria attributes", () => {
    renderOfflineState();

    const alert = screen.getByRole("alert");
    expect(alert).toHaveAttribute("aria-live", "polite");
  });

  it("renders custom description", () => {
    const setLocale = vi.fn();
    const t = vi.fn((key: string) => key);

    render(
      <LocaleContext.Provider value={{ locale: "en", setLocale, t }}>
        <OfflineState description="Custom offline message" />
      </LocaleContext.Provider>,
    );

    expect(screen.getByText("Custom offline message")).toBeInTheDocument();
  });
});