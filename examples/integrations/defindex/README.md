# Defindex Integration

Adaptador base para operaciones de vault/yield inspirado en tu servicio previo: APY, balance, depósito y retiro.

## Uso rápido

```bash
cd examples/integrations
cp .env.example .env
INTEGRATIONS_USE_MOCK=true npm run smoke:defindex
```

## Operaciones

- `healthcheck()`
- `getApy()`
- `getBalance({userAddress})`
- `deposit({userAddress,amount})`
- `withdraw({userAddress,amount})`
