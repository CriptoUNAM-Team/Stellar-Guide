# Semana 11 · Integraciones de protocolos + frontend con wallet

**Módulo 4 · SEPs e integraciones** · Teoría + lab. **Cierre de módulo: Quiz 4.**

## Objetivos de aprendizaje
- Conectar una app a protocolos del ecosistema vía adapters.
- Integrar una wallet de navegador (Freighter) para firmar.
- Invocar un contrato Soroban desde una UI.

## Lecturas previas
- [../../docs/integraciones-protocolos.md](../../docs/integraciones-protocolos.md)
- [../../docs/frontend-contratos.md](../../docs/frontend-contratos.md)

## Guion de teoría (≈40 min)
1. Patrón adapter: normalizar proveedores externos (modo mock vs real). (10 min)
2. Matriz de integraciones del repo: Soroswap, Defindex, Etherfuse, Pollar, ZKProof. (10 min)
3. Wallets y firma en el navegador (Freighter); flujo connect → build → sign → submit. (10 min)
4. Invocar contrato desde JS/TS (`@stellar/stellar-sdk` + Soroban RPC). (10 min)

> **Callout "Vienes de EVM":** Freighter ≈ MetaMask; el patrón connect/sign/submit es familiar, pero
> firmas transacciones Stellar (XDR) y usas Soroban RPC en vez de JSON-RPC EVM.

## Demo en vivo
- Levantar `examples/integrations` en modo mock y correr un smoke test.
- Conectar Freighter en una página simple e invocar el `counter` desplegado.

## Lab calificado 11 — "Mini-dApp"
**Entregable:**
1. Levantar los adapters ([../../examples/integrations](../../examples/integrations)) y correr `smoke:all` en mock.
2. Construir una UI mínima que conecte Freighter y muestre la cuenta.
3. Invocar una función de un contrato tuyo (de Módulo 3) desde la UI y mostrar el resultado.
4. Conectar **un** adapter (p. ej. quote de Soroswap/Defindex) y mostrar su salida en pantalla.

## Quiz 4 (cierre de Módulo 4)
Temas: SEPs, anclas, web auth, integraciones, frontend/wallet.
Banco: [../evaluacion/quizzes.md](../evaluacion/quizzes.md#quiz-4).

## Tarea
- Finalizar el desarrollo del proyecto final; preparar demo y slides para la semana 12.

## Recursos
- https://developers.stellar.org/docs/build/apps
- Freighter: https://www.freighter.app
