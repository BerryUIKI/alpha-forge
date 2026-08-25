import "@testing-library/jest-dom";
import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { WindowTitleBar } from "./WindowTitleBar";

const windowControls = vi.hoisted(() => ({
  minimize: vi.fn().mockResolvedValue(undefined),
  toggleMaximize: vi.fn().mockResolvedValue(undefined),
  close: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@/lib/desktop-api", () => ({
  desktopApi: { window: windowControls },
}));

function renderTitleBar(props: React.ComponentProps<typeof WindowTitleBar> = {}) {
  return render(
    <MemoryRouter>
      <WindowTitleBar {...props} />
    </MemoryRouter>,
  );
}

beforeEach(() => {
  vi.clearAllMocks();
});

describe("WindowTitleBar", () => {
  it("renders the custom menu and window controls", () => {
    renderTitleBar();

    expect(screen.getByRole("menubar", { name: "Application menu" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "File" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Edit" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "View" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Help" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Minimize window" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Maximize or restore window" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Close window" })).toBeInTheDocument();
  });

  it("limits the Tauri drag attribute to non-interactive regions", () => {
    renderTitleBar();

    expect(screen.getByLabelText("Window drag region")).toHaveAttribute("data-tauri-drag-region");
    expect(screen.getByRole("button", { name: "Minimize window" })).not.toHaveAttribute(
      "data-tauri-drag-region",
    );
    expect(screen.getByRole("button", { name: "File" })).not.toHaveAttribute(
      "data-tauri-drag-region",
    );
  });

  it("routes window controls through desktopApi", () => {
    renderTitleBar();

    fireEvent.click(screen.getByRole("button", { name: "Minimize window" }));
    fireEvent.click(screen.getByRole("button", { name: "Maximize or restore window" }));
    fireEvent.click(screen.getByRole("button", { name: "Close window" }));

    expect(windowControls.minimize).toHaveBeenCalledOnce();
    expect(windowControls.toggleMaximize).toHaveBeenCalledOnce();
    expect(windowControls.close).toHaveBeenCalledOnce();
  });

  it("supports sidebar control and title-bar double-click maximize", () => {
    const onToggleLeftSidebar = vi.fn();
    renderTitleBar({ isLeftSidebarExpanded: true, onToggleLeftSidebar });

    fireEvent.click(screen.getByRole("button", { name: "Collapse sidebar" }));
    fireEvent.doubleClick(screen.getByLabelText("Window drag region"));

    expect(onToggleLeftSidebar).toHaveBeenCalledOnce();
    expect(windowControls.toggleMaximize).toHaveBeenCalledOnce();
  });

  it("opens and closes custom menu content", () => {
    renderTitleBar();

    fireEvent.click(screen.getByRole("button", { name: "File" }));
    expect(screen.getByRole("menu", { name: "File menu" })).toBeInTheDocument();
    expect(screen.getByRole("menuitem", { name: "Exit AlphaForge" })).toBeInTheDocument();

    fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.queryByRole("menu", { name: "File menu" })).not.toBeInTheDocument();
  });
});
