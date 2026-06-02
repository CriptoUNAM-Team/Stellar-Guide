# Semana 10 · SEPs y anclas (anchors): web auth, KYC, depósitos/retiros

**Módulo 4 · SEPs e integraciones** · Teoría + lab de SEP-10.

## Objetivos de aprendizaje
- Explicar qué es un SEP y qué es una ancla, y cómo se conectan.
- Mapear los SEPs clave a sus funciones (auth, KYC, on/off-ramp, cotización).
- Implementar el flujo de autenticación SEP-10.

## Lecturas previas
- [../../docs/sep-estandares-anclas.md](../../docs/sep-estandares-anclas.md)

## Guion de teoría (≈55 min)
1. Qué es un SEP y por qué existe la interoperabilidad estandarizada. (10 min)
2. Mapa de SEPs clave: (20 min)
   - **SEP-1** (stellar.toml, descubrimiento)
   - **SEP-10** (web authentication: challenge/response firmado)
   - **SEP-12** (KYC)
   - **SEP-24** (depósito/retiro interactivo) y **SEP-6** (programático)
   - **SEP-31** (pagos transfronterizos entre anclas)
   - **SEP-38** (cotizaciones / quotes)
3. Qué es una ancla y su rol on/off-ramp fiat ↔ Stellar. (10 min)
4. Flujo SEP-10 en detalle: challenge transaction → firma → token JWT. (15 min)

> **Callout "Vienes de EVM":** no hay equivalente directo; los SEPs son **estándares de interoperabilidad
> off-chain** (HTTP + firmas Stellar) que conectan wallets, anclas y servicios regulados.

## Demo en vivo
- Mostrar un `stellar.toml` real (SEP-1) y descubrir endpoints de un ancla de Testnet.
- Recorrer un challenge SEP-10: construir, firmar y validar.

## Lab calificado 10 — "Autenticación SEP-10"
**Entregable:**
1. Implementar (cliente) el flujo SEP-10 contra un ancla de Testnet o un servidor de ejemplo.
2. Obtener el challenge, firmarlo con tu keypair y conseguir el token de sesión.
3. Documentar cada paso con el payload real.
4. Explicar qué SEP usarías para: hacer KYC, depositar fiat, cotizar un cambio.

## Tarea
- Diseñar el diagrama de secuencia de un on-ramp completo (wallet → SEP-10 → SEP-12 → SEP-24).

## Recursos
- https://developers.stellar.org/docs/learn/fundamentals/stellar-ecosystem-proposals
- https://github.com/stellar/stellar-protocol/tree/master/ecosystem
