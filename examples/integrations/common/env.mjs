import fs from "node:fs";
import path from "node:path";

function parseEnvText(text) {
  const out = {};
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const idx = line.indexOf("=");
    if (idx <= 0) continue;
    const key = line.slice(0, idx).trim();
    const val = line.slice(idx + 1).trim();
    out[key] = val;
  }
  return out;
}

export function loadLocalEnv(baseDir = process.cwd()) {
  const envPath = path.join(baseDir, ".env");
  if (!fs.existsSync(envPath)) return;
  const parsed = parseEnvText(fs.readFileSync(envPath, "utf8"));
  for (const [k, v] of Object.entries(parsed)) {
    if (!process.env[k]) process.env[k] = v;
  }
}

export function boolEnv(name, fallback = false) {
  const v = process.env[name];
  if (!v) return fallback;
  return v === "1" || v.toLowerCase() === "true";
}

export function numberEnv(name, fallback) {
  const v = process.env[name];
  if (!v) return fallback;
  const n = Number(v);
  return Number.isFinite(n) ? n : fallback;
}
