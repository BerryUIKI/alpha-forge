import "@testing-library/jest-dom";
import { describe, expect, it, vi, beforeEach } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { RightSidebar } from "./RightSidebar";

vi.mock("@/hooks/useAgentStatus", () => ({
  useAgentGlobalStatusDisplay: () => ({
    status: "idle",
    isLoading: false,
    isError: false,
  }),
}));

vi.mock("./AgentPanel", () => ({
  AgentPanel: () => <div>Agent panel content</div>,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

describe("RightSidebar", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it("expands when the layout changes its controlled state", () => {
    const { rerender } = render(<RightSidebar state="collapsed" />);
    expect(screen.queryByText("Agent panel content")).not.toBeInTheDocument();

    rerender(<RightSidebar state="expanded" onStateChange={vi.fn()} />);

    expect(screen.getByText("Agent panel content")).toBeInTheDocument();
    expect(screen.getByRole("separator")).toBeInTheDocument();
  });

  it("notifies the layout when its own controls toggle the drawer", () => {
    const onStateChange = vi.fn();
    render(<RightSidebar state="collapsed" onStateChange={onStateChange} />);

    const [expandButton] = screen.getAllByLabelText("openAgentPanel");
    fireEvent.click(expandButton!);
    expect(onStateChange).toHaveBeenCalledWith("expanded");
  });

  it("resizes from the keyboard and exposes the current width", () => {
    render(<RightSidebar state="expanded" />);

    const separator = screen.getByRole("separator");
    expect(separator).toHaveAttribute("aria-valuenow", "320");
    expect(separator).toHaveAttribute("aria-valuemin", "260");
    expect(separator).toHaveAttribute("aria-valuemax", "400");

    fireEvent.keyDown(separator, { key: "ArrowLeft" });
    expect(separator).toHaveAttribute("aria-valuenow", "336");

    fireEvent.keyDown(separator, { key: "End" });
    expect(separator).toHaveAttribute("aria-valuenow", "400");
  });
});
