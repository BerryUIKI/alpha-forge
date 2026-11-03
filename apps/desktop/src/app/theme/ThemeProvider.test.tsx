import { act, fireEvent, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { THEME_STORAGE_KEY } from "./theme-context";
import { ThemeProvider } from "./ThemeProvider";
import { useTheme } from "./useTheme";

type MediaListener = () => void;

let systemPrefersDark = false;
let mediaListeners: MediaListener[] = [];

function installMatchMediaMock() {
  vi.mocked(window.matchMedia).mockImplementation((query: string) => {
    return {
      matches: query === "(prefers-color-scheme: dark)" ? systemPrefersDark : false,
      media: query,
      onchange: null,
      addEventListener: (_event: string, listener: MediaListener) => {
        mediaListeners.push(listener);
      },
      removeEventListener: (_event: string, listener: MediaListener) => {
        mediaListeners = mediaListeners.filter((item) => item !== listener);
      },
      addListener: vi.fn(),
      removeListener: vi.fn(),
      dispatchEvent: vi.fn(),
    } as unknown as MediaQueryList;
  });
}

function Probe() {
  const { theme, resolvedTheme, setTheme } = useTheme();
  const next = theme === "system" ? "light" : theme === "light" ? "dark" : "system";

  return (
    <div>
      <span data-testid="theme">{theme}</span>
      <span data-testid="resolved">{resolvedTheme}</span>
      <button onClick={() => setTheme(next)}>cycle theme</button>
    </div>
  );
}

function renderTheme() {
  return render(
    <ThemeProvider>
      <Probe />
    </ThemeProvider>,
  );
}

beforeEach(() => {
  systemPrefersDark = false;
  mediaListeners = [];
  window.localStorage.clear();
  document.documentElement.className = "";
  installMatchMediaMock();
});

afterEach(() => {
  window.localStorage.clear();
  document.documentElement.className = "";
  document.documentElement.style.removeProperty("color-scheme");
});

describe("ThemeProvider", () => {
  it("defaults to the system theme and applies light when the system is light", () => {
    renderTheme();

    expect(screen.getByTestId("theme")).toHaveTextContent("system");
    expect(screen.getByTestId("resolved")).toHaveTextContent("light");
    expect(document.documentElement).not.toHaveClass("dark");
  });

  it("applies and persists an explicit dark theme", () => {
    renderTheme();

    fireEvent.click(screen.getByRole("button", { name: "cycle theme" }));
    fireEvent.click(screen.getByRole("button", { name: "cycle theme" }));

    expect(screen.getByTestId("theme")).toHaveTextContent("dark");
    expect(screen.getByTestId("resolved")).toHaveTextContent("dark");
    expect(document.documentElement).toHaveClass("dark");
    expect(window.localStorage.getItem(THEME_STORAGE_KEY)).toBe("dark");
  });

  it("restores a stored theme", () => {
    window.localStorage.setItem(THEME_STORAGE_KEY, "dark");

    renderTheme();

    expect(screen.getByTestId("theme")).toHaveTextContent("dark");
    expect(document.documentElement).toHaveClass("dark");
  });

  it("follows system changes while in system mode", () => {
    renderTheme();
    expect(document.documentElement).not.toHaveClass("dark");

    systemPrefersDark = true;
    act(() => {
      mediaListeners.forEach((listener) => listener());
    });

    expect(screen.getByTestId("resolved")).toHaveTextContent("dark");
    expect(document.documentElement).toHaveClass("dark");
  });
});
