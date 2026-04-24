import { boolEnv } from "../common/env.mjs";
import { buildHeaders, httpJson } from "../common/http.mjs";

export class DefindexAdapter {
  constructor() {
    this.baseUrl = process.env.DEFINDEX_API_BASE_URL || "https://api.defindex.io";
    this.apiKey = process.env.DEFINDEX_API_KEY || "";
    this.vaultAddress = process.env.DEFINDEX_VAULT_ADDRESS || "";
    this.useMock = boolEnv("INTEGRATIONS_USE_MOCK", false);
  }

  getCapabilities() {
    return { provider: "defindex", operations: ["healthcheck", "getApy", "getBalance", "deposit", "withdraw"] };
  }

  async healthcheck() {
    if (this.useMock) return { ok: true, provider: "defindex", operation: "healthcheck", data: { mode: "mock" } };
    const res = await httpJson(`${this.baseUrl}/health`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "defindex", operation: "healthcheck", data: res.data }
      : { ok: false, provider: "defindex", operation: "healthcheck", error: res.error, data: res.data };
  }

  async getApy() {
    if (this.useMock) return { ok: true, provider: "defindex", operation: "getApy", data: { apy: 6.2, source: "mock" } };
    const res = await httpJson(`${this.baseUrl}/apy?vaultAddress=${encodeURIComponent(this.vaultAddress)}`, {
      headers: buildHeaders(this.apiKey),
    });
    return res.ok
      ? { ok: true, provider: "defindex", operation: "getApy", data: res.data }
      : { ok: false, provider: "defindex", operation: "getApy", error: res.error, data: res.data };
  }

  async getBalance({ userAddress }) {
    if (this.useMock) {
      return { ok: true, provider: "defindex", operation: "getBalance", data: { userAddress, balance: "12500000", source: "mock" } };
    }
    const q = new URLSearchParams({ userAddress, vaultAddress: this.vaultAddress }).toString();
    const res = await httpJson(`${this.baseUrl}/balance?${q}`, { headers: buildHeaders(this.apiKey) });
    return res.ok
      ? { ok: true, provider: "defindex", operation: "getBalance", data: res.data }
      : { ok: false, provider: "defindex", operation: "getBalance", error: res.error, data: res.data };
  }

  async deposit({ userAddress, amount }) {
    if (this.useMock) {
      return { ok: true, provider: "defindex", operation: "deposit", data: { userAddress, amount, txHash: `mock_defindex_dep_${Date.now()}` } };
    }
    const res = await httpJson(`${this.baseUrl}/deposit`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { userAddress, amount, vaultAddress: this.vaultAddress },
    });
    return res.ok
      ? { ok: true, provider: "defindex", operation: "deposit", data: res.data }
      : { ok: false, provider: "defindex", operation: "deposit", error: res.error, data: res.data };
  }

  async withdraw({ userAddress, amount }) {
    if (this.useMock) {
      return { ok: true, provider: "defindex", operation: "withdraw", data: { userAddress, amount, txHash: `mock_defindex_wd_${Date.now()}` } };
    }
    const res = await httpJson(`${this.baseUrl}/withdraw`, {
      method: "POST",
      headers: buildHeaders(this.apiKey),
      body: { userAddress, amount, vaultAddress: this.vaultAddress },
    });
    return res.ok
      ? { ok: true, provider: "defindex", operation: "withdraw", data: res.data }
      : { ok: false, provider: "defindex", operation: "withdraw", error: res.error, data: res.data };
  }
}
