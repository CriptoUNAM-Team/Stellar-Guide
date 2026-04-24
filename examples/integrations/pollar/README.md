# Pollar Integration

Adaptador base alineado al uso de `@pollar/react` y configuración de red Stellar.

## Uso rápido

```bash
cd examples/integrations
cp .env.example .env
INTEGRATIONS_USE_MOCK=true npm run smoke:pollar
```

## Operaciones

- `healthcheck()`
- `createSession({userAddress})`
- `getRampQuote({fiat,amount})`
