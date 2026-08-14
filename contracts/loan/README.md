# Loan Contract (Soroban)

Préstamo colateralizado con tokens reales (SAC). El colateral entra al contrato; el préstamo sale del pool. Si el ratio cae bajo `min_collateral_bps`, un liquidator paga la deuda y se queda el colateral.

`min_collateral_bps` 15000 = 150% de colateral vs deuda (LTV máx. ~66%). En el lab, el admin puede subir el umbral con `set_min_collateral_bps` para forzar una posición liquidable (sin oráculo de precio).

## Funciones

- `initialize(admin, token, min_collateral_bps)`
- `create_position(borrower, collateral_amount)` — transfiere colateral al contrato
- `deposit_collateral(position_id, amount)`
- `borrow(position_id, debt_amount)`
- `repay(position_id, amount)`
- `liquidate(liquidator, position_id)`
- `set_min_collateral_bps(min_collateral_bps)` — solo admin
- `close_position(position_id)` — devuelve colateral si deuda = 0
- `get_position(position_id)`
- `is_liquidatable(position_id)`

## Uso

```bash
cd contracts/loan
make test
```

Deploy e invoke: [docs/comandos-basicos.md](../../docs/comandos-basicos.md).
