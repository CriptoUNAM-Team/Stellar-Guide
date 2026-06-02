# Teoría · Stellar vs otras blockchains (para quien viene de EVM y similares)

> Lectura del **Módulo 1, Semana 1**. Pensada para devs que ya conocen otra cadena.
> Objetivo: recablear tu modelo mental antes de tocar código.

---

## 1. ¿Qué problema resuelve Stellar?

Stellar nació (2014, SDF) como una red de **pagos y emisión de activos** rápida y barata, con foco en
**mover valor entre monedas y fronteras** (remesas, stablecoins, tokenización). No empezó como "computadora
mundial" sino como **red de liquidación**. Soroban (contratos inteligentes) llegó después para añadir
programabilidad **encima** de esa base de pagos.

Esto explica casi todas las diferencias con EVM: en Stellar, **pagar, emitir activos e intercambiar son
operaciones nativas del protocolo**, no contratos que alguien tuvo que escribir.

---

## 2. Diferencias de modelo (tabla maestra)

| Tema | EVM (Ethereum y compatibles) | Stellar |
|---|---|---|
| Unidad de cuenta | Cuenta/contrato con balance ETH | **Cuenta nativa** (Ed25519), direcciones `G...` |
| Operaciones básicas | Todo es una llamada a contrato | **Operaciones nativas**: pago, oferta, trustline, etc. |
| Activos | Cada token es un contrato ERC-20 | **Activos de primera clase** + *trustlines* |
| Intercambio | DEX = contratos (Uniswap…) | **DEX integrado al protocolo** (SDEX) + AMM nativo |
| Contratos | Solidity → bytecode EVM | **Rust → WASM** (Soroban) |
| Direcciones de contrato | `0x...` | `C...` |
| Gas / fees | Gas auction, fees altas y variables | Fee base 100 stroops; muy bajo; *surge pricing* en congestión |
| Finalidad | Probabilística / epochs | **Determinista, ~5 s** (ver [consenso](02-consenso-scp.md)) |
| Estado de contrato | Persiste para siempre | **Tiene TTL / rent**: el estado expira si no se renueva |
| Multifirma | Vía contrato (Gnosis Safe…) | **Nativa** (signers + thresholds en la cuenta) |
| Mempool / MEV | Mempool público, MEV fuerte | Sin subasta de gas equivalente; modelo distinto |

---

## 3. Cuentas: lo que más sorprende al venir de EVM

- Una cuenta Stellar **debe existir on-chain** (creada y fondeada con un **mínimo de reserva** en XLM).
  No basta con "tener una clave". Cada entrada de datos (trustline, oferta, signer) **sube tu reserva mínima**.
- Las claves son **Ed25519**: pública `G...`, secreta `S...`. Las semillas/contratos usan otros prefijos
  (`C...` contrato, `M...` muxed, etc.).
- La cuenta lleva **directamente**: balances de cada activo, trustlines, signers, thresholds, datos. No
  necesitas un contrato para tener un token.

> **Reserva base:** mantener una cuenta cuesta una reserva mínima en XLM (bloqueada, no gastada).
> Esto previene spam de cuentas/estado — es el equivalente conceptual al "rent" del estado.

---

## 4. Activos y trustlines (no hay ERC-20)

En Stellar un activo es `(código, emisor)`, p. ej. `USDC:GA5Z...`. Para **recibir** un activo no nativo,
tu cuenta debe abrir una **trustline** hacia ese emisor (declaras explícitamente que confías en él y hasta
qué límite). Implicaciones:

- No existe "approve infinito" ni el patrón ERC-20; el control está en la **trustline**.
- El emisor puede tener flags (autorización requerida, *clawback*, congelar) — útil para activos regulados.
- XLM (nativo) **no** requiere trustline.

---

## 5. Intercambio integrado: SDEX y AMM

Stellar trae un **exchange descentralizado en el propio protocolo**:

- **SDEX (order book):** publicas ofertas (`manage offer`) que viven en el ledger.
- **Path payments:** envías el activo X y el receptor recibe el activo Y, encontrando la ruta de
  conversión automáticamente a través de ofertas y pools. Clave para remesas/FX.
- **AMM (liquidity pools):** pools de producto constante nativos.

En EVM todo esto son contratos de terceros; en Stellar es parte del núcleo.

---

## 6. Contratos: Soroban en una frase

Soroban ejecuta **WASM** (compilado desde **Rust**), con un modelo de **almacenamiento con TTL** y **fees
medidos por recursos** (instrucciones de CPU, lecturas/escrituras de ledger, tamaño de estado). Lo
veremos a fondo en el Módulo 3, pero la idea para hoy:

- No es la EVM; no hay `msg.sender` global ni gas como lo conoces.
- El estado **no es eterno**: hay tipos `instance`, `persistent` y `temporary`, y debes **extender el TTL**
  (pagar rent) o el dato se archiva.
- La autorización es explícita (`require_auth`) y se integra con la multifirma nativa y los SEPs.

---

## 7. ¿Cuándo conviene Stellar?

**Muy bien:** pagos/remesas, stablecoins y on/off-ramps (anclas + SEPs), FX y tokenización de activos,
microtransacciones (fees mínimas), productos que necesitan **finalidad rápida y costos predecibles**.

**Piénsalo:** DeFi ultra-componible con un ecosistema EVM gigante ya hecho — Soroban es más joven que la
EVM y su tooling/composability sigue creciendo.

---

## 8. Glosario exprés de equivalencias

| Vienes diciendo… | En Stellar es… |
|---|---|
| "wallet address `0x..`" | cuenta `G...` |
| "deploy un ERC-20" | emitir un activo `(código, emisor)` + trustlines |
| "Uniswap" | SDEX / liquidity pools nativos + path payments |
| "gas" | fees por recurso (Soroban) / fee base (clásico) |
| "Gnosis Safe" | multisig nativa (signers + thresholds) |
| "el contrato guarda estado para siempre" | storage con **TTL/rent** (extiende o se archiva) |
| "Solidity" | **Rust** (Soroban) |
| "JSON-RPC node" | **Horizon** (REST clásico) y/o **Soroban RPC** |

> Siguiente lectura: [02-consenso-scp.md](02-consenso-scp.md).
