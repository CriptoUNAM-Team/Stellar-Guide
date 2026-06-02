# Banco de Quizzes (con clave de respuestas para el instructor)

> Uno por módulo. Las respuestas correctas están marcadas con ✅. Oculta esta clave a los estudiantes
> (comparte solo las preguntas, p. ej. exportando sin las marcas).

---

## Quiz 1 — Fundamentos y consenso (cierre Módulo 1, semana 3)

**1. ¿Cuál es la finalidad de las transacciones en Stellar?**
- a) Probabilística, mejora con confirmaciones
- b) ✅ Determinista, ~5 s, sin reorganizaciones
- c) Depende del stake del validador
- d) Solo final tras 12 bloques

**2. Un *quorum slice* es…**
- a) El conjunto global de todos los validadores
- b) ✅ El conjunto que un nodo concreto considera suficiente para convencerse
- c) Una recompensa de bloque
- d) Un tipo de operación

**3. ¿Qué garantiza la *intersección de quórums*?**
- a) Liveness
- b) Menores fees
- c) ✅ Safety (evita forks entre nodos honestos)
- d) Mayor throughput

**4. SCP, ante una mala configuración de confianza, tiende a…**
- a) Forkearse
- b) ✅ Bloquearse (priorizar safety sobre liveness)
- c) Aumentar recompensas
- d) Reducir el fee base

**5. En Stellar, emitir un activo significa…**
- a) Desplegar un contrato ERC-20
- b) ✅ Definir un activo `(código, emisor)` y que los receptores abran trustlines
- c) Minar nuevos XLM
- d) Crear un liquidity pool obligatorio

**6. Una transacción Stellar puede contener…**
- a) Una sola operación
- b) ✅ De 1 a 100 operaciones, de forma atómica
- c) Operaciones de distintas redes
- d) Solo pagos

**7. ¿Para qué sirve el número de secuencia?**
- a) Calcular el fee
- b) ✅ Ordenar tx de una cuenta y prevenir replays
- c) Definir el quórum
- d) Medir el TTL

**8. ¿Cuándo preferirías Soroban RPC sobre Horizon?**
- a) Para historial de pagos clásicos
- b) ✅ Para simular/enviar invocaciones de contratos y leer estado de contrato
- c) Nunca, Horizon hace todo
- d) Solo en mainnet

**9. La reserva mínima de una cuenta…**
- a) Se gasta en fees
- b) ✅ Bloquea XLM como anti-spam de estado; sube con cada entrada (trustline, signer, etc.)
- c) Es opcional
- d) Solo aplica a contratos

**10 (abierta).** Explica en 3-4 líneas por qué la seguridad de Stellar "se configura" en lugar de
"comprarse" como en PoW/PoS.
*Clave:* mencionar confianza declarada (quorum slices), intersección de quórums, ausencia de minería/stake,
riesgo topológico vs económico.

---

## Quiz 2 — Red clásica y activos (cierre Módulo 2, semana 5)

**1. Para recibir un activo no nativo, una cuenta debe…**
- a) Pagar gas extra
- b) ✅ Abrir una trustline hacia el emisor
- c) Desplegar un contrato
- d) Nada, es automático

**2. El SDEX es…**
- a) Un contrato de terceros
- b) ✅ Un exchange de order book integrado al protocolo
- c) Un SEP
- d) Una wallet

**3. Un *path payment* permite…**
- a) Pagar en varias redes
- b) ✅ Enviar activo X y que el receptor reciba activo Y por una ruta de conversión
- c) Evitar fees
- d) Crear cuentas

**4. La multifirma en Stellar es…**
- a) Un contrato tipo Gnosis Safe
- b) ✅ Nativa: signers + thresholds en la propia cuenta
- c) Imposible
- d) Solo en Soroban

**5. Los *thresholds* (low/medium/high) controlan…**
- a) El fee
- b) ✅ Cuánto peso de firmas se requiere según el tipo de operación
- c) El TTL
- d) El quórum de la red

**6. Un *claimable balance* sirve para…**
- a) Minar XLM
- b) ✅ Reservar fondos que otra cuenta puede reclamar bajo condiciones
- c) Pagar fees
- d) Crear trustlines

**7. ¿Qué flag de emisor usarías para un activo regulado que debe poder congelarse?**
- a) Ninguno
- b) ✅ auth_required / clawback / freeze
- c) native
- d) surge

**8. *Sponsored reserves* permiten…**
- a) Quitar fees
- b) ✅ Que otra cuenta pague la reserva mínima de otra (onboarding sin XLM)
- c) Doble gasto
- d) Saltarse trustlines

**9. XLM (nativo) requiere trustline para recibirse.**
- a) Verdadero
- b) ✅ Falso

**10 (abierta).** Diseña una config de tesorería 2-de-3 y explica qué thresholds usarías.
*Clave:* 3 signers con peso, umbral medium = suma de 2 pesos; master weight ajustado.

---

## Quiz 3 — Soroban (cierre Módulo 3, semana 9)

**1. Soroban ejecuta…**
- a) Bytecode EVM
- b) ✅ WASM compilado desde Rust
- c) JavaScript
- d) Python

