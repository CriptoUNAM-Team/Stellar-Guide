# Checklist Pre-Mainnet

## Seguridad

- No hay secretos/API keys en código fuente.
- `.env.example` actualizado para todas las integraciones.
- Validaciones de auth en todas las funciones sensibles de contratos.
- Idempotencia para operaciones críticas (ej. nómina por periodo).
- Manejo de errores explícito y tipado.

## Contratos Soroban

- Tests unitarios verdes en:
  - `contracts/payroll`
  - `contracts/savings`
  - `contracts/loan`
  - `contracts/yield`
  - `contracts/nft-membership`
- Revisión de límites de montos y estados inválidos.
- Eventos de negocio definidos para observabilidad.

## Integraciones Externas

- Smoke tests verdes en `examples/integrations/tests`.
- Timeouts y retries configurados.
- Fallback mock habilitable para demos/control de riesgo.
- Matriz de capacidades y limitaciones documentada.

## Operación

- Runbook de incidentes documentado.
- Monitoreo básico de errores por proveedor.
- Flujos de reversa/cancelación definidos cuando aplique.
