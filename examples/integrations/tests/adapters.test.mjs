import test from "node:test";
import assert from "node:assert/strict";

import { SoroswapAdapter } from "../soroswap/adapter.mjs";
import { EtherfuseAdapter } from "../etherfuse/adapter.mjs";
import { DefindexAdapter } from "../defindex/adapter.mjs";
import { PollarAdapter } from "../pollar/adapter.mjs";
import { ZkProofAdapter } from "../zkproof/adapter.mjs";

process.env.INTEGRATIONS_USE_MOCK = "true";

test("soroswap quote mock", async () => {
  const adapter = new SoroswapAdapter();
  const res = await adapter.quote({ fromAsset: "XLM", toAsset: "USDC", amount: "100" });
  assert.equal(res.ok, true);
  assert.equal(res.provider, "soroswap");
});

test("etherfuse stablebonds mock", async () => {
  const adapter = new EtherfuseAdapter();
  const res = await adapter.lookupStablebonds();
  assert.equal(res.ok, true);
  assert.equal(Array.isArray(res.data), true);
});

test("defindex apy mock", async () => {
  const adapter = new DefindexAdapter();
  const res = await adapter.getApy();
  assert.equal(res.ok, true);
  assert.equal(typeof res.data.apy, "number");
});

test("pollar session mock", async () => {
  const adapter = new PollarAdapter();
  const res = await adapter.createSession({ userAddress: "GMOCKUSERADDRESS" });
  assert.equal(res.ok, true);
  assert.ok(res.data.sessionToken);
});

test("zkproof generate and verify", async () => {
  const adapter = new ZkProofAdapter();
  const generated = adapter.generateProof({ savedAmount: 100, targetAmount: 50, userId: "u1" });
  assert.equal(generated.ok, true);
  const checked = adapter.verifyLocal({ proof: generated.data.proof });
  assert.equal(checked.ok, true);
});
