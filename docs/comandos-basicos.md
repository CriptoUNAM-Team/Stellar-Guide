# Comandos Básicos

## Convenciones rápidas de este documento
- Todas las recetas usan `testnet` por defecto.
- Para montos en operaciones puedes ver valores en stroops (según comando/contexto de CLI); valida en la ayuda del comando antes de usar producción.
- Usa aliases (`alice`, `bob`, `charlie`) para no exponer claves en comandos repetitivos.

## Stellar CLI (recetas y autocompletado)
- Tareas típicas: enviar pagos, gestionar ciclo de vida de contratos, extender instancia/almacenamiento/wasm, y más (ver “Cookbook” del Stellar CLI).
- Autocompletado:
```bash
stellar completion --shell bash
source <(stellar completion --shell bash)
```

## Redes y llaves (Testnet)
```bash
# Usar Testnet
stellar network use testnet

# Generar y FONDEAR identidad (usa Friendbot internamente)
stellar keys generate --fund alice --network testnet

# Generar otra identidad sin fondear
stellar keys generate bob

# Añadir una clave pública existente con alias
stellar keys add --public-key G... charlie

# Ver la llave pública (dirección) de una identidad
stellar keys address alice

# Ver la llave privada (secreta) de una identidad
stellar keys secret alice
```

## Flujo 1: cuenta y pago en Testnet
```bash
# 1) Seleccionar red
stellar network use testnet

# 2) Crear cuenta fuente con fondos
stellar keys generate --fund alice --network testnet

# 3) Crear cuenta destino sin fondos iniciales
stellar keys generate bob

# 4) Crear cuenta en cadena (bob) desde alice
stellar tx new create-account \
  --source alice \
  --destination bob \
  --starting-balance 100_000_000

# 5) Enviar pago nativo
stellar tx new payment \
  --source alice \
  --destination bob \
  --asset native \
  --amount 10_000_000
```

## Fundear cuentas y pagos (tx)
```bash
# Crear y fundear cuenta (bob recibe 10 XLM)
stellar tx new create-account \
  --source alice \
  --destination bob \
  --starting-balance 100_000_000

# Enviar pago nativo (XLM) de bob a charlie: 4 XLM
stellar tx new payment \
  --source bob \
  --destination charlie \
  --asset native \
  --amount 40_000_000
```

## Verificación del flujo de pago
```bash
# Revisar estado de alice
stellar account show --id "$(stellar keys address alice)" --network testnet

# Revisar estado de bob
stellar account show --id "$(stellar keys address bob)" --network testnet
```

## Flujo 2: contrato Soroban en Testnet
```bash
# 1) Crear proyecto
stellar contract init --name counter

# 2) Compilar
stellar contract build --manifest-path contracts/counter/Cargo.toml

# 3) Desplegar
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/counter.wasm \
  --network testnet \
  --source alice

# 4) Invocar función
stellar contract invoke --id <CONTRACT_ID> \
  --source alice \
  --network testnet -- \
  increment
```

## Contratos con Stellar CLI
```bash
# Inicializar proyecto de contrato
stellar contract init --name counter

# Compilar
stellar contract build --manifest-path contracts/counter/Cargo.toml

# Desplegar (WASM ya compilado)
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/counter.wasm \
  --network testnet --source alice

# Invocar función (pasando args tras --)
stellar contract invoke --id <CONTRACT_ID> \
  --source alice --network testnet -- \
  increment
```

## Consulta rápida (opcional)
Si deseas consultar balances/estado de una cuenta con Horizon:
```bash
curl "https://horizon-testnet.stellar.org/accounts/<PUBLIC_KEY>"
```

## Flujo 3: integración con anclas y SEP (mapa de decisión)
No todos los proyectos necesitan los mismos estándares. Usa esta guía rápida:

- Si necesitas **depósito/retiro simple**: revisa `SEP-6`.
- Si necesitas **flujo interactivo web**: revisa `SEP-24`.
- Si necesitas **autenticación wallet-anchor**: revisa `SEP-10`.
- Si necesitas **KYC estructurado**: revisa `SEP-12`.
- Si necesitas **quotes previas**: revisa `SEP-38`.

Referencia completa: [SEP, Estándares y Anclas](sep-estandares-anclas.md).

## SDK JS (snippet mínimo)
```js
import { Asset, Keypair, Server, Networks, TransactionBuilder, Operation } from "@stellar/stellar-sdk";

const server = new Server("https://horizon-testnet.stellar.org");
const pair = Keypair.random();
// Fondea con Friendbot antes de usar

async function pagoSimple(destination, amount) {
  const account = await server.loadAccount(pair.publicKey());
  const tx = new TransactionBuilder(account, { fee: "100", networkPassphrase: Networks.TESTNET })
    .addOperation(Operation.payment({ destination, asset: Asset.native(), amount }))
    .setTimeout(30)
    .build();
  tx.sign(pair);
  const res = await server.submitTransaction(tx);
  console.log(res.hash);
}
```
