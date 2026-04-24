import { boolEnv } from "../common/env.mjs";
import { buildHeaders, httpJson } from "../common/http.mjs";

export class EtherfuseAdapter {
  constructor() {
    this.baseUrl = process.env.ETHERFUSE_API_BASE_URL || "https://app.etherfuse.com/api";
    this.apiKey = process.env.ETHERFUSE_API_KEY || "";
    this.useMock = boolEnv("INTEGRATIONS_USE_MOCK", false);
  }

  getCapabilities() {
    return { provider: "etherfuse", operations: ["healthcheck", "lookupStablebonds", "quoteOnramp", "quoteOfframp"] };
  }

  async healthcheck() {
    if (this.useMock) return { ok: true, provider: "etherfuse", operation: "healthcheck", data: { mode: "mock" } };
    const res = await httpJson(`${this.baseUrl}/health`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "etherfuse", operation: "healthcheck", data: res.data }
      : { ok: false, provider: "etherfuse", operation: "healthcheck", error: res.error, data: res.data };
  }

  async lookupStablebonds() {
    if (this.useMock) {
      return {
        ok: true,
        provider: "etherfuse",
        operation: "lookupStablebonds",
        data: [{ symbol: "MXN-CETES", currency: "MXN", apy: 0.1, source: "mock" }],
      };
    }
    const res = await httpJson(`${this.baseUrl}/lookup/stablebonds`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "etherfuse", operation: "lookupStablebonds", data: res.data }
      : { ok: false, provider: "etherfuse", operation: "lookupStablebonds", error: res.error, data: res.data };
  }

  async quoteOnramp({ fiatAmount, fiat = "MXN" }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "etherfuse",
        operation: "quoteOnramp",
        data: { fiatAmount, fiat, expectedTokenAmount: Number(fiatAmount) * 0.98, source: "mock" },
      };
    }
    const res = await httpJson(`${this.baseUrl}/quote/onramp`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { fiatAmount, fiat },
    });
    return res.ok
      ? { ok: true, provider: "etherfuse", operation: "quoteOnramp", data: res.data }
      : { ok: false, provider: "etherfuse", operation: "quoteOnramp", error: res.error, data: res.data };
  }

  async quoteOfframp({ tokenAmount, fiat = "MXN" }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "etherfuse",
        operation: "quoteOfframp",
        data: { tokenAmount, fiat, expectedFiatAmount: Number(tokenAmount) * 1.01, source: "mock" },
      };
    }
    const res = await httpJson(`${this.baseUrl}/quote/offramp`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { tokenAmount, fiat },
    });
    return res.ok
      ? { ok: true, provider: "etherfuse", operation: "quoteOfframp", data: res.data }
      : { ok: false, provider: "etherfuse", operation: "quoteOfframp", error: res.error, data: res.data };
  }
}
