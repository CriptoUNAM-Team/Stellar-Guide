import { loadLocalEnv } from "./common/env.mjs";
import { SoroswapAdapter } from "./soroswap/adapter.mjs";
import { EtherfuseAdapter } from "./etherfuse/adapter.mjs";
import { DefindexAdapter } from "./defindex/adapter.mjs";
import { PollarAdapter } from "./pollar/adapter.mjs";
import { ZkProofAdapter } from "./zkproof/adapter.mjs";

loadLocalEnv(new URL(".", import.meta.url).pathname);
process.env.INTEGRATIONS_USE_MOCK = process.env.INTEGRATIONS_USE_MOCK || "true";

const soroswap = new SoroswapAdapter();
const etherfuse = new EtherfuseAdapter();
const defindex = new DefindexAdapter();
const pollar = new PollarAdapter();
const zk = new ZkProofAdapter();

console.log(await soroswap.quote({ fromAsset: "XLM", toAsset: "USDC", amount: "100" }));
console.log(await etherfuse.lookupStablebonds());
console.log(await defindex.getApy());
console.log(await pollar.createSession({ userAddress: "GMOCKUSERADDRESS" }));
const p = zk.generateProof({ savedAmount: 10, targetAmount: 5, userId: "demo" });
console.log(p);
