# Guía 0 → Builder

## 0. Fundamentos
- Cuentas, claves y Horizon
- Testnet vs Mainnet
- Instalación y herramientas

## 1. Primeras transacciones
- Crear cuenta y fondear con Friendbot
- Enviar pago simple XLM
- Consultar estado en Horizon

## 2. Primer contrato Soroban
- Plantilla counter
- Compilar y desplegar en Testnet
- Invocar funciones y leer estado

## 2.1 Contratos orientados a negocio
- Préstamos colateralizados (loan vault)
- Dispersión de nómina (pagos por lote)
- Ahorro programado por metas
- Bóveda de rendimiento (yield por shares)
- NFTs de membresía/certificados
- Referencia: [Contratos Soroban por Casos de Uso](contratos-casos-uso.md)
- Implementaciones iniciales: `contracts/payroll` y `contracts/savings`

## 3. Patrones y seguridad
- Gestión de claves y permisos
- Límites de recursos y costos
- Eventos y observabilidad
- Controles de auth y estados válidos por caso de uso

## 4. Integración cliente
- SDK JS: construir y firmar transacciones
- End-to-end: cliente + contrato
- Integraciones por proveedor: Defindex, Soroswap, Etherfuse, Pollar, ZKProof
- Referencia: [Integraciones de Protocolos](integraciones-protocolos.md)
- Frontend para invocar contratos: [Frontend para probar contratos Soroban](frontend-contratos.md)

## 5. Publicación
- Checklist de revisión
- Documentación y diagramas Mermaid
- Empaquetado del taller y handouts
