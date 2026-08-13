import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (file) => fs.readFileSync(path.join(root, file), "utf8");
const wrappers = [
  ...read("apps/desktop/src/lib/desktop-api/options.ts").matchAll(
    /invoke(?:Option|OptionVoid)?\(\s*["']([^"']+)/g,
  ),
].map((m) => m[1]);
const block = read("apps/desktop/src-tauri/src/lib.rs").match(
  /\/\/ Option commands([\s\S]*?)(?=\/\/ Goose commands)/,
)?.[1];
if (!block) throw new Error("Could not find Option registration block");
const handlers = [...block.matchAll(/commands::options::([a-z0-9_]+)/g)].map((m) => m[1]);
const missing = wrappers.filter((name) => !handlers.includes(name));
const extra = handlers.filter((name) => !wrappers.includes(name));
if (missing.length || extra.length || wrappers.includes("create_option_chain")) {
  throw new Error(
    `Option IPC mismatch; missing: ${missing.join(", ") || "none"}; extra: ${extra.join(", ") || "none"}`,
  );
}
console.log(
  `Option IPC registration OK (${wrappers.length} wrapper commands, ${handlers.length} registered handlers).`,
);
