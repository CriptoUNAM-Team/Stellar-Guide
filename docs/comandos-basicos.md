# Comandos Básicos

Recetas para talleres en **Testnet** con Stellar CLI **25**. Ejecuta los contratos desde la raíz de este repo (`Stellar-Guide`).

## Convenciones

- **Red**: `stellar network use testnet` (o `--network testnet` en cada comando).
- **Identidades**: aliases (`alice`, `bob`, `profe`, `alumno`) — no pegues secretas en el chat ni en slides.
- **Stroops**: en `stellar tx new payment` / `create-account`, `--amount` va en stroops.

| XLM | Stroops |
|-----|---------|
| 1   | `10_000_000` |
| 10  | `100_000_000` |
| 0.5 | `5_000_000` |

- **Contratos**: todo lo que va *después* de `--` son argumentos de la función (`--arg valor`).
- **WASM**: el target actual es `wasm32v1-none` (no `wasm32-unknown-unknown`).
- **Error `#1`**: casi siempre `NotInitialized`. Falta `initialize` o usaste otro `--id`.

Autocompletado:

```bash
stellar completion --shell zsh
# o: bash | fish
```

## Redes y llaves

```bash
stellar network use testnet

# Generar y fondear (Friendbot)
stellar keys generate --fund alice --network testnet

# Generar sin fondear
stellar keys generate bob

# Fondear una identidad ya creada
stellar keys fund bob --network testnet

# Alias solo con clave pública (no firma)
stellar keys add charlie --public-key G...

stellar keys public-key alice    # alias: stellar keys address alice
stellar keys secret alice        # no proyectar en clase
stellar keys ls
```

## Flujo 1: cuenta y pago en Testnet

```bash
stellar network use testnet
stellar keys generate --fund alice --network testnet
stellar keys generate bob

# Crear cuenta on-chain: bob recibe 10 XLM
stellar tx new create-account \
  --source alice \
  --destination bob \
  --starting-balance 100_000_000 \
  --network testnet

# Pago nativo: 1 XLM
stellar tx new payment \
  --source alice \
  --destination bob \
  --asset native \
  --amount 10_000_000 \
  --network testnet
```

Verificar (no existe `stellar account show` en CLI 25):

```bash
curl -s "https://horizon-testnet.stellar.org/accounts/$(stellar keys public-key bob)" | head
stellar ledger latest --network testnet
```

## Flujo 2: contrato Soroban (este repo)

Compila **desde la raíz** con `--manifest-path`, o entra a `contracts/<nombre>`.

```bash
# Tests (sin red)
cargo test --manifest-path contracts/attendance/Cargo.toml

# Compilar
stellar contract build --manifest-path contracts/attendance/Cargo.toml

# WASM: target/wasm32v1-none/release/<crate>.wasm
# attendance → attendance.wasm | voting → voting.wasm | grades → grades.wasm

stellar contract deploy \
  --wasm target/wasm32v1-none/release/attendance.wasm \
  --source alice \
  --network testnet
# imprime CONTRACT_ID (C...)
```

Casi todos los ejemplos de este repo piden `initialize` **una vez**:

```bash
stellar contract invoke --id <CONTRACT_ID> \
  --source alice --network testnet -- \
  initialize --admin alice
```

`--admin alice` acepta el alias. Lecturas (sin cambiar estado) también usan `invoke`.

Ayuda del contrato ya desplegado:

```bash
stellar contract invoke --id <CONTRACT_ID> \
  --source alice --network testnet -- --help
```

Plantilla vacía (si quieres un proyecto nuevo fuera del repo):

```bash
stellar contract init --name mi-contrato
```

## Labs campus (sesión técnica)

Identidades sugeridas: `profe` (admin) y `alumno`.

```bash
stellar keys generate --fund profe --network testnet
stellar keys generate --fund alumno --network testnet
```

### Asistencia — `contracts/attendance`

```bash
stellar contract build --manifest-path contracts/attendance/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/attendance.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- initialize --admin profe
stellar contract invoke --id "$ID" --source profe --network testnet -- \
  open_session --course '"Redes"' --topic '"Ledger y fees"'
# imprime session_id, p. ej. 1

stellar contract invoke --id "$ID" --source profe --network testnet -- \
  mark_present --session_id 1 --student alumno

stellar contract invoke --id "$ID" --source profe --network testnet -- \
  is_present --session_id 1 --student alumno
```

### Votación — `contracts/voting`

```bash
stellar contract build --manifest-path contracts/voting/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/voting.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- initialize --admin profe
stellar contract invoke --id "$ID" --source profe --network testnet -- \
  create_proposal --title '"Laboratorio en Testnet"'

stellar contract invoke --id "$ID" --source alumno --network testnet -- \
  vote --proposal_id 1 --voter alumno --support true

stellar contract invoke --id "$ID" --source profe --network testnet -- \
  get_proposal --proposal_id 1
```

