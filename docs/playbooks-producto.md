# Playbooks de Producto (E2E)

Playbooks listos para taller, combinando contratos + integraciones externas.

## 1) Ahorro + Yield + Defindex

1. Crear meta en `contracts/savings`.
2. Depositar fondos y acumular balance.
3. Mover fondos a bóveda `contracts/yield`.
4. Consultar APY y balance externo con `examples/integrations/defindex`.

Resultado esperado: usuario ve crecimiento entre rendimiento interno (shares) y referencia externa de vault.

## 2) Nómina + Stable Asset + Etherfuse/Pollar

1. Registrar destinatarios en `contracts/payroll`.
2. Ejecutar dispersión por `period_id`.
3. Obtener quote de ramp con `examples/integrations/etherfuse` o `examples/integrations/pollar`.
4. Simular salida fiat para beneficiarios.

Resultado esperado: pago en lote idempotente + puente de salida a fiat modelado.

## 3) Préstamo + Score privado + ZKProof

1. Crear posición en `contracts/loan`.
2. Generar prueba con `examples/integrations/zkproof` (`savedAmount >= targetAmount`).
3. Verificar prueba local y/o attestation on-chain.
4. Permitir borrow sólo cuando flujo de scoring/prueba sea válido en backend de negocio.

Resultado esperado: originación de crédito con dato privado verificable sin exponer raw balance.
