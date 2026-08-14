# amm — Pool de producto constante (DeFi)

Swap A→B con `x * y = k`. Sin fee, para el lab. Liquidez la aporta un `provider` con `add_liquidity`.

Complementa `loan` (colateral + liquidación) y `yield` (shares). Aquí el tema es precio implícito: más A entra, menos B sale.

## Funciones

- `initialize(admin, token_a, token_b)`
- `add_liquidity(provider, amount_a, amount_b)`
- `swap_a_for_b(trader, amount_in) -> amount_out`
- `get_reserves()`

## Lab (sesión 2)

```bash
cd contracts/amm
make test
```

Deploy e invoke: [docs/comandos-basicos.md](../../docs/comandos-basicos.md).
