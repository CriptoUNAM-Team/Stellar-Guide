# Troubleshooting de Integraciones

## Errores comunes

### 401 / 403
- Verifica API key y formato de header `Authorization: Bearer`.
- Confirma que la key corresponde al entorno correcto (test/prod).

### 404 en endpoint
- Revisa `*_API_BASE_URL` en `.env`.
- Confirma rutas actuales del proveedor (pueden cambiar por versión).

### Timeouts intermitentes
- Incrementa `INTEGRATIONS_TIMEOUT_MS`.
- Implementa reintento en capa superior para operaciones no idempotentes con cuidado.

### Fallas de ZK proof
- Valida formato (`0x` + 64 hex mínimo en esta base).
- Verifica consistencia de inputs públicos y backend de verificación.

### Tests de adapters fallan local
- Ejecuta en modo mock:
  - `INTEGRATIONS_USE_MOCK=true npm test`
- Si usas modo real, define todos los env vars obligatorios.
