# Frontend para probar contratos Soroban

Esta guía te permite levantar un frontend simple para invocar funciones de contrato y ver respuestas.

## Objetivo

- Conectar wallet/cuenta.
- Invocar funciones (`view` y `write`) de un contrato.
- Mostrar resultado en UI.

## Estructura sugerida

```text
frontend/
  src/
    lib/stellar.js
    app.js
  package.json
```

## Dependencias mínimas

```bash
npm init -y
npm install @stellar/stellar-sdk
```

## Configuración base (`src/lib/stellar.js`)

```js
import { Networks, rpc, TransactionBuilder, Account, BASE_FEE } from "@stellar/stellar-sdk";

const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = Networks.TESTNET;

export const soroban = new rpc.Server(RPC_URL);
export { NETWORK_PASSPHRASE, TransactionBuilder, Account, BASE_FEE };
```

## Conectar frontend con contrato (paso a paso)

1. Define `contractId` y función a invocar.
2. Construye transacción de invocación.
3. Simula con RPC (opcional recomendado).
4. Firma con wallet/secret.
5. Envía y espera confirmación.
6. Renderiza resultado en la UI.

## Ejemplo mínimo de invocación

```js
import { Keypair, rpc, scValToNative, xdr } from "@stellar/stellar-sdk";
import { soroban, NETWORK_PASSPHRASE, TransactionBuilder, Account, BASE_FEE } from "./lib/stellar.js";

export async function invokeContract({
  contractId,
  sourcePublicKey,
  sourceSecret,
  method,
  args = [],
}) {
  const accountData = await soroban.getAccount(sourcePublicKey);
  const source = new Account(accountData.accountId(), accountData.sequenceNumber());

  const tx = new TransactionBuilder(source, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(
      xdr.Operation.invokeHostFunction(
        xdr.HostFunction.hostFunctionTypeInvokeContract(),
        xdr.InvokeContractArgs.invokeContractArgs(
          xdr.ScAddress.scAddressTypeContract(xdr.Hash.fromXDR(contractId, "hex")),
          xdr.ScSymbol.scSymbol(method),
          args
        )
      )
    )
    .setTimeout(30)
    .build();

  const simulated = await soroban.simulateTransaction(tx);
  if (simulated.error) throw new Error(simulated.error);

  const prepared = rpc.assembleTransaction(tx, simulated).build();
  prepared.sign(Keypair.fromSecret(sourceSecret));

  const sent = await soroban.sendTransaction(prepared);
  if (sent.status !== "PENDING") throw new Error(`sendTransaction status: ${sent.status}`);

  let result;
  do {
    result = await soroban.getTransaction(sent.hash);
    await new Promise((r) => setTimeout(r, 1200));
  } while (result.status === "NOT_FOUND");

  if (result.status !== "SUCCESS") throw new Error(`tx status: ${result.status}`);
  return result.returnValue ? scValToNative(result.returnValue) : null;
}
```

## Ejemplo UI rápida (`src/app.js`)

```js
import { invokeContract } from "./invoke.js";

async function run() {
  const output = document.getElementById("output");
  output.textContent = "Invocando contrato...";
  try {
    const value = await invokeContract({
      contractId: "<CONTRACT_ID_HEX>",
      sourcePublicKey: "<G...>",
      sourceSecret: "<S...>",
      method: "get_total",
      args: [],
    });
    output.textContent = `OK: ${JSON.stringify(value)}`;
  } catch (e) {
    output.textContent = `Error: ${e.message}`;
  }
}

document.getElementById("run").addEventListener("click", run);
```

## Recomendaciones prácticas

- Empieza con funciones de lectura (`get_*`) para validar conexión.
- No hardcodees secretos en frontend de producción.
- Usa backend signer o wallet provider para operaciones reales.
- Reutiliza este patrón para `payroll`, `savings`, `loan`, `yield` y `nft-membership`.
