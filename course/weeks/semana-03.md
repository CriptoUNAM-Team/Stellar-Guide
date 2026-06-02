# Semana 3 · Ledger, transacciones, fees y APIs (Horizon vs RPC)

**Módulo 1 · Fundamentos** · Teoría + primer lab de pago. **Cierre de módulo: Quiz 1.**

## Objetivos de aprendizaje
- Describir la estructura del ledger y la jerarquía transacción → operaciones.
- Explicar fees, número de secuencia y reservas mínimas.
- Elegir entre Horizon y Soroban RPC según el caso.
- Enviar un pago en Testnet y leer el resultado.

## Lecturas previas
- [../teoria/03-arquitectura-red-y-ledger.md](../teoria/03-arquitectura-red-y-ledger.md)
- [../../docs/comandos-basicos.md](../../docs/comandos-basicos.md)

## Guion de teoría (≈45 min)
1. El ledger como estado mundial; finalidad e history archives. (10 min)
2. Transacción atómica con 1..100 operaciones; tipos de operación. (10 min)
3. Fees, surge pricing, secuencia, timebounds. (10 min)
4. Horizon vs Soroban RPC; SDKs y CLI; XDR. (10 min)
5. Testnet/Futurenet/Mainnet y passphrases. (5 min)

> **Callout "Vienes de EVM":** una tx puede empacar varias operaciones atómicas sin "multicall".

## Demo en vivo
- Construir y enviar un pago con la CLI; abrir el hash en Stellar Expert.
- Mostrar el XDR de la transacción en Stellar Lab.

## Lab calificado 3 — "Pago simple y lectura de ledger"
Basado en [../../exercises/01-pago-simple.md](../../exercises/01-pago-simple.md).
**Entregable:**
1. Crear 2 cuentas (alice, bob) y fondearlas.
2. Enviar un pago de XLM de alice → bob.
3. Reportar: hash, fee pagado, secuencia antes/después, ledger en que cerró.
4. Repetir consultando el resultado vía Horizon **y** describir cómo lo harías con RPC.

## Quiz 1 (cierre de Módulo 1)
Temas: Stellar vs EVM, consenso SCP, ledger/tx/operaciones, APIs.
Banco de preguntas: [../evaluacion/quizzes.md](../evaluacion/quizzes.md#quiz-1).

## Tarea
- Diseñar (en papel) una transacción con 3 operaciones que tenga sentido de negocio y justificar la
  atomicidad.

## Recursos
- https://developers.stellar.org/docs/learn/fundamentals/transactions
