// Minimal Node built-in declarations for filesystem-based tests.
// @types/node is intentionally not a dependency of the desktop app.

declare module "node:fs" {
  export function readFileSync(path: string, encoding: "utf8"): string;
}

declare module "node:path" {
  export function resolve(...segments: string[]): string;
}

declare const process: {
  cwd(): string;
};
