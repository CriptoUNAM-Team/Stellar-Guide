# Semana 1 · Qué es Stellar y por qué (Stellar vs otras cadenas)

**Módulo 1 · Fundamentos** · Sesión teoría + lab de setup.

## Objetivos de aprendizaje
Al terminar la semana, el estudiante podrá:
- Explicar para qué fue diseñada Stellar y por qué sus operaciones base son nativas.
- Contrastar el modelo de cuentas y activos de Stellar con el de EVM.
- Tener su entorno listo y crear/fondear una cuenta en Testnet.

## Lecturas previas
- [../teoria/01-stellar-vs-otras-cadenas.md](../teoria/01-stellar-vs-otras-cadenas.md)
- [../../docs/introduccion.md](../../docs/introduccion.md)

## Guion de teoría (≈45 min)
1. Historia y propósito: red de pagos/activos → Soroban después. (5 min)
2. Cuentas nativas, reservas mínimas, claves Ed25519 (`G/S/C`). (10 min)
3. Activos `(código, emisor)` y trustlines: no hay ERC-20. (10 min)
4. SDEX, path payments y AMM nativos. (10 min)
5. Tabla maestra de diferencias con EVM + cuándo conviene Stellar. (10 min)

> **Callout "Vienes de EVM":** en Stellar pagar, emitir y cambiar activos son **operaciones del
> protocolo**, no contratos. Esto cambia cómo diseñas tu producto.

## Demo en vivo
- Mostrar [Stellar Lab](https://lab.stellar.org) y [Stellar Expert](https://stellar.expert): explorar una
  cuenta, sus balances y trustlines.
- Mostrar una transacción real con múltiples operaciones.

## Lab calificado 1 — "Tu primera cuenta"
**Entregable:** captura + texto en el repo personal con:
1. Instalar entorno ([../../docs/instalacion.md](../../docs/instalacion.md)): Node, Rust, Stellar CLI.
2. Generar un keypair y fondearlo con Friendbot en Testnet.
3. Verificar la cuenta en Stellar Expert y reportar: balance, reserva mínima, secuencia.
4. Responder: ¿qué 3 diferencias con tu cadena previa te sorprendieron más?

Rúbrica: [../evaluacion/rubricas.md](../evaluacion/rubricas.md) (rúbrica de lab).

## Tarea
- Leer [../teoria/02-consenso-scp.md](../teoria/02-consenso-scp.md) antes de la semana 2.
- Escribir media página: "¿Por qué creo que Stellar prioriza safety sobre liveness?" (hipótesis previa).

## Recursos
- https://developers.stellar.org/docs/learn/fundamentals
- Glosario: [../recursos/glosario.md](../recursos/glosario.md)
