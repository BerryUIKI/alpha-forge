import "@testing-library/jest-dom";
import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { LoadingSpinner } from "./LoadingSpinner";

describe("LoadingSpinner", () => {
  it("renders with default aria-label", () => {
    render(<LoadingSpinner />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "Loading");
    expect(screen.getByText("Loading")).toHaveClass("sr-only");
  });

  it("renders with custom aria-label", () => {
    render(<LoadingSpinner ariaLabel="Loading data" />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "Loading data");
    expect(screen.getByText("Loading data")).toHaveClass("sr-only");
  });

  it("applies size classes", () => {
    const { container, rerender } = render(<LoadingSpinner size="sm" />);
    expect(container.querySelector(".h-4.w-4")).toBeInTheDocument();

    rerender(<LoadingSpinner size="md" />);
    expect(container.querySelector(".h-6.w-6")).toBeInTheDocument();

    rerender(<LoadingSpinner size="lg" />);
    expect(container.querySelector(".h-8.w-8")).toBeInTheDocument();
  });

  it("applies custom className", () => {
    const { container } = render(<LoadingSpinner className="mt-4" />);

    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper).toHaveClass("mt-4");
  });

  it("has aria-live attribute", () => {
    render(<LoadingSpinner />);

    expect(screen.getByRole("status")).toHaveAttribute("aria-live", "polite");
  });
});