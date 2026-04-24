# ZKProof Integration

Implementación base inspirada en tu servicio anterior: generar prueba, validar formato local y simular/ejecutar atestación on-chain.

## Uso rápido

```bash
cd examples/integrations
cp .env.example .env
INTEGRATIONS_USE_MOCK=true npm run smoke:zkproof
```

## Operaciones

- `generateProof({savedAmount,targetAmount,userId})`
- `verifyLocal({proof})`
- `verifyOnChainAttestation({proof,publicInputs})`

Para integración real, configura `ZKPROOF_BACKEND_URL` y `ZKPROOF_ATTESTATION_CONTRACT_ID`.
