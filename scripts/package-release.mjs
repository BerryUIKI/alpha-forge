#!/usr/bin/env node

/**
 * Release Packaging, Artifact Renaming, and Checksum Verification (M11-05)
 *
 * Automates the packaging pipeline:
 * 1. Collects release artifacts from target/release/bundle
 * 2. Normalizes filenames according to the approved naming scheme:
 *    - macOS: AlphaForge_<version>_aarch64.dmg (or x64)
 *    - Windows: AlphaForge_<version>_x64-setup.exe
 * 3. Computes cryptographically secure SHA-256 digests into SHA256SUMS.txt
 * 4. Supports self-verification (--verify) to validate artifact integrity
 */

import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..");

function getAppVersion() {
  const tauriConfPath = path.join(rootDir, "apps", "desktop", "src-tauri", "tauri.conf.json");
  if (fs.existsSync(tauriConfPath)) {
    const conf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
    return conf.version || "0.1.0";
  }
  return "0.1.0";
}

function computeSha256(filePath) {
  const hash = crypto.createHash("sha256");
  const data = fs.readFileSync(filePath);
  hash.update(data);
  return hash.digest("hex");
}

function collectAndNormalizeArtifacts() {
  const version = getAppVersion();
  const bundleDir = path.join(rootDir, "apps", "desktop", "src-tauri", "target", "release", "bundle");
  const distDir = path.join(rootDir, "dist-release");

  if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir, { recursive: true });
  }

  const collected = [];

  if (!fs.existsSync(bundleDir)) {
    console.log(`ℹ️ Bundle directory not found: ${bundleDir}`);
    return collected;
  }

  // Helper to walk directory recursively
  function walk(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true });
    for (const e of entries) {
      const full = path.join(dir, e.name);
      if (e.isDirectory()) {
        walk(full);
      } else if (e.isFile()) {
        if (e.name.endsWith(".dmg")) {
          // Normalize macOS dmg filename
          const arch = process.arch === "arm64" ? "aarch64" : "x64";
          const destName = `AlphaForge_${version}_${arch}.dmg`;
          const destPath = path.join(distDir, destName);
          fs.copyFileSync(full, destPath);
          collected.push({ name: destName, path: destPath });
        } else if (e.name.endsWith(".exe") && (e.name.includes("setup") || e.name.includes("nsis") || e.name.includes("AlphaForge"))) {
          // Normalize Windows NSIS setup filename
          const destName = `AlphaForge_${version}_x64-setup.exe`;
          const destPath = path.join(distDir, destName);
          fs.copyFileSync(full, destPath);
          collected.push({ name: destName, path: destPath });
        }
      }
    }
  }

  walk(bundleDir);
  return collected;
}

function generateChecksums() {
  const distDir = path.join(rootDir, "dist-release");
  if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir, { recursive: true });
  }

  const entries = fs.readdirSync(distDir, { withFileTypes: true });
  const checksumEntries = [];

  for (const e of entries) {
    if (e.isFile() && (e.name.endsWith(".dmg") || e.name.endsWith(".exe") || e.name.endsWith(".zip") || e.name.endsWith(".tar.gz"))) {
      const fullPath = path.join(distDir, e.name);
      const digest = computeSha256(fullPath);
      checksumEntries.push(`${digest}  ${e.name}`);
      console.log(`🔐 SHA-256 [${e.name}]: ${digest}`);
    }
  }

  if (checksumEntries.length > 0) {
    const sumsFile = path.join(distDir, "SHA256SUMS.txt");
    fs.writeFileSync(sumsFile, checksumEntries.join("\n") + "\n", "utf8");
    console.log(`✅ Checksum manifest written to: ${path.relative(rootDir, sumsFile)}`);
  } else {
    console.log("ℹ️ No release binary packages found in dist-release to hash.");
  }
}

function verifyChecksums() {
  const distDir = path.join(rootDir, "dist-release");
  const sumsFile = path.join(distDir, "SHA256SUMS.txt");

  if (!fs.existsSync(sumsFile)) {
    console.error(`❌ Checksum file not found: ${sumsFile}`);
    process.exit(1);
  }

  const lines = fs.readFileSync(sumsFile, "utf8").trim().split("\n");
  let verified = 0;
  let failed = 0;

  for (const line of lines) {
    if (!line.trim() || line.startsWith("#")) continue;
    const parts = line.trim().split(/\s+/);
    if (parts.length < 2) continue;

    const [expectedHash, fileName] = parts;
    const filePath = path.join(distDir, fileName);

    if (!fs.existsSync(filePath)) {
      console.error(`❌ Missing file: ${fileName}`);
      failed++;
      continue;
    }

    const actualHash = computeSha256(filePath);
    if (actualHash === expectedHash) {
      console.log(`✅ Verified: ${fileName} (${actualHash})`);
      verified++;
    } else {
      console.error(`❌ Hash mismatch: ${fileName}\n  Expected: ${expectedHash}\n  Actual:   ${actualHash}`);
      failed++;
    }
  }

  if (failed > 0) {
    console.error(`❌ Verification failed: ${failed} errors, ${verified} passed.`);
    process.exit(1);
  } else {
    console.log(`🎉 All ${verified} artifacts successfully verified against SHA256SUMS.txt`);
  }
}

function main() {
  const args = process.argv.slice(2);

  if (args.includes("--verify")) {
    console.log("==> Verifying SHA-256 checksums in dist-release...");
    verifyChecksums();
  } else if (args.includes("--checksums")) {
    console.log("==> Generating SHA-256 checksums...");
    generateChecksums();
  } else {
    console.log("==> Collecting and packaging release artifacts...");
    const collected = collectAndNormalizeArtifacts();
    console.log(`📦 Collected ${collected.length} release package(s).`);
    generateChecksums();
  }
}

main();
