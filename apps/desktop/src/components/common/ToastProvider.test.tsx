import { fireEvent, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { describe, expect, it } from "vitest";

import { ToastProvider } from "./ToastProvider";
import { useToast } from "./useToast";

function Trigger() {
  const { toast } = useToast();

  return (
    <button
      onClick={() =>
        toast({
          title: "Workspace created",
          description: "Alpha Research is ready.",
          variant: "success",
        })
      }
    >
      show toast
    </button>
  );
}

describe("ToastProvider", () => {
  it("renders a toast with its title and description", async () => {
    render(
      <ToastProvider>
        <Trigger />
      </ToastProvider>,
    );

    fireEvent.click(screen.getByRole("button", { name: "show toast" }));

    expect(await screen.findByText("Workspace created")).toBeInTheDocument();
    expect(screen.getByText("Alpha Research is ready.")).toBeInTheDocument();
  });
});
