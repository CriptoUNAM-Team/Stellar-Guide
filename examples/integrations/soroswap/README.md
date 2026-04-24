# Soroswap Integration

Adaptador base para cotizar y ejecutar swaps desde un proveedor estilo Soroswap.

## Uso rápido

```bash
cd examples/integrations
cp .env.example .env
INTEGRATIONS_USE_MOCK=true npm run smoke:soroswap
```

## Operaciones

- `healthcheck()`
- `quote({fromAsset,toAsset,amount})`
- `execute({fromAsset,toAsset,amount,userAddress})`

Este módulo está inspirado en tu implementación de AITrade y encapsula quote/execute con formato normalizado.
