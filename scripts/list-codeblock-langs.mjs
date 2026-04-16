#!/usr/bin/env node
/**
 * Scans all markdown files in the project, outputs every code-block language
 * that is NOT "mermaid", and updates the LANGS array in shiki.entry.js.
 *
 * Usage:
 *   node scripts/list-codeblock-langs.mjs [paths...]
 *
 * Defaults to scanning: contents/
 * You can pass any number of directories or files as arguments:
 *   node scripts/list-codeblock-langs.mjs contents/ apps/
 */

import { readFileSync, writeFileSync, statSync } from "node:fs";
import { readdir } from "node:fs/promises";
import { resolve, extname } from "node:path";

// Path to shiki.entry.js to update
const SHIKI_ENTRY_PATH = resolve("apps/frontend/shiki.entry.js");

// Languages always kept regardless of content (fallback + shell essentials)
const BASE_LANGS = ["bash", "sh"];

// Mapping from user-written fence language to @shikijs/langs/<name>.
// If a lang is not in this map, it is tried as-is (e.g. "rust" → "@shikijs/langs/rust").
// If a lang resolves to null it is skipped (no shiki grammar available).
const LANG_MAP = {
  vuejs: "vue",
  react: "jsx",
  sh: "sh",
  shell: "shell",
  txt: null, // no shiki grammar for plain text
  text: null,
  "(none)": null,
};

// --------------------------------------------------------------------------
// CLI arguments
// --------------------------------------------------------------------------
const EXCLUDE_LANG = "mermaid";
const roots = process.argv.slice(2).length
  ? process.argv.slice(2).map((p) => resolve(p))
  : [resolve("contents")];

// --------------------------------------------------------------------------
// Helpers
// --------------------------------------------------------------------------

/** Recursively collect every .md file under a path. */
async function collectMarkdownFiles(root) {
  const stat = statSync(root, { throwIfNoEntry: false });
  if (!stat) return [];
  if (stat.isFile()) return extname(root) === ".md" ? [root] : [];

  const entries = await readdir(root, { withFileTypes: true });
  const nested = await Promise.all(
    entries.map((e) => collectMarkdownFiles(resolve(root, e.name))),
  );
  return nested.flat();
}

