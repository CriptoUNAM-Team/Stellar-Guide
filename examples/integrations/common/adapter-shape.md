# Contrato común de adapters

Cada integración implementa una clase con métodos equivalentes:

- `getCapabilities()`: metadatos y operaciones soportadas.
- `healthcheck()`: validación de conexión al proveedor.
- `quote(params)`: cotización normalizada.
- `execute(params)`: ejecución normalizada (si aplica).

## Respuesta normalizada

```json
{
  "ok": true,
  "provider": "soroswap",
  "operation": "quote",
  "requestId": "uuid/opcional",
  "data": {},
  "error": null
}
```

## Reglas

- Nunca guardar secretos en código.
- Respetar timeout global y fallback mock opcional.
- Propagar `txHash` o `operationId` en `data` cuando exista.
- Devolver `ok=false` con `error` legible para onboarding.
