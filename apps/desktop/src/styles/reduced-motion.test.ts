import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

describe("reduced motion", () => {
  it("ships a global reduced-motion rule that stills decorative animation", () => {
    const css = readFileSync(resolve(process.cwd(), "apps/desktop/src/styles/globals.css"), "utf8");

    expect(css).toMatch(/@media \(prefers-reduced-motion: reduce\)/);
    expect(css).toMatch(/animation-duration: 0\.01ms !important/);
    expect(css).toMatch(/animation-iteration-count: 1 !important/);
    expect(css).toMatch(/transition-duration: 0\.01ms !important/);
    // Status remains understandable without motion: loading and status text
    // live in dedicated elements rather than being conveyed by animation.
    expect(css).toMatch(/scroll-behavior: auto !important/);
  });
});
