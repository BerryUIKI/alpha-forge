// Tests for the Markdown preview.

import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";

import { MarkdownPreview } from "./MarkdownPreview";

describe("MarkdownPreview", () => {
  afterEach(() => {
    cleanup();
  });

  it("renders Markdown headings and lists", () => {
    render(<MarkdownPreview content={"# Findings\n\n- Strong demand\n- Margins improving"} />);

    expect(screen.getByRole("heading", { name: "Findings" })).toBeInTheDocument();
    expect(screen.getByText("Strong demand")).toBeInTheDocument();
    expect(screen.getByText("Margins improving")).toBeInTheDocument();
  });

  it("does not render raw HTML elements", () => {
    const { container } = render(
      <MarkdownPreview content={"Text\n\n<script>alert(1)</script>\n\n<b>bold</b>"} />,
    );

    expect(container.querySelector("script")).toBeNull();
    expect(container.querySelector("b")).toBeNull();
    expect(screen.queryByText("alert(1)")).not.toBeInTheDocument();
    expect(screen.getByText("bold")).toBeInTheDocument();
  });

  it("renders links as plain text instead of navigable anchors", () => {
    render(<MarkdownPreview content={"[Example](javascript:alert(1))"} />);

    expect(screen.queryByRole("link")).not.toBeInTheDocument();
    expect(screen.getByText("Example")).toBeInTheDocument();
  });

  it("shows a placeholder for empty content", () => {
    render(<MarkdownPreview content="   " />);

    expect(screen.getByText("Nothing to preview yet.")).toBeInTheDocument();
  });
});
