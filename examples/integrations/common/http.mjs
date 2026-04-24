import { numberEnv } from "./env.mjs";

const DEFAULT_TIMEOUT = numberEnv("INTEGRATIONS_TIMEOUT_MS", 12000);

export async function httpJson(url, { method = "GET", headers = {}, body, timeoutMs = DEFAULT_TIMEOUT } = {}) {
  const ctl = new AbortController();
  const timer = setTimeout(() => ctl.abort(), timeoutMs);
  try {
    const res = await fetch(url, {
      method,
      headers: { "content-type": "application/json", ...headers },
      body: body ? JSON.stringify(body) : undefined,
      signal: ctl.signal,
    });
    const text = await res.text();
    let data = {};
    try {
      data = text ? JSON.parse(text) : {};
    } catch {
      data = { raw: text };
    }
    if (!res.ok) {
      return { ok: false, status: res.status, error: data.error || data.message || `HTTP ${res.status}`, data };
    }
    return { ok: true, status: res.status, data };
  } catch (error) {
    return { ok: false, status: 0, error: error instanceof Error ? error.message : String(error), data: {} };
  } finally {
    clearTimeout(timer);
  }
}

export function buildHeaders(apiKey) {
  if (!apiKey) return {};
  return { Authorization: `Bearer ${apiKey}` };
}
