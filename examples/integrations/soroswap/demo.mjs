import { loadLocalEnv } from "../common/env.mjs";
import { SoroswapAdapter } from "./adapter.mjs";

loadLocalEnv(new URL("..", import.meta.url).pathname);

const adapter = new SoroswapAdapter();
const fromAsset = process.env.SOROSWAP_DEFAULT_FROM_ASSET || "XLM";
const toAsset = process.env.SOROSWAP_DEFAULT_TO_ASSET || "USDC";

console.log(await adapter.healthcheck());
console.log(await adapter.quote({ fromAsset, toAsset, amount: "100" }));
console.log(await adapter.execute({ fromAsset, toAsset, amount: "25", userAddress: "GMOCKUSERADDRESS" }));
