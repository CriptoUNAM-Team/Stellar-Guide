# Glosario del curso

Términos clave en orden alfabético. Pensado para devs que vienen de otra cadena.

- **AMM / Liquidity pool:** pools de liquidez nativos de Stellar para swaps de producto constante.
- **Ancla (anchor):** entidad que conecta dinero del mundo real (fiat) con activos en Stellar (on/off-ramp).
- **Asset (activo):** par `(código, emisor)`, p. ej. `USDC:GA5Z...`. XLM es el activo nativo.
- **Ballot protocol:** fase de balotaje de SCP (prepare → commit → externalize) que da finalidad.
- **Claimable balance:** fondos reservados que otra cuenta puede reclamar bajo condiciones.
- **Cuenta (`G...`):** entidad nativa con balances, trustlines, signers y datos. Debe existir y tener reserva.
- **Contrato (`C...`):** dirección de un contrato Soroban.
- **FBA (Federated Byzantine Agreement):** modelo de consenso de membresía abierta basado en confianza declarada.
- **Fee base:** costo mínimo por operación (100 stroops). Soroban añade costo por recursos.
- **Finalidad determinista:** una vez externalizado el ledger, es definitivo (sin reorgs).
- **Friendbot:** servicio que fondea cuentas en Testnet gratis.
- **Horizon:** API REST clásica para cuentas, pagos, trustlines, ofertas e historial.
- **Ledger:** estado mundial que se cierra cada ~5 s (equivalente al "bloque").
- **Multisig nativa:** firmas múltiples vía signers + thresholds en la cuenta (sin contrato externo).
- **Operación:** acción atómica (payment, manage offer, change trust, invoke host function, etc.).
- **Path payment:** envío de activo X que el receptor recibe como activo Y por una ruta de conversión.
- **Quorum:** conjunto de nodos suficiente para acordar; contiene una slice de cada miembro.
- **Quorum slice:** conjunto que un nodo concreto considera suficiente para convencerse de un valor.
- **Quorum intersection:** propiedad de que dos quórums comparten un nodo honesto → safety.
- **Rent / TTL:** el estado de contrato expira si no se extiende su time-to-live; hay que pagar por renovarlo.
- **Reserva mínima:** XLM bloqueado por cuenta/entrada de estado (anti-spam).
- **SAC (Stellar Asset Contract):** expone un activo clásico como contrato token con interfaz estándar.
- **SCP (Stellar Consensus Protocol):** instancia concreta de FBA usada por Stellar.
- **SDEX:** exchange de order book integrado al protocolo.
- **SEP (Stellar Ecosystem Proposal):** estándar de interoperabilidad (p. ej. SEP-10 web auth, SEP-12 KYC).
- **Soroban:** plataforma de contratos inteligentes de Stellar (Rust → WASM).
- **Soroban RPC:** API JSON-RPC para simular/enviar invocaciones de contratos y leer su estado.
- **Storage (instance/persistent/temporary):** tipos de almacenamiento de contrato con TTL distinto.
- **Stroops:** unidad mínima de XLM (1 XLM = 10,000,000 stroops).
- **Surge pricing:** subida de fees cuando un ledger se satura.
- **Trustline:** declaración explícita de una cuenta de confiar en un emisor para recibir su activo.
- **v-blocking set:** conjunto que interseca todas las slices de un nodo; propaga la verdad en SCP.
- **XDR:** formato binario de serialización de transacciones y resultados.
- **XLM (Lumen):** activo nativo de Stellar.
