# food-trace — Trazabilidad alimentaria en Soroban

Contrato para registrar la cadena productiva de alimentos: participantes, lotes, custodia y eventos inmutables. Pensado como base para una implementación industrial (HACCP, recalls, auditorías).

## Modelo

| Concepto | Descripción |
|----------|-------------|
| **Actor** | Participante registrado (productor, procesador, transportista, almacén, distribuidor, minorista, certificador) |
| **Lote (batch)** | Unidad trazable: producto, código de lote, cantidad, metadatos (URI a IPFS/JSON) |
| **Etapa** | Punto en la cadena: producción, procesamiento, transporte, retail, etc. |
| **Evento** | Registro inmutable en el historial del lote |

Los `metadata_uri` y `notes` apuntan a documentación off-chain (certificados, temperaturas, fotos, COA) mientras el contrato guarda la cadena de custodia on-chain.

## Compilar

```bash
stellar contract build --manifest-path contracts/food-trace/Cargo.toml
```

## Desplegar e inicializar

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/food_trace.wasm \
  --source blockchain \
  --network testnet

stellar contract invoke --id <CONTRACT_ID> --source blockchain --network testnet -- \
  initialize --admin blockchain
```

## Flujo de ejemplo (tomate)

```bash
# 1) Registrar actores
stellar contract invoke --id <ID> --source blockchain --network testnet -- \
  register_actor \
  --actor <PRODUCTOR_G...> \
  --role Producer \
  --name '"Finca El Sol"' \
  --metadata_uri '"ipfs://certs/finca"'

# 2) Crear lote en origen
stellar contract invoke --id <ID> --source <PRODUCTOR> --network testnet -- \
  create_batch \
  --producer <PRODUCTOR> \
  --product_name '"Tomate cherry"' \
  --batch_code '"LOT-2026-001"' \
  --quantity 500 \
  --unit '"kg"' \
  --metadata_uri '"ipfs://batches/lot-001"'

# 3) Transferir custodia al transportista
stellar contract invoke --id <ID> --source <PRODUCTOR> --network testnet -- \
  transfer_custody \
  --from <PRODUCTOR> \
  --to <TRANSPORTISTA> \
  --batch_id 1 \
  --stage Transport \
  --metadata_uri '"ipfs://events/salida"' \
  --notes '"En camino a planta"'

# 4) Consultar historial
stellar contract invoke --id <ID> --source blockchain --network testnet -- \
  get_trace_history --batch_id 1
```

## Funciones principales

- `initialize` — autoridad administradora
- `register_actor` — alta de participantes (solo admin)
- `create_batch` — origen del lote (solo productor)
- `update_stage` — cambio de etapa sin transferir custodia
- `transfer_custody` — entrega del lote al siguiente eslabón
- `record_quality_check` — inspección (solo certificador)
- `recall_batch` — retiro de mercado (admin o certificador)
- `get_batch`, `get_actor`, `get_trace_history` — consultas

## Próximos pasos para producción

- Integrar lectores QR / NFC con el `batch_id` y `batch_code`
- Subir metadatos a IPFS o storage empresarial y guardar solo el hash/URI on-chain
- Panel web que lea `get_trace_history` para consumidor final
- Roles adicionales por normativa local (SENASICA, FDA, EU TRACES)
