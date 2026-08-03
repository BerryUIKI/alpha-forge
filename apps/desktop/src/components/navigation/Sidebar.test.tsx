import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { BrowserRouter } from "react-router-dom";
import { Sidebar } from "./Sidebar";
import { LocaleContext } from "@/lib/i18n/locale-context";
import type { Locale } from "@/lib/i18n/locale";

function renderSidebar(locale: Locale = "zh-CN") {
  const setLocale = vi.fn();
  const t = vi.fn((key: string) => key);

  return render(
    <LocaleContext.Provider value={{ locale, setLocale, t }}>
      <BrowserRouter>
        <Sidebar />
      </BrowserRouter>
    </LocaleContext.Provider>,
  );
}

describe("Sidebar", () => {
  it("renders navigation items", () => {
    renderSidebar();

    const navLinks = screen.getAllByRole("link");
    expect(navLinks).toHaveLength(6);
  });

  it("applies aria-label to navigation items", () => {
    renderSidebar();

    const todayLink = screen.getByRole("link", { name: "today" });
    expect(todayLink).toHaveAttribute("aria-label", "today");
  });

  it("applies title attribute for tooltips", () => {
    renderSidebar();

    const todayLink = screen.getByRole("link", { name: "today" });
    expect(todayLink).toHaveAttribute("title", "today");
  });

  it("renders in English locale", () => {
    renderSidebar("en");

    const navLinks = screen.getAllByRole("link");
    expect(navLinks).toHaveLength(6);
  });
});