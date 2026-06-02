# Semana 6 · Introducción a Soroban + Rust esencial

**Módulo 3 · Contratos Soroban** · Teoría + primer despliegue.

## Objetivos de aprendizaje
- Explicar el modelo de ejecución de Soroban (WASM) y su diferencia con la EVM.
- Entender los tipos de almacenamiento (instance / persistent / temporary) y TTL/rent.
- Compilar, desplegar e invocar un contrato en Testnet.

## Lecturas previas
- [../../docs/guia-0-a-builder.md](../../docs/guia-0-a-builder.md)
- [../../docs/contratos-casos-uso.md](../../docs/contratos-casos-uso.md) (secciones iniciales)

## Guion de teoría (≈50 min)
1. Soroban = WASM compilado desde Rust; por qué Rust (seguridad de memoria, no GC). (10 min)
2. Estructura de un contrato: `#[contract]`, `#[contractimpl]`, `Env`. (10 min)
3. Almacenamiento: instance vs persistent vs temporary; **TTL y rent** (el estado expira). (15 min)
4. Fees por recursos: instrucciones CPU, ledger I/O, tamaño de estado. (10 min)
5. Ciclo: `build` → `deploy` → `invoke`. (5 min)

> **Callout "Vienes de Solidity":** no hay EVM ni `msg.sender` global; la autorización es explícita y el
> **estado no es eterno** — debes extender el TTL o se archiva. El "gas" es metering por recursos.

## Demo en vivo
- Recorrer `contracts/counter/src/lib.rs`.
- `stellar contract build` → deploy en Testnet → `invoke` la función y leer el resultado.

## Lab calificado 6 — "Hola Soroban: counter en Testnet"
**Entregable:**
1. Compilar `contracts/counter` a WASM.
2. Desplegarlo en Testnet (guardar el contract ID `C...`).
3. Invocar sus funciones y mostrar el cambio de estado.
4. Modificar el contrato (p. ej. añadir un decremento o un reset) y redesplegar.
5. Reportar: tamaño del WASM, fee de despliegue, fee de invocación.

## Tarea
- Leer sobre Rust básico (ownership, `Option`, `Result`) si no lo dominas.
- Identificar qué tipo de storage usarías para: un contador global, una sesión temporal, un balance de usuario.

## Recursos
- https://developers.stellar.org/docs/build/smart-contracts/getting-started
- https://developers.stellar.org/docs/learn/encyclopedia/storage/state-archival
