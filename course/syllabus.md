# Syllabus — Stellar & Soroban (12 semanas, intensivo)

**Audiencia:** desarrolladores con experiencia previa en blockchain (EVM/Solidity, Cosmos, Solana u otra).
**Modalidad:** intensiva, 12 semanas. ~2 sesiones/semana (teoría + lab).
**Idioma:** español. **Prerrequisitos:** saber programar; deseable nociones de blockchain.

---

## Objetivos generales del curso

Al terminar, el estudiante será capaz de:

1. Explicar el **mecanismo de consenso de Stellar (SCP/FBA)** y contrastarlo con PoW, PoS y BFT clásico.
2. Describir la **arquitectura de la red**: ledger, cuentas, transacciones, operaciones, Horizon y RPC.
3. Operar la **red clásica**: emitir activos, trustlines, DEX/AMM, multifirma y control de cuentas.
4. **Escribir, testear, desplegar e invocar** contratos Soroban en Rust sobre Testnet.
5. Aplicar **patrones de seguridad** y entender el modelo de almacenamiento/fees de Soroban.
6. Integrar **SEPs y anclas** (web auth, KYC, depósitos/retiros, cotizaciones).
7. Llevar un producto **a mainnet** siguiendo un checklist de hardening.

---

## Calendario y temario

### Módulo 1 — Fundamentos de Stellar (semanas 1-3)

| Sem | Tema | Teoría | Lab / Entregable |
|---|---|---|---|
| 1 | Qué es Stellar y por qué. Modelo de cuentas. **Stellar vs EVM/L1s** | [teoria/01-stellar-vs-otras-cadenas.md](teoria/01-stellar-vs-otras-cadenas.md) | Setup + primera cuenta en Testnet (Friendbot) |
| 2 | **Consenso SCP / Federated Byzantine Agreement** | [teoria/02-consenso-scp.md](teoria/02-consenso-scp.md) | Analizar quorum sets reales + reporte |
| 3 | Ledger, transacciones, operaciones, fees. Horizon vs RPC, SDKs | [teoria/03-arquitectura-red-y-ledger.md](teoria/03-arquitectura-red-y-ledger.md) | Pago simple + lectura de ledger ([exercises/01](../exercises/01-pago-simple.md)) |

**Cierre de módulo:** Quiz 1 (teoría de fundamentos y consenso).

### Módulo 2 — Red clásica y activos (semanas 4-5)

| Sem | Tema | Lab / Entregable |
|---|---|---|
| 4 | Emisión de activos, trustlines, SDEX, path payments, AMM | Emitir un token propio + crear una oferta en el DEX |
| 5 | Multifirma, thresholds, sponsored reserves, claimable balances | Cuenta multisig 2-de-3 + transacción co-firmada |

**Cierre de módulo:** Quiz 2 (red clásica y activos).

### Módulo 3 — Contratos inteligentes Soroban (semanas 6-9)

| Sem | Tema | Lab / Entregable |
|---|---|---|
| 6 | Intro a Soroban. Rust esencial. Modelo de ejecución y storage (instance/persistent/temporary), TTL/rent | Compilar, desplegar e invocar `contracts/counter` en Testnet |
| 7 | Tipos, autorización (`require_auth`), eventos, errores, testing | Extender y testear `contracts/payroll` |
| 8 | Stellar Asset Contract (SAC), tokens, llamadas cross-contract, composabilidad | `contracts/savings` + `contracts/yield` integrados |
| 9 | Seguridad, upgrades, optimización de recursos/fees, auditoría | `contracts/loan` o `nft-membership` + security review |

**Cierre de módulo:** Quiz 3 (Soroban) + checkpoint del proyecto final.

### Módulo 4 — SEPs, anclas e integraciones (semanas 10-11)

| Sem | Tema | Lab / Entregable |
|---|---|---|
| 10 | SEPs a fondo: SEP-1, SEP-10 (web auth), SEP-12 (KYC), SEP-24/6, SEP-31, SEP-38. Anclas | Implementar flujo de autenticación SEP-10 |
| 11 | Integraciones de protocolos + frontend con wallet (Freighter) | Conectar un adapter ([examples/integrations](../examples/integrations)) a una UI |

**Cierre de módulo:** Quiz 4 (SEPs e integraciones).

### Módulo 5 — Producción y proyecto final (semana 12)

| Sem | Tema | Entregable |
|---|---|---|
| 12 | Hardening de mainnet, observabilidad, checklist. **Presentaciones** | Proyecto final + demo |

**Cierre:** Quiz 5 (producción) + **defensa del proyecto final**.

---

## Evaluación

| Componente | Peso | Detalle |
|---|---|---|
| Quizzes por módulo (5) | 20% | 4% cada uno · [evaluacion/quizzes.md](evaluacion/quizzes.md) |
| Labs calificados | 35% | ~10 labs · rúbrica en [evaluacion/rubricas.md](evaluacion/rubricas.md) |
| Proyecto final | 35% | Especificación en [evaluacion/proyecto-final.md](evaluacion/proyecto-final.md) |
| Participación / portafolio | 10% | Asistencia, foro, repo de labs |

**Aprobación:** ≥ 70% global y proyecto final aprobado.

### Política de entregas
- Labs se entregan en el repo personal del estudiante (link en LMS).
- Tardanzas: −10% por día, hasta 3 días; después no se recibe.
- Integridad: usar IA está permitido para aprender, **pero** debes poder explicar tu código en la defensa.

---

## Materiales y herramientas

- **Software:** Node.js LTS, Rust + cargo, Stellar CLI, Git. Setup: [../docs/instalacion.md](../docs/instalacion.md).
- **Cuentas:** Testnet (gratis, vía Friendbot). Mainnet solo en semana 12 (opcional).
- **Wallet:** Freighter (extensión de navegador).
- **Exploradores:** [Stellar Lab](https://lab.stellar.org), [Stellar Expert](https://stellar.expert).
- **Docs oficiales:** https://developers.stellar.org

Glosario del curso: [recursos/glosario.md](recursos/glosario.md).
