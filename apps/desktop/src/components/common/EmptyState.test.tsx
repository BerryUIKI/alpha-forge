import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { EmptyState } from "./EmptyState";

describe("EmptyState", () => {
  it("renders title and description", () => {
    render(<EmptyState title="No items" description="No items found" />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "No items");
    expect(screen.getByText("No items")).toBeInTheDocument();
    expect(screen.getByText("No items found")).toBeInTheDocument();
  });

  it("renders with custom aria-label", () => {
    render(<EmptyState title="No items" ariaLabel="Empty results" />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "Empty results");
  });

  it("renders action button when provided", () => {
    const onAction = vi.fn();
    render(
      <EmptyState
        title="No items"
        action={<button onClick={onAction}>Create item</button>}
      />,
    );

    const button = screen.getByRole("button", { name: "Create item" });
    fireEvent.click(button);
    expect(onAction).toHaveBeenCalledOnce();
  });

  it("uses default aria-label from title when not specified", () => {
    render(<EmptyState title="Empty workspace" />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "Empty workspace");
  });
});