### Calificaciones — `contracts/grades`

```bash
stellar contract build --manifest-path contracts/grades/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/grades.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- initialize --admin profe
stellar contract invoke --id "$ID" --source profe --network testnet -- \
  record_grade --student alumno --assignment '"Lab-1"' --score 85 --max_score 100

stellar contract invoke --id "$ID" --source alumno --network testnet -- \
  get_grade --student alumno --assignment '"Lab-1"'
```

### Biblioteca — `contracts/library`

Cupos de un ejemplar: el alumno toma y devuelve. Enseña máquina de estados.

```bash
stellar contract build --manifest-path contracts/library/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/library.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- initialize --admin profe
stellar contract invoke --id "$ID" --source profe --network testnet -- \
  add_title --title '"Whitepaper Stellar"' --copies 2

stellar contract invoke --id "$ID" --source alumno --network testnet -- \
  checkout --title_id 1 --student alumno

stellar contract invoke --id "$ID" --source alumno --network testnet -- \
  return_copy --title_id 1 --student alumno
```

### Inscripción — `contracts/enrollment`

Curso con cupo. El segundo `enroll` con el mismo alumno falla; al llenar el cupo también.

```bash
stellar contract build --manifest-path contracts/enrollment/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/enrollment.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- initialize --admin profe
stellar contract invoke --id "$ID" --source profe --network testnet -- \
  create_course --name '"Soroban 101"' --capacity 2

stellar contract invoke --id "$ID" --source alumno --network testnet -- \
  enroll --course_id 1 --student alumno
```

### Depósito condicional — `contracts/escrow`

El pagador bloquea XLM nativo (SAC de Testnet). El árbitro (`profe`) libera al beneficiario o devuelve al pagador.

SAC nativo Testnet: `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`

```bash
TOKEN=CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
stellar contract build --manifest-path contracts/escrow/Cargo.toml
ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/escrow.wasm --source profe --network testnet)

stellar contract invoke --id "$ID" --source profe --network testnet -- \
  initialize --arbiter profe --token "$TOKEN"

# 1 XLM bloqueado (stroops en el contrato = i128)
stellar contract invoke --id "$ID" --source alice --network testnet -- \
  lock --payer alice --payee alumno --amount 10000000

stellar contract invoke --id "$ID" --source profe --network testnet -- \
  release --deal_id 1
# o: refund --deal_id 1
```

## Labs DeFi (si hay tiempo)

| Contrato | Qué demostrar |
|----------|----------------|
| `contracts/loan` | colateral real, `borrow`, overborrow, `liquidate` |
| `contracts/yield` | `deposit` / `harvest` / `withdraw` por shares |
| `contracts/amm` | `x * y = k`, slippage |
| `contracts/payroll` | pagos por lote, no pagar dos veces el mismo periodo |
| `contracts/savings` | `unlock_time` y penalización |

Build: `stellar contract build --manifest-path contracts/<nombre>/Cargo.toml`.  
WASM: `target/wasm32v1-none/release/<nombre>.wasm` (`nft-membership` → `nft_membership.wasm`, `food-trace` → `food_trace.wasm`).

## Errores frecuentes en clase

| Síntoma | Causa típica |
|---------|----------------|
| `error #1` | No corriste `initialize`, o el `--id` es de otro deploy |
| `cannot find wasm` | Ruta vieja `wasm32-unknown-unknown`; usa `wasm32v1-none` |
| `unrecognized subcommand 'account'` | CLI 25 no tiene `stellar account`; usa Horizon o `keys public-key` |
| Auth / simulation fail | La `--source` no es quien debe firmar (`admin` vs `student`) |
| Strings en invoke | Pasa `'"texto"'` (comillas internas) en `--course`, `--title`, `--assignment` |

## Flujo 3: anclas y SEP (mapa rápido)

- Depósito/retiro simple → `SEP-6`
- Flujo web interactivo → `SEP-24`
- Auth wallet–ancla → `SEP-10`
- KYC → `SEP-12`
- Cotizaciones → `SEP-38`

Detalle: [SEP, Estándares y Anclas](sep-estandares-anclas.md).

## SDK JS (pago clásico Horizon)

Montos aquí van en **XLM** (string), no en stroops.

```js
import { Asset, Horizon, Keypair, Networks, TransactionBuilder, Operation } from "@stellar/stellar-sdk";

const server = new Horizon.Server("https://horizon-testnet.stellar.org");
const pair = Keypair.random();

async function pagoSimple(destination, amountXlm) {
  const account = await server.loadAccount(pair.publicKey());
  const tx = new TransactionBuilder(account, { fee: "100", networkPassphrase: Networks.TESTNET })
    .addOperation(Operation.payment({ destination, asset: Asset.native(), amount: amountXlm }))
    .setTimeout(30)
    .build();
  tx.sign(pair);
  const res = await server.submitTransaction(tx);
  console.log(res.hash);
}
```
