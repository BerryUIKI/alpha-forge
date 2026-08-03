import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ErrorState } from "./ErrorState";

describe("ErrorState", () => {
  it("renders error message", () => {
    render(<ErrorState message="Failed to load data" />);

    expect(screen.getByRole("alert")).toBeInTheDocument();
    expect(screen.getByText("Failed to load data")).toBeInTheDocument();
  });

  it("renders custom title", () => {
    render(<ErrorState title="Custom Error" message="Error occurred" />);

    expect(screen.getByText("Custom Error")).toBeInTheDocument();
  });

  it("uses default title when not specified", () => {
    render(<ErrorState message="Error occurred" />);

    expect(screen.getByText("Something went wrong")).toBeInTheDocument();
  });

  it("renders retry button with custom label", () => {
    const onRetry = vi.fn();
    render(<ErrorState message="Error" retryLabel="Retry Now" onRetry={onRetry} />);

    const button = screen.getByRole("button", { name: "Retry Now" });
    fireEvent.click(button);
    expect(onRetry).toHaveBeenCalledOnce();
  });

  it("hides retry button when onRetry not provided", () => {
    render(<ErrorState message="Error" />);

    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });

  it("has correct aria attributes", () => {
    render(<ErrorState message="Error message" />);

    const alert = screen.getByRole("alert");
    expect(alert).toHaveAttribute("aria-live", "polite");
  });
});