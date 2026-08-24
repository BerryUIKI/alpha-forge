/**
 * Tests for the WorkspaceSwitcher TopBar component.
 *
 * The switcher is the single global workspace selector (ADR-0008): it reads
 * the active-workspace context and writes back through setActiveWorkspace.
 */

import "@testing-library/jest-dom";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { WorkspaceSwitcher } from "./WorkspaceSwitcher";

const { setActiveWorkspace, useActiveWorkspaceMock } = vi.hoisted(() => ({
  setActiveWorkspace: vi.fn(),
  useActiveWorkspaceMock: vi.fn(),
}));

const mockWorkspaces = [
  { id: "ws-1", name: "AI Infrastructure", created_at: "2026-01-01", updated_at: "2026-01-01" },
  { id: "ws-2", name: "Semiconductors", created_at: "2026-01-02", updated_at: "2026-01-02" },
];

const defaultContext = {
  workspaceId: "ws-1",
  workspace: mockWorkspaces[0]!,
  workspaces: mockWorkspaces,
  isLoading: false,
  setActiveWorkspace,
};

useActiveWorkspaceMock.mockReturnValue(defaultContext);

vi.mock("@/features/workspace/hooks/useActiveWorkspace.context", () => ({
  useActiveWorkspace: useActiveWorkspaceMock,
}));

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

beforeEach(() => {
  vi.clearAllMocks();
  // Restore the default context for tests that don't override it.
  useActiveWorkspaceMock.mockReturnValue(defaultContext);
});

describe("WorkspaceSwitcher", () => {
  it("renders the active workspace and all options", () => {
    render(<WorkspaceSwitcher />);
    const select = screen.getByLabelText("workspace") as HTMLSelectElement;
    expect(select.value).toBe("ws-1");
    const options = Array.from(select.querySelectorAll("option"));
    expect(options.map((option) => option.textContent)).toEqual([
      "AI Infrastructure",
      "Semiconductors",
    ]);
  });

  it("switches the active workspace on change", () => {
    render(<WorkspaceSwitcher />);
    fireEvent.change(screen.getByLabelText("workspace"), {
      target: { value: "ws-2" },
    });
    expect(setActiveWorkspace).toHaveBeenCalledWith("ws-2");
  });

  it("renders nothing when there are no workspaces", () => {
    useActiveWorkspaceMock.mockReturnValue({
      workspaceId: "",
      workspace: null,
      workspaces: [],
      isLoading: false,
      setActiveWorkspace,
    });
    const { container } = render(<WorkspaceSwitcher />);
    expect(container.firstChild).toBeNull();
  });
});
