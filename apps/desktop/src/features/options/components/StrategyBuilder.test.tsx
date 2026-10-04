import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { StrategyBuilder } from "./StrategyBuilder";

describe("StrategyBuilder Component", () => {
  it("renders with initial empty state and default underlying price", () => {
    render(<StrategyBuilder />);
    expect(screen.getByText("Strategy Builder")).toBeInTheDocument();
    expect(screen.getByText("No legs added yet.")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Analyze Strategy" })).toBeDisabled();
  });

  it("allows adding and removing strategy legs", () => {
    render(<StrategyBuilder />);
    const addLegBtn = screen.getByRole("button", { name: "Add Leg" });
    fireEvent.click(addLegBtn);

    expect(screen.getByText("Leg 1")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Analyze Strategy" })).not.toBeDisabled();

    const removeBtn = screen.getByRole("button", { name: "Remove" });
    fireEvent.click(removeBtn);

    expect(screen.queryByText("Leg 1")).not.toBeInTheDocument();
    expect(screen.getByText("No legs added yet.")).toBeInTheDocument();
  });

  it("calculates real net cost and break-even points when analyzed", () => {
    const onBuild = vi.fn();
    render(<StrategyBuilder onBuild={onBuild} />);

    // Add Leg 1: Default Call strike 100, premium 5, long
    fireEvent.click(screen.getByRole("button", { name: "Add Leg" }));

    // Click Analyze Strategy
    const analyzeBtn = screen.getByRole("button", { name: "Analyze Strategy" });
    fireEvent.click(analyzeBtn);

    // Verify Analysis Output
    expect(screen.getByText("Strategy Analysis")).toBeInTheDocument();
    // Net Cost and Max Loss for 1 long call @ 5 = $5.00
    expect(screen.getAllByText("$5.00").length).toBeGreaterThanOrEqual(1);
    // Break-even for long call = 100 + 5 = 105
    expect(screen.getByText("$105")).toBeInTheDocument();

    expect(onBuild).toHaveBeenCalledWith(
      expect.arrayContaining([
        expect.objectContaining({ strike: 100, premium: 5, optionType: "call", positionType: "long" }),
      ]),
      expect.objectContaining({
        netCost: 5,
        breakEvenPoints: [105],
      }),
    );
  });
});
