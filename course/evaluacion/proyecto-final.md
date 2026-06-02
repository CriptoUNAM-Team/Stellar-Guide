# Proyecto Final — Producto Stellar end-to-end

Vale **35%** de la calificación. Se desarrolla desde la semana 8 y se defiende en la semana 12.

## Objetivo

Construir un producto funcional sobre Stellar que combine **al menos**:
1. **Uno o más contratos Soroban** propios o extendidos (no solo los del repo sin cambios).
2. **Una integración real**: un SEP (p. ej. SEP-10) **o** un adapter de protocolo **o** un frontend con wallet.
3. **Despliegue en Testnet** con demo reproducible.

## Ideas válidas (elige o propón)

- **Pagos/remesas:** on-ramp simulado + path payment + contrato de reglas de envío.
- **Ahorro/DeFi:** metas de ahorro (`savings`) + bóveda de rendimiento (`yield`) + consulta de APY externa.
- **Nómina/tesorería:** dispersión (`payroll`) con cuenta multisig 2-de-3 y reportes.
- **Membresías/credenciales:** NFT de membresía (`nft-membership`) + control de acceso en frontend.
- **Crédito:** préstamo colateralizado (`loan`) con liquidación y oráculo simulado.
- **Tu propuesta:** debe aprobarse en el checkpoint de la semana 9.

## Entregables

1. **Repo** con:
   - `README.md` (qué hace, cómo correrlo, arquitectura con diagrama Mermaid).
   - Contratos + tests (`cargo test` verde).
   - Código de integración/frontend.
   - Instrucciones de despliegue y **contract IDs / hashes** de Testnet.
   - Checklist de hardening completado ([../../docs/checklist-pre-mainnet.md](../../docs/checklist-pre-mainnet.md)).
2. **Demo** funcional (CLI o UI) reproducible por el instructor.
3. **Slides** de la defensa (8-10 min).

## Hitos (checkpoints)

| Semana | Hito |
|---|---|
| 8 | Idea inicial + contratos a usar. |
| 9 | **Checkpoint formal:** propuesta aprobada, arquitectura, plan de 3 semanas. |
| 10-11 | Desarrollo + integración. |
| 12 | **Defensa** y entrega final. |

## Criterios de evaluación

Se usa la rúbrica del proyecto en [rubricas.md](rubricas.md#rúbrica-del-proyecto-final-100-pts--35-del-curso) (100 pts).

## Reglas

- Equipos de 1-3 personas (el alcance escala con el tamaño del equipo).
- Puedes partir de los contratos del repo, pero **debes aportar cambios sustanciales** y entenderlos.
- Uso de IA permitido, pero en la defensa debes poder **explicar cualquier línea** de tu proyecto.
- Todo debe correr en **Testnet** (mainnet opcional y bajo tu propio riesgo).
