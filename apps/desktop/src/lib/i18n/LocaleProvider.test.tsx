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

  it("uses Chinese by default and persists a user-selected language", async () => {
    render(
      <LocaleProvider>
        <LocaleProbe />
      </LocaleProvider>,
    );

    expect(screen.getByText("zh-CN")).toBeInTheDocument();
    expect(screen.getByText("设置")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Switch language" }));

    await waitFor(() => expect(screen.getByText("en")).toBeInTheDocument());
    expect(screen.getByText("Settings")).toBeInTheDocument();
    expect(settingsMock.setSetting).toHaveBeenCalledWith("app.locale", "en");
  });
});
