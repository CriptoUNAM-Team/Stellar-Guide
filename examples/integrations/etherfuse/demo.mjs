import { loadLocalEnv } from "../common/env.mjs";
import { EtherfuseAdapter } from "./adapter.mjs";

loadLocalEnv(new URL("..", import.meta.url).pathname);

const adapter = new EtherfuseAdapter();
const fiat = process.env.ETHERFUSE_DEFAULT_FIAT || "MXN";

console.log(await adapter.healthcheck());
console.log(await adapter.lookupStablebonds());
console.log(await adapter.quoteOnramp({ fiatAmount: "1000", fiat }));
console.log(await adapter.quoteOfframp({ tokenAmount: "900", fiat }));
