# Yield Vault Contract (Soroban)

Bóveda simple de rendimiento por shares.

## Funciones

- `initialize(admin, token)`
- `deposit(user, amount) -> shares`
- `harvest(yield_amount)` (incrementa activos totales)
- `withdraw(user, shares) -> amount`
- `get_share_balance(user)`
- `get_totals()`

## Uso

```bash
cd contracts/yield
make test
```

Deploy e invoke: [docs/comandos-basicos.md](../../docs/comandos-basicos.md).
