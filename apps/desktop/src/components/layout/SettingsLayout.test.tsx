import "@testing-library/jest-dom";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes, useLocation } from "react-router-dom";
import { SettingsLayout } from "./SettingsLayout";
import { rememberAppPath } from "@/lib/navigation/last-path";

vi.mock("./WindowTitleBar", () => ({
  WindowTitleBar: () => null,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

function LocationProbe() {
  const location = useLocation();
  return <span data-testid="location">{location.pathname + location.search + location.hash}</span>;
}

function renderSettings(initialEntry: string) {
  return render(
    <MemoryRouter initialEntries={[initialEntry]}>
      <Routes>
        <Route path="/settings" element={<SettingsLayout />}>
          <Route
            index
            element={
              <div>
                <section id="general">General</section>
                <section id="data">Data</section>
              </div>
            }
          />
        </Route>
        <Route path="/research" element={<LocationProbe />} />
        <Route path="/" element={<LocationProbe />} />
      </Routes>
    </MemoryRouter>,
  );
}

describe("SettingsLayout", () => {
  const scrollTo = vi.fn();

  beforeEach(() => {
    sessionStorage.clear();
    scrollTo.mockReset();
    Object.defineProperty(HTMLElement.prototype, "scrollTo", {
      value: scrollTo,
      configurable: true,
      writable: true,
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("marks exactly the section named by the hash as current", () => {
    renderSettings("/settings#data");

    expect(screen.getByRole("link", { name: /settingsData/ })).toHaveAttribute(
      "aria-current",
      "page",
    );
    expect(screen.getByRole("link", { name: /settingsGeneral/ })).not.toHaveAttribute(
      "aria-current",
    );
  });

  it("scrolls the content container to a deep-linked section", async () => {
    renderSettings("/settings#data");

    await waitFor(() => expect(scrollTo).toHaveBeenCalled());
    expect(scrollTo).toHaveBeenCalledWith(expect.objectContaining({ top: expect.any(Number) }));
  });

  it("returns to the last application route in one action", async () => {
    rememberAppPath("/research?project=p1");
    renderSettings("/settings#data");

    fireEvent.click(screen.getByLabelText("backToApplication"));

    expect(await screen.findByTestId("location")).toHaveTextContent("/research?project=p1");
  });

  it("falls back to the dashboard when no previous route is known", async () => {
    renderSettings("/settings");

    fireEvent.click(screen.getByLabelText("backToApplication"));

    expect(await screen.findByTestId("location")).toHaveTextContent("/");
  });
});
