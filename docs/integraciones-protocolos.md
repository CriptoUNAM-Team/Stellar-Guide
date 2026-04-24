# Integraciones de Protocolos (Defindex, Soroswap, Etherfuse, Pollar, ZKProof)

Guía práctica para developers nuevos usando implementaciones autocontenidas en `examples/integrations`.

## Quickstart

```bash
cd examples/integrations
cp .env.example .env
npm install
npm test
INTEGRATIONS_USE_MOCK=true npm run smoke:all
```

## Matriz de capacidades

| Integración | Healthcheck | Quote | Execute | Notas |
|---|---|---|---|---|
| Soroswap | Sí | Sí | Sí | Swap XLM/USDC y pares compatibles |
| Etherfuse | Sí | Sí (on/off ramp) | Parcial | Incluye lookup de stablebonds |
| Defindex | Sí | N/A | Sí (deposit/withdraw) | Incluye APY y balance |
| Pollar | Sí | Sí (ramp quote) | Parcial | Enfoque session + ramp |
| ZKProof | N/A | N/A | Sí (attestation flow) | Generación/verificación de prueba |

## Flujo sugerido por fases

1. Ejecuta todo en mock para entender contratos de datos.
2. Activa Soroswap y Etherfuse con endpoints reales.
3. Activa Defindex y Pollar con llaves reales.
4. Conecta ZKProof backend para attestation on-chain.

## Troubleshooting rápido

- `401/403`: valida API keys en `.env`.
- `timeout`: incrementa `INTEGRATIONS_TIMEOUT_MS`.
- `404 en provider`: revisa base URL y ruta de endpoint.
- `proof inválido`: valida formato `0x` + 64 hex.
