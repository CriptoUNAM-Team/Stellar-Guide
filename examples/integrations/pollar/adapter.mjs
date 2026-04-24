import { boolEnv } from "../common/env.mjs";
import { buildHeaders, httpJson } from "../common/http.mjs";

export class PollarAdapter {
  constructor() {
    this.baseUrl = process.env.POLLAR_API_BASE_URL || "https://api.pollar.xyz";
    this.apiKey = process.env.POLLAR_API_KEY || "";
    this.network = process.env.POLLAR_STELLAR_NETWORK || "testnet";
    this.useMock = boolEnv("INTEGRATIONS_USE_MOCK", false);
  }

  getCapabilities() {
    return { provider: "pollar", operations: ["healthcheck", "createSession", "getRampQuote"] };
  }

  async healthcheck() {
    if (this.useMock) return { ok: true, provider: "pollar", operation: "healthcheck", data: { mode: "mock", network: this.network } };
    const res = await httpJson(`${this.baseUrl}/health`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "pollar", operation: "healthcheck", data: res.data }
      : { ok: false, provider: "pollar", operation: "healthcheck", error: res.error, data: res.data };
  }

  async createSession({ userAddress }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "pollar",
        operation: "createSession",
        data: { userAddress, network: this.network, sessionToken: `mock_pollar_${Date.now()}` },
      };
    }
    const res = await httpJson(`${this.baseUrl}/session`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { userAddress, network: this.network },
    });
    return res.ok
      ? { ok: true, provider: "pollar", operation: "createSession", data: res.data }
      : { ok: false, provider: "pollar", operation: "createSession", error: res.error, data: res.data };
  }

  async getRampQuote({ fiat, amount }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "pollar",
        operation: "getRampQuote",
        data: { fiat, amount, network: this.network, expectedAssetAmount: Number(amount) * 0.985, source: "mock" },
      };
    }
    const q = new URLSearchParams({ fiat, amount: String(amount), network: this.network }).toString();
    const res = await httpJson(`${this.baseUrl}/ramp/quote?${q}`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "pollar", operation: "getRampQuote", data: res.data }
      : { ok: false, provider: "pollar", operation: "getRampQuote", error: res.error, data: res.data };
  }
}
