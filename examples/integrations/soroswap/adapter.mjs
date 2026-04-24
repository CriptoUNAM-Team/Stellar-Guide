import { boolEnv } from "../common/env.mjs";
import { buildHeaders, httpJson } from "../common/http.mjs";

export class SoroswapAdapter {
  constructor() {
    this.baseUrl = process.env.SOROSWAP_API_BASE_URL || "https://api.soroswap.finance";
    this.apiKey = process.env.SOROSWAP_API_KEY || "";
    this.useMock = boolEnv("INTEGRATIONS_USE_MOCK", false);
  }

  getCapabilities() {
    return { provider: "soroswap", operations: ["healthcheck", "quote", "execute"] };
  }

  async healthcheck() {
    if (this.useMock) return { ok: true, provider: "soroswap", operation: "healthcheck", data: { mode: "mock" } };
    const res = await httpJson(`${this.baseUrl}/health`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "soroswap", operation: "healthcheck", data: res.data }
      : { ok: false, provider: "soroswap", operation: "healthcheck", error: res.error, data: res.data };
  }

  async quote({ fromAsset, toAsset, amount }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "soroswap",
        operation: "quote",
        data: { fromAsset, toAsset, amount, expectedOut: Number(amount) * 0.97, source: "mock" },
      };
    }
    const res = await httpJson(`${this.baseUrl}/quote`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { fromAsset, toAsset, amount },
    });
    return res.ok
      ? { ok: true, provider: "soroswap", operation: "quote", data: res.data }
      : { ok: false, provider: "soroswap", operation: "quote", error: res.error, data: res.data };
  }

  async execute({ fromAsset, toAsset, amount, userAddress }) {
    if (this.useMock) {
      return {
        ok: true,
        provider: "soroswap",
        operation: "execute",
        data: {
          fromAsset,
          toAsset,
          amount,
          userAddress,
          txHash: `mock_soroswap_${Date.now()}`,
          source: "mock",
        },
      };
    }
    const res = await httpJson(`${this.baseUrl}/swap`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { fromAsset, toAsset, amount, userAddress },
    });
    return res.ok
      ? { ok: true, provider: "soroswap", operation: "execute", data: res.data }
      : { ok: false, provider: "soroswap", operation: "execute", error: res.error, data: res.data };
  }
}
