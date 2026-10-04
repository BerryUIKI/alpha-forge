import "@testing-library/jest-dom";
import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ConfirmDialog } from "./ConfirmDialog";

describe("ConfirmDialog", () => {
  it("renders nothing when isOpen is false", () => {
    const { container } = render(
      <ConfirmDialog
        isOpen={false}
        message="Are you sure?"
        onConfirm={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    expect(container).toBeEmptyDOMElement();
  });

  it("renders dialog with title, message, and accessibility attributes", () => {
    render(
      <ConfirmDialog
        isOpen={true}
        title="Custom Title"
        message="This is a dangerous operation."
        onConfirm={vi.fn()}
        onClose={vi.fn()}
      />,
    );

    const dialog = screen.getByRole("alertdialog");
    expect(dialog).toBeInTheDocument();
    expect(dialog).toHaveAttribute("aria-modal", "true");
    expect(screen.getByText("Custom Title")).toBeInTheDocument();
    expect(screen.getByText("This is a dangerous operation.")).toBeInTheDocument();
  });

  it("calls onConfirm when confirm button is clicked", () => {
    const onConfirm = vi.fn();
    render(
      <ConfirmDialog
        isOpen={true}
        message="Delete this item?"
        confirmLabel="Yes, Delete"
        onConfirm={onConfirm}
        onClose={vi.fn()}
      />,
    );

    const confirmBtn = screen.getByRole("button", { name: "Yes, Delete" });
    fireEvent.click(confirmBtn);
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("calls onClose when cancel button is clicked", () => {
    const onClose = vi.fn();
    render(
      <ConfirmDialog
        isOpen={true}
        message="Delete this item?"
        cancelLabel="Nevermind"
        onConfirm={vi.fn()}
        onClose={onClose}
      />,
    );

    const cancelBtn = screen.getByRole("button", { name: "Nevermind" });
    fireEvent.click(cancelBtn);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("calls onClose when Escape key is pressed", () => {
    const onClose = vi.fn();
    render(
      <ConfirmDialog
        isOpen={true}
        message="Delete this item?"
        onConfirm={vi.fn()}
        onClose={onClose}
      />,
    );

    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("disables buttons and does not close on escape when isConfirming is true", () => {
    const onClose = vi.fn();
    const onConfirm = vi.fn();
    render(
      <ConfirmDialog
        isOpen={true}
        message="Deleting in progress..."
        isConfirming={true}
        onConfirm={onConfirm}
        onClose={onClose}
      />,
    );

    const buttons = screen.getAllByRole("button");
    for (const btn of buttons) {
      expect(btn).toBeDisabled();
    }

    fireEvent.keyDown(document, { key: "Escape" });
    expect(onClose).not.toHaveBeenCalled();
  });
});
