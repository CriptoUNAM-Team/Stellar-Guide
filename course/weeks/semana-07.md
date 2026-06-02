# Semana 7 · Autorización, eventos, errores y testing

**Módulo 3 · Contratos Soroban** · Teoría + lab sobre `payroll`.

## Objetivos de aprendizaje
- Usar `require_auth` para autorización explícita por dirección.
- Emitir eventos y definir errores tipados con `#[contracterror]`.
- Escribir tests unitarios con el entorno de pruebas de Soroban.

## Lecturas previas
- `contracts/payroll/src/lib.rs` y `contracts/payroll/src/test.rs`.

## Guion de teoría (≈50 min)
1. Modelo de autorización de Soroban: `Address`, `require_auth`, `require_auth_for_args`. (15 min)
2. Integración con firmas de cuenta y multisig nativa. (5 min)
3. Eventos: para qué sirven y cómo los consumen los indexadores. (10 min)
4. Errores tipados y manejo determinista. (10 min)
5. Testing: `Env::default()`, mocks de auth, snapshots. (10 min)

> **Callout "Vienes de Solidity":** no hay `require(msg.sender == ...)`; declaras `addr.require_auth()` y
> el framework verifica la firma/autorización del invocador de forma explícita y componible.

## Demo en vivo
- Recorrer la lógica de `payroll`: cómo autoriza al administrador y cómo evita doble dispersión por periodo.
- Correr `cargo test -p payroll` y leer un test.

## Lab calificado 7 — "Nómina robusta"
**Entregable:**
1. Correr y entender los tests de `payroll`.
2. Añadir una función nueva (p. ej. `pause()` / `resume()` con auth de admin).
3. Escribir **2 tests** nuevos: uno de caso feliz y uno de error esperado (`#[should_panic]` o assert de error).
4. Emitir un evento al dispersar y verificarlo en un test.
5. Reportar cobertura de casos y decisiones de autorización.

## Tarea
- Auditar mentalmente `savings`: ¿qué funciones deberían requerir auth y de quién?

## Recursos
- https://developers.stellar.org/docs/build/smart-contracts/example-contracts/auth
- https://developers.stellar.org/docs/build/guides/testing
