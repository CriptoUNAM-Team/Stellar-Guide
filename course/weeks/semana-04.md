# Semana 4 · Activos, trustlines, DEX y AMM

**Módulo 2 · Red clásica y activos** · Teoría + lab de emisión.

## Objetivos de aprendizaje
- Emitir un activo propio y entender el rol del emisor y las trustlines.
- Publicar ofertas en el SDEX y entender path payments.
- Conocer los liquidity pools (AMM) nativos.

## Lecturas previas
- Repaso de "Activos y trustlines" en [../teoria/01-stellar-vs-otras-cadenas.md](../teoria/01-stellar-vs-otras-cadenas.md)
- Docs: emisión de activos (ver Recursos).

## Guion de teoría (≈40 min)
1. Activo = `(código, emisor)`. Cuenta emisora vs distribuidora. (10 min)
2. Trustlines: límites, autorización, flags (auth_required, clawback, freeze). (10 min)
3. SDEX: ofertas de compra/venta en el ledger. (10 min)
4. Path payments (envío X → recepción Y) y AMM/liquidity pools. (10 min)

> **Callout "Vienes de EVM":** no despliegas un ERC-20; **emites** un activo y los receptores abren
> trustlines. El control regulatorio vive en flags del emisor, no en lógica de contrato.

## Demo en vivo
- Crear emisor + distribuidor, emitir `EDU`, abrir trustline, hacer un pago del activo.
- Publicar una oferta `EDU/XLM` y verla en el order book.

## Lab calificado 4 — "Emite tu token y crea mercado"
**Entregable:**
1. Crear cuentas emisora y distribuidora.
2. Emitir un activo propio (código de 4 letras).
3. Abrir trustline desde una tercera cuenta y enviarle tokens.
4. Publicar una oferta en el SDEX y reportar el order book resultante.
5. Explicar: ¿qué flag usarías si tu activo fuera regulado y por qué?

## Tarea
- Diseñar un path payment hipotético (remesa USD→XLM→token local) y diagramarlo en Mermaid.

## Recursos
- https://developers.stellar.org/docs/tokens
- https://developers.stellar.org/docs/learn/fundamentals/liquidity-on-stellar-sdex-liquidity-pools
