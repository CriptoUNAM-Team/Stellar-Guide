# Integraciones Externas (Starter Kit)

Implementación base autocontenida para:

- Soroswap
- Etherfuse
- Defindex
- Pollar
- ZKProof

## Setup

```bash
cd examples/integrations
cp .env.example .env
npm install
```

## Pruebas y demos

```bash
npm test
INTEGRATIONS_USE_MOCK=true npm run smoke:all
```

También puedes ejecutar cada integración por separado con:

- `npm run smoke:soroswap`
- `npm run smoke:etherfuse`
- `npm run smoke:defindex`
- `npm run smoke:pollar`
- `npm run smoke:zkproof`

## Contrato común

Revisa `common/adapter-shape.md` para mantener la misma forma de respuesta en todos los providers.
