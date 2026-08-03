import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import "@testing-library/jest-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { LocaleProvider } from "./LocaleProvider";
import { useLocale } from "./useLocale";

const settingsMock = vi.hoisted(() => ({
  getSetting: vi.fn(),
  setSetting: vi.fn(),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: {
    settings: {
      getSetting: settingsMock.getSetting,
      setSetting: settingsMock.setSetting,
    },
  },
}));

function LocaleProbe() {
  const { locale, setLocale, t } = useLocale();

  return (
    <>
      <p>{locale}</p>
      <p>{t("settings")}</p>
      <button onClick={() => void setLocale("en")}>Switch language</button>
    </>
  );
}

describe("LocaleProvider", () => {
  beforeEach(() => {
    settingsMock.getSetting.mockReset();
    settingsMock.setSetting.mockReset();
    settingsMock.getSetting.mockResolvedValue(null);
    settingsMock.setSetting.mockResolvedValue(undefined);
  });

  it("detects system locale on first launch (en in test environment)", async () => {
    render(
      <LocaleProvider>
        <LocaleProbe />
      </LocaleProvider>,
    );

    // In test environment, navigator.language is typically "en"
    // So the provider should detect and use "en" as system locale
    expect(screen.getByText("en")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("uses saved locale preference over system locale", async () => {
    // Simulate a user who previously selected zh-CN
    settingsMock.getSetting.mockResolvedValue("zh-CN");

    render(
      <LocaleProvider>
        <LocaleProbe />
      </LocaleProvider>,
    );

    await waitFor(() => expect(screen.getByText("zh-CN")).toBeInTheDocument());
    await waitFor(() => expect(screen.getByText("设置")).toBeInTheDocument());
  });

  it("persists a user-selected language", async () => {
    render(
      <LocaleProvider>
        <LocaleProbe />
      </LocaleProvider>,
    );

    // Initially uses system locale (en in test env)
    expect(screen.getByText("en")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();

    // User switches to zh-CN
    const button = screen.getByRole("button", { name: "Switch language" });
    fireEvent.click(button);

    await waitFor(() => expect(screen.getByText("en")).toBeInTheDocument());
    expect(settingsMock.setSetting).toHaveBeenCalledWith("app.locale", "en");
  });
});