**2. ¿Cuál NO es un tipo de storage de Soroban?**
- a) instance
- b) persistent
- c) temporary
- d) ✅ eternal

**3. El TTL/rent significa que…**
- a) El estado vive para siempre
- b) ✅ El estado expira/archiva si no se extiende su TTL
- c) Se paga solo una vez
- d) Solo aplica a eventos

**4. La autorización en Soroban se hace con…**
- a) `require(msg.sender == x)`
- b) ✅ `address.require_auth()`
- c) Un signer global
- d) Nada, es pública

**5. Los fees de Soroban se miden por…**
- a) Un gas fijo
- b) ✅ Recursos: instrucciones CPU, ledger I/O, tamaño de estado, rent
- c) Número de cuentas
- d) Tamaño del bloque

**6. El SAC (Stellar Asset Contract) sirve para…**
- a) Crear cuentas
- b) ✅ Exponer un activo clásico como contrato token con interfaz estándar
- c) Validar consenso
- d) Firmar tx

**7. Una llamada cross-contract requiere cuidar…**
- a) El fee base solamente
- b) ✅ Propagación de auth, reentrada y TTL del estado
- c) Nada especial
- d) El passphrase

**8. El patrón de upgrade de Soroban usa…**
- a) Redeploy con nuevo ID siempre
- b) ✅ `update_current_contract_wasm` protegido por auth
- c) No existe
- d) Un proxy obligatorio

**9. ¿Qué storage usarías para un dato de sesión efímero?**
- a) instance
- b) persistent
- c) ✅ temporary
- d) ninguno

**10 (abierta).** Nombra dos vulnerabilidades propias de Soroban (más allá de overflow/auth genéricos).
*Clave:* estado expirado no manejado, auth mal propagada en cross-contract, suposición de persistencia.

---

## Quiz 4 — SEPs e integraciones (cierre Módulo 4, semana 11)

**1. Un SEP es…**
- a) Un contrato Soroban
- b) ✅ Un estándar de interoperabilidad del ecosistema Stellar
- c) Un tipo de cuenta
- d) Una wallet

**2. SEP-10 corresponde a…**
- a) KYC
- b) ✅ Autenticación web (challenge/response firmado → token)
- c) Cotizaciones
- d) stellar.toml

**3. SEP-1 define…**
- a) Pagos transfronterizos
- b) ✅ El archivo `stellar.toml` para descubrimiento de endpoints/info
- c) Multisig
- d) AMM

**4. Una *ancla* (anchor) es…**
- a) Un validador
- b) ✅ Entidad que conecta dinero del mundo real (fiat) con activos en Stellar
- c) Un contrato
- d) Un signer

**5. ¿Qué SEP usarías para KYC?**
- a) SEP-10
- b) ✅ SEP-12
- c) SEP-31
- d) SEP-38

**6. SEP-24 vs SEP-6:**
- a) Son lo mismo
- b) ✅ SEP-24 es depósito/retiro interactivo; SEP-6 es programático
- c) SEP-24 es KYC
- d) SEP-6 es web auth

**7. SEP-38 sirve para…**
- a) Multisig
- b) ✅ Cotizaciones (quotes) de cambio
- c) Crear cuentas
- d) Eventos

**8. En el frontend, Freighter cumple el rol de…**
- a) Validador
- b) ✅ Wallet que firma transacciones (similar a MetaMask)
- c) RPC
- d) Ancla

**9. El patrón adapter del repo sirve para…**
- a) Minar
- b) ✅ Normalizar proveedores externos (mock/real) tras una interfaz común
- c) Cerrar ledgers
- d) Firmar SEP-10

**10 (abierta).** Ordena los SEPs en un on-ramp típico de fiat a Stellar.
*Clave:* SEP-1 (descubrir) → SEP-10 (auth) → SEP-12 (KYC) → SEP-24/6 (depósito), opcional SEP-38 (quote).

---

## Quiz 5 — Producción (cierre Módulo 5, semana 12)

**1. Mainnet de Stellar…**
- a) Se resetea cada mes
- b) ✅ No se resetea y usa dinero real
- c) Es gratis con Friendbot
- d) No tiene fees

**2. Firmar una tx con la passphrase equivocada…**
- a) Funciona igual
- b) ✅ Falla por diseño (red equivocada)
- c) Duplica el pago
- d) Sube el fee

**3. Buena práctica de claves:**
- a) Guardarlas en el código
- b) ✅ Nunca en código; usar secretos/entorno y firma segura
- c) Compartirlas por chat
- d) Subirlas al repo

**4. Idempotencia importa porque…**
- a) Baja el fee
- b) ✅ Evita efectos duplicados al reintentar operaciones
- c) Acelera el consenso
- d) No importa

**5. Antes de mainnet debes…**
- a) Saltarte los tests
- b) ✅ Completar checklist de hardening, tests verdes y revisión de auth
- c) Desactivar logs
- d) Borrar el .env.example

**6 (abierta).** Lista 3 ítems críticos de tu checklist pre-mainnet y por qué.
*Clave:* sin secretos en código; auth validada en funciones sensibles; idempotencia; manejo de errores;
TTL del estado; monitoreo.
