# Etherfuse Integration

Adaptador base alineado al flujo de `seyf-app`: lookup de stablebonds y cotizaciones onramp/offramp.

## Uso rápido

```bash
cd examples/integrations
cp .env.example .env
INTEGRATIONS_USE_MOCK=true npm run smoke:etherfuse
```

## Operaciones

- `healthcheck()`
- `lookupStablebonds()`
- `quoteOnramp({fiatAmount,fiat})`
- `quoteOfframp({tokenAmount,fiat})`