/** Extract opening fence languages from markdown text, skipping the excluded one. */
function extractLanguages(text, filePath) {
  const results = [];
  // Match opening fences: ``` or ~~~ followed by optional language token
  const fenceRe = /^(?:```|~~~)([\w.+-][\w.+\-:/ ]*)?/gm;
  let match;
  let lineNumber = 1;
  let lastIndex = 0;

  while ((match = fenceRe.exec(text)) !== null) {
    // Count lines up to current position
    lineNumber += (text.slice(lastIndex, match.index).match(/\n/g) || [])
      .length;
    lastIndex = match.index;

    const rawLang = (match[1] || "").trim();
    // The first word before any space/brace is the language identifier
    const lang = rawLang.split(/[\s{]/)[0].toLowerCase() || "(none)";

    if (lang !== EXCLUDE_LANG) {
      results.push({ lang, file: filePath, line: lineNumber });
    }
  }
  return results;
}

// --------------------------------------------------------------------------
// Main
// --------------------------------------------------------------------------
(async () => {
  /** @type {Array<{lang: string, file: string, line: number}>} */
  const allOccurrences = [];

  for (const root of roots) {
    const files = await collectMarkdownFiles(root);
    for (const file of files) {
      const text = readFileSync(file, "utf8");
      allOccurrences.push(...extractLanguages(text, file));
    }
  }

  if (allOccurrences.length === 0) {
    console.log("No non-mermaid code blocks found.");
    process.exit(0);
  }

  // Group by language
  /** @type {Map<string, Array<{file: string, line: number}>>} */
  const byLang = new Map();
  for (const { lang, file, line } of allOccurrences) {
    if (!byLang.has(lang)) byLang.set(lang, []);
    byLang.get(lang).push({ file, line });
  }

  // Sort by occurrence count descending
  const sorted = [...byLang.entries()].sort(
    (a, b) => b[1].length - a[1].length,
  );

  // --------------------------------------------------------------------------
  // Output
  // --------------------------------------------------------------------------
  const maxLangLen = Math.max(
    ...sorted.map(([l]) => l.length),
    "language".length,
  );
  const header = `${"Language".padEnd(maxLangLen)}  Count  Locations`;
  const separator = "-".repeat(header.length);

  console.log("\nCode block languages (excluding mermaid):\n");
  console.log(header);
  console.log(separator);

  for (const [lang, locs] of sorted) {
    const locStr = locs
      .map(
        ({ file, line }) => `${file.replace(process.cwd() + "/", "")}:${line}`,
      )
      .join(", ");
    console.log(
      `${lang.padEnd(maxLangLen)}  ${String(locs.length).padStart(5)}  ${locStr}`,
    );
  }

  console.log(separator);
  console.log(
    `\nTotal: ${allOccurrences.length} code blocks, ${sorted.length} unique languages.\n`,
  );

  // --------------------------------------------------------------------------
  // Update shiki.entry.js — fine-grained imports + LANGS array
  // --------------------------------------------------------------------------

  /**
   * Convert a user-written fence language to the @shikijs/langs module name.
   * Returns null if the language has no shiki grammar.
   */
  function resolveShikiLang(userLang) {
    if (Object.prototype.hasOwnProperty.call(LANG_MAP, userLang)) {
      return LANG_MAP[userLang]; // may be null → skip
    }
    return userLang; // use as-is
  }

  /** Turn a shiki lang name into a JS identifier: "shell-script" → "langShellScript" */
  function langToIdentifier(shikiName) {
    return (
      "lang" +
      shikiName
        .split(/[-_]/)
        .map((p) => p.charAt(0).toUpperCase() + p.slice(1))
        .join("")
    );
  }

  // Collect all user langs (BASE_LANGS first, then found), dedupe, resolve, drop nulls
  const allUserLangs = [...new Set([...BASE_LANGS, ...sorted.map(([l]) => l)])];
  const resolvedLangs = allUserLangs
    .map((u) => ({ user: u, shiki: resolveShikiLang(u) }))
    .filter(({ shiki }) => shiki !== null);

  // Dedupe on shiki name (multiple user names may resolve to the same grammar)
  const seenShiki = new Set();
  const uniqueLangs = resolvedLangs.filter(({ shiki }) => {
    if (seenShiki.has(shiki)) return false;
    seenShiki.add(shiki);
    return true;
  });
  uniqueLangs.sort((a, b) => a.shiki.localeCompare(b.shiki));

  // Build code blocks
  const importsBlock = uniqueLangs
    .map(
      ({ shiki }) =>
        `import ${langToIdentifier(shiki)} from "@shikijs/langs/${shiki}";`,
    )
    .join("\n");

  const langsArray =
    "const LANGS = [" +
    uniqueLangs.map(({ shiki }) => langToIdentifier(shiki)).join(", ") +
    "];";

  let entrySource;
  try {
    entrySource = readFileSync(SHIKI_ENTRY_PATH, "utf8");
  } catch {
    console.error(
      `\n⚠️  Could not read ${SHIKI_ENTRY_PATH} — skipping update.`,
    );
    process.exit(0);
  }

  let updated = entrySource;

  // Replace import block between markers
  const hasImportMarkers =
    entrySource.includes("// @shiki-lang-imports-start") &&
    entrySource.includes("// @shiki-lang-imports-end");
  const hasLangsMarkers =
    entrySource.includes("// @shiki-langs-start") &&
    entrySource.includes("// @shiki-langs-end");

  if (!hasImportMarkers || !hasLangsMarkers) {
    console.log(
      "⚠️  Markers not found in shiki.entry.js — no changes written.",
    );
    process.exit(0);
  }

  updated = updated.replace(
    /(\/\/ @shiki-lang-imports-start[^\n]*\n)[\s\S]*?(\/\/ @shiki-lang-imports-end)/,
    `$1${importsBlock}\n$2`,
  );

  // Replace LANGS line between markers
  updated = updated.replace(
    /(\/\/ @shiki-langs-start[^\n]*\n)[\s\S]*?(\/\/ @shiki-langs-end)/,
    `$1${langsArray}\n$2`,
  );

  writeFileSync(SHIKI_ENTRY_PATH, updated, "utf8");
  const shikiNames = uniqueLangs.map(({ shiki }) => shiki);
  console.log(`✅  shiki.entry.js updated (${shikiNames.length} langs):`);
  console.log(`   ${shikiNames.join(", ")}\n`);
})();
