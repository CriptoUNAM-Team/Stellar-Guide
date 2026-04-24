import { loadLocalEnv } from "../common/env.mjs";
import { DefindexAdapter } from "./adapter.mjs";

loadLocalEnv(new URL("..", import.meta.url).pathname);

const adapter = new DefindexAdapter();
const userAddress = "GMOCKUSERADDRESS";

console.log(await adapter.healthcheck());
console.log(await adapter.getApy());
console.log(await adapter.getBalance({ userAddress }));
console.log(await adapter.deposit({ userAddress, amount: "5000000" }));
console.log(await adapter.withdraw({ userAddress, amount: "1000000" }));
