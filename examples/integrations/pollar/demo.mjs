import { loadLocalEnv } from "../common/env.mjs";
import { PollarAdapter } from "./adapter.mjs";

loadLocalEnv(new URL("..", import.meta.url).pathname);

const adapter = new PollarAdapter();
const userAddress = "GMOCKUSERADDRESS";

console.log(await adapter.healthcheck());
console.log(await adapter.createSession({ userAddress }));
console.log(await adapter.getRampQuote({ fiat: "MXN", amount: "1000" }));
