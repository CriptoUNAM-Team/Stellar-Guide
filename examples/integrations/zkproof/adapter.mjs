import crypto from "node:crypto";
import { boolEnv } from "../common/env.mjs";
import { httpJson } from "../common/http.mjs";

export class ZkProofAdapter {
  constructor() {
    this.secretSalt = process.env.ZKPROOF_SECRET_SALT || "stellar-guide-dev-salt";
    this.backendUrl = process.env.ZKPROOF_BACKEND_URL || "";
    this.contractId = process.env.ZKPROOF_ATTESTATION_CONTRACT_ID || "";
    this.useMock = boolEnv("INTEGRATIONS_USE_MOCK", false);
  }

  getCapabilities() {
    return { provider: "zkproof", operations: ["generateProof", "verifyLocal", "verifyOnChainAttestation"] };
  }

  generateProof({ savedAmount, targetAmount, userId }) {
    if (savedAmount < targetAmount) {
      return { ok: false, provider: "zkproof", operation: "generateProof", error: "savedAmount debe ser >= targetAmount", data: {} };
    }
    const payload = `${userId}:${savedAmount}:${targetAmount}:${this.secretSalt}`;
    const proofHex = `0x${crypto.createHash("sha256").update(payload).digest("hex")}`;
    return {
      ok: true,
      provider: "zkproof",
      operation: "generateProof",
      data: { proof: proofHex, publicInputs: [String(savedAmount), String(targetAmount)], proofId: proofHex.slice(0, 18) },
    };
  }

  verifyLocal({ proof }) {
    const valid = /^0x[a-fA-F0-9]{64}$/.test(proof);
    return { ok: valid, provider: "zkproof", operation: "verifyLocal", data: { valid }, error: valid ? null : "Formato de proof inválido" };
  }

  async verifyOnChainAttestation({ proof, publicInputs }) {
    if (this.useMock || !this.backendUrl) {
      return {
        ok: true,
        provider: "zkproof",
        operation: "verifyOnChainAttestation",
        data: { verified: true, txHash: `mock_zk_attest_${Date.now()}`, contractId: this.contractId, proof, publicInputs, source: "mock" },
      };
    }
    const res = await httpJson(`${this.backendUrl}/api/soroban/verify-proof`, {
      method: "POST",
      body: { proof, publicInputs, contractAddress: this.contractId },
    });
    return res.ok
      ? { ok: true, provider: "zkproof", operation: "verifyOnChainAttestation", data: res.data }
      : { ok: false, provider: "zkproof", operation: "verifyOnChainAttestation", error: res.error, data: res.data };
  }
}
