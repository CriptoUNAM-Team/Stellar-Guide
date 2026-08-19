# Stellar Guide (Talleres en Español)

Guía visual y práctica para aprender **Stellar + Soroban** desde cero, con rutas por perfil, contratos listos, integraciones reales y playbooks de producto.

![Stellar](https://img.shields.io/badge/Stellar-Network-0A0F1F?style=for-the-badge&logo=stellar&logoColor=white)
![Soroban](https://img.shields.io/badge/Soroban-Smart%20Contracts-4F46E5?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-Contracts-D97706?style=for-the-badge&logo=rust&logoColor=white)
![JavaScript](https://img.shields.io/badge/JavaScript-Integrations-F7DF1E?style=for-the-badge&logo=javascript&logoColor=111111)
![Node.js](https://img.shields.io/badge/Node.js-Tooling-16A34A?style=for-the-badge&logo=node.js&logoColor=white)
![Mermaid](https://img.shields.io/badge/Mermaid-Flows-06B6D4?style=for-the-badge&logo=mermaid&logoColor=white)

## 🎓 Curso de 12 semanas (para impartir clases)

¿Quieres dar un **curso completo** sobre Stellar? Hay una capa pedagógica lista para enseñar en
[`course/`](course/README.md): syllabus, teoría (consenso SCP, arquitectura), plan de clase semana a
semana, labs calificados, banco de quizzes, rúbricas y proyecto final.

- **Programa completo:** [course/syllabus.md](course/syllabus.md)
- **Para docentes (cómo incorporarlo en clase):** [course/programa-docentes.md](course/programa-docentes.md)
- **Mapa del curso:** [course/README.md](course/README.md)
- **Teoría de consenso (SCP/FBA):** [course/teoria/02-consenso-scp.md](course/teoria/02-consenso-scp.md)

El curso reutiliza los `docs/`, `contracts/` y `examples/` de este repo como material de laboratorio.

## 💡 Ideathon de 1 día (formato evento)

¿Necesitas un **evento corto** en lugar de un curso? El programa completo de un ideathon de 8 horas
—orientado a negocio y casos de uso, donde cada equipo publica una demo y la participación se mide
con el historial del repositorio— vive en su propio repo:

**[MarxMad/Ideathon-Stellar-BAF-Canacintra](https://github.com/MarxMad/Ideathon-Stellar-BAF-Canacintra)**

- **Plan de trabajo:** [organizacion/temario.md](https://github.com/MarxMad/Ideathon-Stellar-BAF-Canacintra/blob/main/organizacion/temario.md) · [versión en página](https://marxmad.github.io/Ideathon-Stellar-BAF-Canacintra/plan-de-trabajo.html)
- **Cómo se mide:** [organizacion/metricas.md](https://github.com/MarxMad/Ideathon-Stellar-BAF-Canacintra/blob/main/organizacion/metricas.md)
- **Guía de GitHub para asistentes sin experiencia:** [guia-github.md](https://github.com/MarxMad/Ideathon-Stellar-BAF-Canacintra/blob/main/guia-github.md)

Ese programa usa este repo como material de laboratorio: el catálogo de [`docs/contratos-casos-uso.md`](docs/contratos-casos-uso.md),
los [`contracts/`](contracts) como patrones de referencia y [`frontend/`](frontend) para el nivel avanzado.

**Presentaciones (HTML, tecla N = notas):**

- Programa para docentes: [`presentacion/docentes.html`](presentacion/docentes.html)
- Capacitación sesión 1 (teoría): [`presentacion/capacitacion-sesion-1.html`](presentacion/capacitacion-sesion-1.html)
- Capacitación sesión 2 (CLI y contratos): [`presentacion/capacitacion-sesion-2.html`](presentacion/capacitacion-sesion-2.html)
- Labs CLI / contratos: [`docs/comandos-basicos.md`](docs/comandos-basicos.md)

## Empieza aquí

Guía práctica para aprender haciendo: pagos, contratos e integraciones en Stellar.

## Requisitos mínimos

- Terminal básica.
- Ganas de probar cosas.
- JS o Rust ayudan, pero no son obligatorios para arrancar.

## Mapa rápido

```mermaid
flowchart LR
  start["0. Inicio"] --> docs["1. Fundamentos y CLI"]
  docs --> payments["2. Pagos en Testnet"]
  payments --> contracts["3. Contratos Soroban"]
  contracts --> integrations["4. Integraciones externas"]
  integrations --> playbooks["5. Playbooks E2E"]
  playbooks --> release["6. Checklist pre-mainnet"]
```

## Mini glosario

- **Stellar**: red para pagos y activos digitales.
- **Soroban**: plataforma de smart contracts en Stellar.
- **Testnet**: red de pruebas (sin dinero real).
- **SEP**: estándares para que wallets/proveedores se entiendan entre sí.
- **Adapter**: capa que conecta tu app con un proveedor externo.

## Guía concentrada (todo en un solo lugar)

| Bloque | Qué resuelve | Dónde empezar |
|---|---|---|
| Fundamentos | Cuentas, red, CLI y flujos base | [docs/introduccion.md](docs/introduccion.md) |
| CLI | Llaves, pagos, build/deploy/invoke (CLI 25) | [docs/comandos-basicos.md](docs/comandos-basicos.md) |
| Pagos | Crear cuentas y enviar XLM | [exercises/01-pago-simple.md](exercises/01-pago-simple.md) |
| Contratos | Casos de negocio en Soroban | [docs/contratos-casos-uso.md](docs/contratos-casos-uso.md) |
| Frontend | Probar funciones de contrato desde UI | [docs/frontend-contratos.md](docs/frontend-contratos.md) |
| Integraciones | Conectar protocolos externos | [docs/integraciones-protocolos.md](docs/integraciones-protocolos.md) |
| Operación | Hardening y salida a producción | [docs/checklist-pre-mainnet.md](docs/checklist-pre-mainnet.md) |

## Stack y tecnologías

| Área | Lenguajes/Tecnologías |
|---|---|
| Red y estándares | Stellar, SEP-1, SEP-6, SEP-10, SEP-12, SEP-24, SEP-31, SEP-38 |
| Contratos | Soroban, Rust, WASM |
| Integraciones | JavaScript (Node.js), Fetch API, adapters por proveedor |
| DevEx | Stellar CLI, Cargo, Mermaid, Markdown |

## Arquitectura del repositorio

```mermaid
flowchart LR
  docs["docs/"] --> learn["Onboarding y referencia"]
  exercises["exercises/"] --> practice["Práctica guiada"]
  contracts["contracts/"] --> onchain["Lógica on-chain Soroban"]
  integrations["examples/integrations/"] --> offchain["Adapters off-chain"]
  onchain --> playbooks["Playbooks E2E"]
  offchain --> playbooks
  playbooks --> release["Checklist pre-mainnet"]
```

## Estructura del repositorio

```text
docs/                    Guías, CLI, casos de uso, playbooks
exercises/               Ejercicios prácticos
contracts/               Contratos Soroban (campus, DeFi, nómina, trazabilidad)
course/                  Syllabus 12 semanas + guía para docentes
presentacion/            Decks HTML para proyectar
examples/integrations/   Adapters y demos de integraciones externas
assets/                  Recursos visuales
```

## Rutas recomendadas (elige una)

### Ruta A: Primer contacto (90 min)
1. [docs/introduccion.md](docs/introduccion.md)
2. [docs/instalacion.md](docs/instalacion.md)
3. [docs/comandos-basicos.md](docs/comandos-basicos.md)
4. [docs/flujos-mermaid.md](docs/flujos-mermaid.md)
5. [exercises/01-pago-simple.md](exercises/01-pago-simple.md)

### Ruta B: Builder de contratos
1. [docs/guia-0-a-builder.md](docs/guia-0-a-builder.md)
2. [docs/contratos-casos-uso.md](docs/contratos-casos-uso.md)
3. [docs/comandos-basicos.md](docs/comandos-basicos.md) — build, deploy, invoke (CLI 25)
4. Campus: `attendance` → `voting` → `grades` (sin tokens)
5. Tokens: `escrow`, `payroll`, `loan`, `amm`, `yield`

### Ruta C: Integraciones y producto
1. [docs/sep-estandares-anclas.md](docs/sep-estandares-anclas.md)
2. [docs/integraciones-protocolos.md](docs/integraciones-protocolos.md)
3. `examples/integrations/*`
4. [docs/playbooks-producto.md](docs/playbooks-producto.md)

## Contratos listos para practicar

Invoke y WASM (`wasm32v1-none`): [docs/comandos-basicos.md](docs/comandos-basicos.md).

### Campus (talleres con profesores)

| Contrato | Caso de uso |
|---|---|
| `contracts/attendance` | Lista de asistencia por sesión |
| `contracts/voting` | Propuesta sí/no, un voto por address |
| `contracts/grades` | Calificación por alumno y actividad |
| `contracts/library` | Préstamo de ejemplares con cupo |
| `contracts/enrollment` | Inscripción a curso con `capacity` |
| `contracts/escrow` | Depósito condicional; el árbitro libera o reembolsa |

### DeFi y operaciones

| Contrato | Caso de uso |
|---|---|
| `contracts/payroll` | Dispersión de nómina por periodo (idempotente) |
| `contracts/savings` | Ahorro por metas con penalización temprana |
| `contracts/loan` | Colateral real, LTV, `liquidate` |
| `contracts/yield` | Vault por shares (`deposit` / `harvest` / `withdraw`) |
| `contracts/amm` | Pool `x * y = k`, slippage |
| `contracts/nft-membership` | NFT de membresía / certificado |
| `contracts/food-trace` | Trazabilidad de lotes (cadena alimentaria) |

## Integraciones listas para practicar

| Integración | Carpeta | Operaciones base |
|---|---|---|
| Soroswap | `examples/integrations/soroswap` | `healthcheck`, `quote`, `execute` |
| Etherfuse | `examples/integrations/etherfuse` | `lookupStablebonds`, `quoteOnramp`, `quoteOfframp` |
| Defindex | `examples/integrations/defindex` | `getApy`, `getBalance`, `deposit`, `withdraw` |
| Pollar | `examples/integrations/pollar` | `createSession`, `getRampQuote` |
| ZKProof | `examples/integrations/zkproof` | `generateProof`, `verifyLocal`, `verifyOnChainAttestation` |

## Frontend para probar contratos

UI lista en [`frontend/`](frontend/README.md): la lista se lee sin wallet; Freighter firma. Abre `attendance.html` ligada a

`CBQ2RV6RGJMGOJGJRMV6JFCYIVMBQNPDGUNW57I2YJCG2RQU5T7MD2BO`

```bash
cd frontend && python3 -m http.server 8080
```

Al desplegar otro contrato, pega el `C…` en la página y **Guardar ID**. Detalle de SDK: [docs/frontend-contratos.md](docs/frontend-contratos.md).

## Flujos Mermaid clave

### Flujo 1: De dev local a demo funcional
```mermaid
flowchart LR
  setup["Configurar entorno"] --> tests["Correr tests"]
  tests --> contracts["Probar contratos Soroban"]
  contracts --> adapters["Probar adapters en mock/real"]
  adapters --> e2e["Ejecutar playbook E2E"]
```

### Flujo 2: Producto híbrido (on-chain + off-chain)
```mermaid
sequenceDiagram
  participant User
  participant App
  participant Adapter
  participant Contract
  participant Stellar
  User->>App: Solicita operación (ej. ahorro, swap, préstamo)
  App->>Adapter: Cotiza/consulta proveedor externo
  Adapter-->>App: Datos normalizados
  App->>Contract: Invoca función Soroban
  Contract->>Stellar: Ejecuta transacción
  Stellar-->>App: Hash + estado
  App-->>User: Resultado final
```

## Quickstart (copiar y correr)

### 1) Integraciones (modo demo/mock)
```bash
cd examples/integrations
cp .env.example .env
npm install
INTEGRATIONS_USE_MOCK=true npm test
INTEGRATIONS_USE_MOCK=true npm run smoke:all
```

### 2) Contratos Soroban (tests)
```bash
# Campus (sin tokens)
cargo test -p attendance -p voting -p grades -p library -p enrollment

# Tokens / DeFi
cargo test -p escrow -p payroll -p savings -p loan -p yield -p amm -p nft-membership
```

Build de un contrato:

```bash
stellar contract build --manifest-path contracts/attendance/Cargo.toml
# WASM: target/wasm32v1-none/release/attendance.wasm
```

### 3) Pago rápido en Testnet

`--amount` va en **stroops** (1 XLM = `10_000_000`).

```bash
stellar tx new payment \
  --source alice \
  --destination bob \
  --asset native \
  --amount 10_000_000 \
  --network testnet
```

## Retos para practicar

1. Campus: abre sesión en `attendance`, vota en `voting`, registra nota en `grades`.
2. Llena el cupo en `enrollment` o los ejemplares en `library` y confirma el error.
3. En `escrow`, bloquea 1 XLM y prueba `release` vs `refund`.
4. En `loan`, sube `min_collateral_bps` y liquida; en `amm`, observa el slippage.

## Referencias clave del repo

- Guía principal: [docs/guia-0-a-builder.md](docs/guia-0-a-builder.md)
- CLI y labs: [docs/comandos-basicos.md](docs/comandos-basicos.md)
- SEP y anclas: [docs/sep-estandares-anclas.md](docs/sep-estandares-anclas.md)
- Integraciones: [docs/integraciones-protocolos.md](docs/integraciones-protocolos.md)
- Frontend + contratos: [docs/frontend-contratos.md](docs/frontend-contratos.md)
- Playbooks E2E: [docs/playbooks-producto.md](docs/playbooks-producto.md)
- Checklist producción: [docs/checklist-pre-mainnet.md](docs/checklist-pre-mainnet.md)
- Troubleshooting: [docs/troubleshooting-integraciones.md](docs/troubleshooting-integraciones.md)

## Contrato desplegado (referencia)

| Contrato | ID | Enlaces |
|---|---|---|
| Dispersor de Nóminas | `CBM3OJUPURMLBUN563QN7I62J3SF4OYVIDDN3HPEROCQ3V4AL4VDEZXD` | [Stellar Lab](https://lab.stellar.org/r/testnet/contract/CBM3OJUPURMLBUN563QN7I62J3SF4OYVIDDN3HPEROCQ3V4AL4VDEZXD) · [Stellar Expert](https://stellar.expert/explorer/testnet/contract/CBM3OJUPURMLBUN563QN7I62J3SF4OYVIDDN3HPEROCQ3V4AL4VDEZXD) |

## Licencia

MIT

## Autor

Hecho por **Gerry Vela**.
