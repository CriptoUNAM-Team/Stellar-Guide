# Semana 8 · SAC, tokens y composabilidad (cross-contract)

**Módulo 3 · Contratos Soroban** · Teoría + lab integrando `savings` + `yield`.

## Objetivos de aprendizaje
- Usar el Stellar Asset Contract (SAC) para mover activos desde un contrato.
- Realizar llamadas cross-contract (composabilidad).
- Integrar dos contratos en un flujo único.

## Lecturas previas
- `contracts/savings/src/lib.rs` y `contracts/yield/src/lib.rs`.

## Guion de teoría (≈45 min)
1. SAC: cómo un activo clásico se expone como contrato token (interfaz estándar). (15 min)
2. Interfaz token (`transfer`, `balance`, `approve`) y `token::Client`. (10 min)
3. Llamadas cross-contract: `Client::new(&env, &contract_id)`; cuidado con reentrada y auth. (15 min)
4. Patrón de composición: un contrato que mueve fondos a otro. (5 min)

> **Callout "Vienes de EVM":** la composabilidad existe pero el estado tiene TTL y la auth se propaga
> explícitamente; no asumas "approve infinito" ni estado eterno entre llamadas.

## Demo en vivo
- Crear un SAC para un activo de prueba e invocar `transfer` desde la CLI.
- Mostrar una llamada de `savings` hacia `yield` (depósito que entra a la bóveda por shares).

## Lab calificado 8 — "Ahorro con rendimiento"
**Entregable:**
1. Desplegar `savings` y `yield` en Testnet.
2. Implementar/usar un flujo: crear meta en `savings` → mover fondos a `yield` (shares).
3. Consultar balance de shares y explicar el cálculo.
4. Escribir un test de integración que cubra depósito y retiro.
5. Reportar los contract IDs y los hashes de las invocaciones.

Referencia de playbook: [../../docs/playbooks-producto.md](../../docs/playbooks-producto.md) (Ahorro + Yield).

## Tarea
- Empezar a definir tu **proyecto final** (idea + contratos que usarás). Checkpoint en semana 9.

## Recursos
- https://developers.stellar.org/docs/tokens/stellar-asset-contract
- https://developers.stellar.org/docs/build/smart-contracts/example-contracts/cross-contract-call
