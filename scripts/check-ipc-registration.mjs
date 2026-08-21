import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (file) => fs.readFileSync(path.join(root, file), "utf8");

// 1. Extract all registered commands from lib.rs
const libRs = read("apps/desktop/src-tauri/src/lib.rs");
const handlerBlockMatch = libRs.match(/generate_handler!\[([\s\S]*?)\]/);
if (!handlerBlockMatch) {
  console.error("Could not find generate_handler! block in apps/desktop/src-tauri/src/lib.rs");
  process.exit(1);
}

const registeredHandlers = new Set(
  [...handlerBlockMatch[1].matchAll(/commands::[a-z0-9_]+::([a-z0-9_]+)/g)].map((m) => m[1])
);

// 2. Scan all files in apps/desktop/src/lib/desktop-api/
const apiDir = path.join(root, "apps/desktop/src/lib/desktop-api");
const apiFiles = fs.readdirSync(apiDir).filter((file) => file.endsWith(".ts") && !file.endsWith(".test.ts"));

const invokedCommands = new Map(); // commandName -> array of source files

for (const file of apiFiles) {
  const content = fs.readFileSync(path.join(apiDir, file), "utf8");
  // Match any invoke call: invoke("command"), invoke<Type>("command"), invokeXxx("command")
  const matches = content.matchAll(/\binvoke(?:\w+)?(?:<[^>]+>)?\(\s*["']([a-z0-9_]+)["']/g);
  for (const match of matches) {
    const cmd = match[1];
    if (!invokedCommands.has(cmd)) {
      invokedCommands.set(cmd, []);
    }
    invokedCommands.get(cmd).push(file);
  }
}

// 3. Compare registered vs invoked
const invokedList = [...invokedCommands.keys()];
const missingInBackend = invokedList.filter((cmd) => !registeredHandlers.has(cmd));
const uninvokedInFrontend = [...registeredHandlers].filter((cmd) => !invokedCommands.has(cmd));

console.log(`\n=== IPC Registration Check ===`);
console.log(`Registered Rust Handlers: ${registeredHandlers.size}`);
console.log(`Frontend Wrapper Invocations: ${invokedList.length}`);

let hasError = false;

if (missingInBackend.length > 0) {
  hasError = true;
  console.error(`\n❌ ERROR: Frontend invokes commands that are NOT registered in lib.rs:`);
  for (const cmd of missingInBackend) {
    console.error(`  - ${cmd} (in ${invokedCommands.get(cmd).join(", ")})`);
  }
}

if (uninvokedInFrontend.length > 0) {
  // Warn about uninvoked commands (could be WIP or internal)
  console.warn(`\n⚠️  WARNING: Registered Rust handlers with no frontend wrapper in desktop-api:`);
  for (const cmd of uninvokedInFrontend) {
    console.warn(`  - ${cmd}`);
  }
}

if (hasError) {
  console.error(`\nIPC registration check failed.`);
  process.exit(1);
} else {
  console.log(`\n✅ All ${invokedList.length} frontend IPC wrapper commands are properly registered in Tauri lib.rs!\n`);
}
