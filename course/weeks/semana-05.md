# Semana 5 · Multifirma y control de cuentas

**Módulo 2 · Red clásica y activos** · Teoría + lab. **Cierre de módulo: Quiz 2.**

## Objetivos de aprendizaje
- Configurar signers y thresholds (low/medium/high) en una cuenta.
- Crear y co-firmar una transacción multisig.
- Entender sponsored reserves y claimable balances.

## Lecturas previas
- Docs de multifirma y control de cuentas (ver Recursos).

## Guion de teoría (≈40 min)
1. Signers, pesos y umbrales por nivel de operación. (15 min)
2. `setOptions`: master weight, thresholds, signers adicionales. (10 min)
3. Sponsored reserves (otra cuenta paga tu reserva) y claimable balances. (15 min)

> **Callout "Vienes de EVM":** no necesitas un contrato tipo Gnosis Safe; la **multisig es nativa** en
> la propia cuenta vía signers + thresholds.

## Demo en vivo
- Convertir una cuenta a multisig 2-de-3; intentar firmar con una sola clave (falla) y con dos (pasa).
- Crear un claimable balance y reclamarlo desde otra cuenta.

## Lab calificado 5 — "Tesorería 2-de-3"
**Entregable:**
1. Configurar una cuenta con 3 signers y umbral medium que requiera 2 firmas.
2. Construir una tx de pago, firmarla con 2 claves y enviarla.
3. Mostrar el intento fallido con 1 firma.
4. Explicar un caso de negocio real (tesorería/DAO) para esta config.

## Quiz 2 (cierre de Módulo 2)
Temas: activos, trustlines, SDEX/AMM, multisig, control de cuentas.
Banco: [../evaluacion/quizzes.md](../evaluacion/quizzes.md#quiz-2).

## Tarea
- Investigar sponsored reserves y escribir cuándo lo usarías para onboarding de usuarios sin XLM.

## Recursos
- https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig
- https://developers.stellar.org/docs/learn/encyclopedia/transactions-specialized/claimable-balances
