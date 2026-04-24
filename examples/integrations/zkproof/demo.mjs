import { loadLocalEnv } from "../common/env.mjs";
import { ZkProofAdapter } from "./adapter.mjs";

loadLocalEnv(new URL("..", import.meta.url).pathname);

const adapter = new ZkProofAdapter();
const generated = adapter.generateProof({ savedAmount: 1200, targetAmount: 1000, userId: "user-01" });
console.log(generated);
if (generated.ok) {
  console.log(adapter.verifyLocal({ proof: generated.data.proof }));
  console.log(await adapter.verifyOnChainAttestation({ proof: generated.data.proof, publicInputs: generated.data.publicInputs }));
}
