# Payroll Contract (Soroban)

Contrato base para dispersar pagos por lote con idempotencia por periodo.

## Funciones principales

- `initialize(admin, token)`
- `add_recipient(recipient, amount)`
- `remove_recipient(recipient)`
- `get_recipient(recipient)`
- `get_all_recipients()`
- `disperse_period(period_id)`

`period_id` permite evitar doble pago para la misma corrida (ejemplo: `202604`).

## Comandos locales

```bash
cd contracts/payroll
make test
```

## Build y deploy (Testnet)

```bash
cd contracts/payroll
stellar contract build

stellar contract deploy \
  --wasm target/wasm32v1-none/release/payroll.wasm \
  --network testnet \
  --source alice
```

## Ejemplo de uso

```bash
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  initialize --admin "$(stellar keys address alice)" --token "<TOKEN_ADDRESS>"

stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  add_recipient --recipient "<G...EMP1>" --amount 1000000

stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- \
  disperse_period --period_id 202604
```
