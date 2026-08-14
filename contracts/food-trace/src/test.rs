#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (
    Env,
    Address,
    Address,
    Address,
    Address,
    Address,
    FoodTraceContractClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let producer = Address::generate(&env);
    let processor = Address::generate(&env);
    let transporter = Address::generate(&env);
    let certifier = Address::generate(&env);

    let contract_id = env.register(FoodTraceContract, ());
    let client = FoodTraceContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.register_actor(
        &producer,
        &ActorRole::Producer,
        &String::from_str(&env, "Finca El Sol"),
        &String::from_str(&env, "ipfs://certs/finca-el-sol"),
    );
    client.register_actor(
        &processor,
        &ActorRole::Processor,
        &String::from_str(&env, "Procesadora Norte"),
        &String::from_str(&env, "ipfs://certs/procesadora-norte"),
    );
    client.register_actor(
        &transporter,
        &ActorRole::Transporter,
        &String::from_str(&env, "LogiFrio SA"),
        &String::from_str(&env, "ipfs://certs/logifrio"),
    );
    client.register_actor(
        &certifier,
        &ActorRole::Certifier,
        &String::from_str(&env, "CertAlimentos MX"),
        &String::from_str(&env, "ipfs://certs/certalimentos"),
    );

    (
        env,
        admin,
        producer,
        processor,
        transporter,
        certifier,
        client,
    )
}

#[test]
fn test_full_supply_chain_trace() {
    let (env, _, producer, processor, transporter, certifier, client) = setup();

    let batch_id = client.create_batch(
        &producer,
        &String::from_str(&env, "Tomate cherry"),
        &String::from_str(&env, "LOT-2026-001"),
        &500,
        &String::from_str(&env, "kg"),
        &String::from_str(&env, "ipfs://batches/lot-2026-001"),
    );
    assert_eq!(batch_id, 1);

    client.update_stage(
        &producer,
        &batch_id,
        &SupplyStage::Packaging,
        &String::from_str(&env, "ipfs://events/empaque-001"),
        &String::from_str(&env, "Empaque en cajas de 10 kg"),
    );

    client.transfer_custody(
        &producer,
        &transporter,
        &batch_id,
        &SupplyStage::Transport,
        &String::from_str(&env, "ipfs://events/transporte-001"),
        &String::from_str(&env, "Salida hacia procesadora"),
    );

    client.transfer_custody(
        &transporter,
        &processor,
        &batch_id,
        &SupplyStage::Processing,
        &String::from_str(&env, "ipfs://events/recepcion-001"),
        &String::from_str(&env, "Recepcion en planta"),
    );

    client.record_quality_check(
        &certifier,
        &batch_id,
        &SupplyStage::Processing,
        &String::from_str(&env, "ipfs://qc/reporte-001"),
        &String::from_str(&env, "Aprobado HACCP"),
    );

    let batch = client.get_batch(&batch_id);
    assert_eq!(batch.current_holder, processor);
    assert_eq!(batch.current_stage, SupplyStage::Processing);
    assert_eq!(batch.status, BatchStatus::Active);

    let history = client.get_trace_history(&batch_id);
    assert_eq!(history.len(), 5);
    assert_eq!(history.get(0).unwrap().event_type, EventType::BatchCreated);
    assert_eq!(history.get(4).unwrap().event_type, EventType::QualityCheck);
}

#[test]
fn test_recall_batch() {
    let (env, admin, producer, _, _, certifier, client) = setup();

    let batch_id = client.create_batch(
        &producer,
        &String::from_str(&env, "Lechuga"),
        &String::from_str(&env, "LOT-2026-002"),
        &100,
        &String::from_str(&env, "kg"),
        &String::from_str(&env, "ipfs://batches/lot-2026-002"),
    );

    client.recall_batch(
        &certifier,
        &batch_id,
        &String::from_str(&env, "ipfs://recall/motivo-002"),
        &String::from_str(&env, "Deteccion de contaminante"),
    );

    let batch = client.get_batch(&batch_id);
    assert_eq!(batch.status, BatchStatus::Recalled);

    let result = client.try_update_stage(
        &producer,
        &batch_id,
        &SupplyStage::Storage,
        &String::from_str(&env, "ipfs://events/x"),
        &String::from_str(&env, "no deberia permitirse"),
    );
    assert_eq!(result, Err(Ok(TraceError::BatchNotActive)));

    let _ = admin;
}
