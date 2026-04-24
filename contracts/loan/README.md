# Loan Contract (Soroban)

Contrato base de préstamos colateralizados para talleres.

## Funciones

- `initialize(admin, token, min_collateral_bps)`
- `create_position(borrower, collateral_amount)`
- `borrow(position_id, debt_amount)`
- `repay(position_id, amount)`
- `close_position(position_id)`
- `get_position(position_id)`

## Uso

```bash
cd contracts/loan
make test
```
