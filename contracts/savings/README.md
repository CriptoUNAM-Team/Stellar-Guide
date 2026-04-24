# Savings Contract (Soroban)

Contrato base de ahorro por meta con desbloqueo temporal y penalización por retiro anticipado.

## Funciones principales

- `initialize(admin, token)`
- `create_goal(owner, target_amount, unlock_time, penalty_bps)`
- `deposit(goal_id, from, amount)`
- `withdraw(goal_id, to) -> (payout, penalty)`
- `get_goal(goal_id)`

`penalty_bps` usa basis points (`100` = 1%, `1000` = 10%).

## Comandos locales

```bash
cd contracts/savings
make test
```

## Build y deploy (Testnet)

```bash
cd contracts/savings
stellar contract build

stellar contract deploy \
  --wasm target/wasm32v1-none/release/savings.wasm \
  --network testnet \
  --source alice
```

## Ejemplo de uso

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  initialize --admin "$(stellar keys address alice)" --token "<TOKEN_ADDRESS>"

stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  create_goal --owner "$(stellar keys address bob)" --target_amount 5000000 --unlock_time 1893456000 --penalty_bps 500

stellar contract invoke --id <CONTRACT_ID> --source bob --network testnet -- \
  deposit --goal_id 1 --from "$(stellar keys address bob)" --amount 1000000
```
