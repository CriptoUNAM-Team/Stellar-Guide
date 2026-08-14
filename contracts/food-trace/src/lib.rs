#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String, Vec};

/// Rol del participante en la cadena productiva alimentaria.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ActorRole {
    Producer = 1,
    Processor = 2,
    Transporter = 3,
    Warehouse = 4,
    Distributor = 5,
    Retailer = 6,
    Certifier = 7,
}

/// Etapa de la cadena de suministro.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SupplyStage {
    RawMaterial = 1,
    Production = 2,
    Processing = 3,
    Packaging = 4,
    Storage = 5,
    Transport = 6,
    Distribution = 7,
    Retail = 8,
}

/// Tipo de evento registrado en la trazabilidad.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EventType {
    BatchCreated = 1,
    StageUpdate = 2,
    CustodyTransfer = 3,
    QualityCheck = 4,
    Recall = 5,
}

/// Estado operativo del lote.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum BatchStatus {
    Active = 1,
    Recalled = 2,
    Consumed = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub address: Address,
    pub role: ActorRole,
    pub name: String,
    pub metadata_uri: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductBatch {
    pub batch_id: u64,
    pub product_name: String,
    pub batch_code: String,
    pub quantity: u64,
    pub unit: String,
    pub metadata_uri: String,
    pub created_at: u64,
    pub current_holder: Address,
    pub current_stage: SupplyStage,
    pub status: BatchStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceEvent {
    pub event_id: u64,
    pub batch_id: u64,
    pub actor: Address,
    pub stage: SupplyStage,
    pub event_type: EventType,
    pub timestamp: u64,
    pub metadata_uri: String,
    pub notes: String,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextBatchId,
    NextEventId,
    Actor(Address),
    Batch(u64),
    BatchEvents(u64),
    Event(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TraceError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    ActorNotRegistered = 3,
    ActorAlreadyRegistered = 4,
    BatchNotFound = 5,
    EventNotFound = 6,
    NotCurrentHolder = 7,
    BatchNotActive = 8,
    InvalidQuantity = 9,
    UnauthorizedRole = 10,
}

#[contract]
pub struct FoodTraceContract;

#[contractimpl]
impl FoodTraceContract {
    /// Inicializa el registro de trazabilidad con una autoridad administradora.
    pub fn initialize(env: Env, admin: Address) -> Result<(), TraceError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(TraceError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextBatchId, &1u64);
        env.storage().persistent().set(&DataKey::NextEventId, &1u64);
        Ok(())
    }

    /// Registra un participante de la cadena (productor, transportista, etc.).
    pub fn register_actor(
        env: Env,
        actor: Address,
        role: ActorRole,
        name: String,
        metadata_uri: String,
    ) -> Result<(), TraceError> {
        let admin = Self::admin(&env)?;
        admin.require_auth();

        if env
            .storage()
            .persistent()
            .has(&DataKey::Actor(actor.clone()))
        {
            return Err(TraceError::ActorAlreadyRegistered);
        }

        env.storage().persistent().set(
            &DataKey::Actor(actor.clone()),
            &Actor {
                address: actor,
                role,
                name,
                metadata_uri,
            },
        );
        Ok(())
    }

    /// Crea un lote en origen. Solo un productor registrado puede hacerlo.
    pub fn create_batch(
        env: Env,
        producer: Address,
        product_name: String,
        batch_code: String,
        quantity: u64,
        unit: String,
        metadata_uri: String,
    ) -> Result<u64, TraceError> {
        producer.require_auth();
        let actor = Self::get_actor(env.clone(), producer.clone())?;
        if actor.role != ActorRole::Producer {
            return Err(TraceError::UnauthorizedRole);
        }
        if quantity == 0 {
            return Err(TraceError::InvalidQuantity);
        }

        let batch_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextBatchId)
            .ok_or(TraceError::NotInitialized)?;

        let batch = ProductBatch {
            batch_id,
            product_name,
            batch_code,
            quantity,
            unit,
            metadata_uri: metadata_uri.clone(),
            created_at: env.ledger().timestamp(),
            current_holder: producer.clone(),
            current_stage: SupplyStage::Production,
            status: BatchStatus::Active,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Batch(batch_id), &batch);
        env.storage()
            .persistent()
            .set(&DataKey::NextBatchId, &(batch_id + 1));
        env.storage()
            .persistent()
            .set(&DataKey::BatchEvents(batch_id), &Vec::<u64>::new(&env));

        Self::append_event(
            &env,
            batch_id,
            producer,
            SupplyStage::Production,
            EventType::BatchCreated,
            metadata_uri,
            String::from_str(&env, "Lote creado en origen"),
        )?;

        Ok(batch_id)
    }

    /// Actualiza la etapa del lote sin cambiar de custodio (p. ej. empaque, almacenaje).
    pub fn update_stage(
        env: Env,
        holder: Address,
        batch_id: u64,
        stage: SupplyStage,
        metadata_uri: String,
        notes: String,
    ) -> Result<(), TraceError> {
        holder.require_auth();
        Self::get_actor(env.clone(), holder.clone())?;
        let mut batch = Self::get_batch(env.clone(), batch_id)?;
        Self::ensure_active(&batch)?;
        if batch.current_holder != holder {
            return Err(TraceError::NotCurrentHolder);
        }

        batch.current_stage = stage;
        env.storage()
            .persistent()
            .set(&DataKey::Batch(batch_id), &batch);

        Self::append_event(
            &env,
            batch_id,
            holder,
            stage,
            EventType::StageUpdate,
            metadata_uri,
            notes,
        )?;
        Ok(())
    }

    /// Transfiere la custodia del lote a otro participante registrado.
    pub fn transfer_custody(
        env: Env,
        from: Address,
        to: Address,
        batch_id: u64,
        stage: SupplyStage,
        metadata_uri: String,
        notes: String,
    ) -> Result<(), TraceError> {
        from.require_auth();
        Self::get_actor(env.clone(), from.clone())?;
        Self::get_actor(env.clone(), to.clone())?;

        let mut batch = Self::get_batch(env.clone(), batch_id)?;
        Self::ensure_active(&batch)?;
        if batch.current_holder != from {
            return Err(TraceError::NotCurrentHolder);
        }

        batch.current_holder = to.clone();
        batch.current_stage = stage;
        env.storage()
            .persistent()
            .set(&DataKey::Batch(batch_id), &batch);

        Self::append_event(
            &env,
            batch_id,
            from,
            stage,
            EventType::CustodyTransfer,
            metadata_uri,
            notes,
        )?;
        Ok(())
    }

    /// Registra una inspección o control de calidad sobre el lote.
    pub fn record_quality_check(
        env: Env,
        certifier: Address,
        batch_id: u64,
        stage: SupplyStage,
        metadata_uri: String,
        notes: String,
    ) -> Result<(), TraceError> {
        certifier.require_auth();
        let actor = Self::get_actor(env.clone(), certifier.clone())?;
        if actor.role != ActorRole::Certifier {
            return Err(TraceError::UnauthorizedRole);
        }

        let batch = Self::get_batch(env.clone(), batch_id)?;
        Self::ensure_active(&batch)?;

        Self::append_event(
            &env,
            batch_id,
            certifier,
            stage,
            EventType::QualityCheck,
            metadata_uri,
            notes,
        )?;
        Ok(())
    }

    /// Marca un lote como retirado del mercado (recall).
    pub fn recall_batch(
        env: Env,
        authority: Address,
        batch_id: u64,
        reason_uri: String,
        notes: String,
    ) -> Result<(), TraceError> {
        authority.require_auth();
        let admin = Self::admin(&env)?;
        let actor = Self::get_actor(env.clone(), authority.clone())?;
        if authority != admin && actor.role != ActorRole::Certifier {
            return Err(TraceError::UnauthorizedRole);
        }

        let mut batch = Self::get_batch(env.clone(), batch_id)?;
        if batch.status == BatchStatus::Recalled {
            return Err(TraceError::BatchNotActive);
        }

        batch.status = BatchStatus::Recalled;
        env.storage()
            .persistent()
            .set(&DataKey::Batch(batch_id), &batch);

        Self::append_event(
            &env,
            batch_id,
            authority,
            batch.current_stage,
            EventType::Recall,
            reason_uri,
            notes,
        )?;
        Ok(())
    }

    pub fn get_actor(env: Env, address: Address) -> Result<Actor, TraceError> {
        env.storage()
            .persistent()
            .get(&DataKey::Actor(address))
            .ok_or(TraceError::ActorNotRegistered)
    }

    pub fn get_batch(env: Env, batch_id: u64) -> Result<ProductBatch, TraceError> {
        env.storage()
            .persistent()
            .get(&DataKey::Batch(batch_id))
            .ok_or(TraceError::BatchNotFound)
    }

    pub fn get_event(env: Env, event_id: u64) -> Result<TraceEvent, TraceError> {
        env.storage()
            .persistent()
            .get(&DataKey::Event(event_id))
            .ok_or(TraceError::EventNotFound)
    }

    /// Devuelve el historial completo de eventos de un lote, en orden cronológico.
    pub fn get_trace_history(env: Env, batch_id: u64) -> Result<Vec<TraceEvent>, TraceError> {
        Self::get_batch(env.clone(), batch_id)?;
        let event_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::BatchEvents(batch_id))
            .unwrap_or(Vec::<u64>::new(&env));

        let mut history = Vec::new(&env);
        for event_id in event_ids.iter() {
            let event = Self::get_event(env.clone(), event_id)?;
            history.push_back(event);
        }
        Ok(history)
    }

    fn admin(env: &Env) -> Result<Address, TraceError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(TraceError::NotInitialized)
    }

    fn ensure_active(batch: &ProductBatch) -> Result<(), TraceError> {
        if batch.status != BatchStatus::Active {
            return Err(TraceError::BatchNotActive);
        }
        Ok(())
    }

    fn append_event(
        env: &Env,
        batch_id: u64,
        actor: Address,
        stage: SupplyStage,
        event_type: EventType,
        metadata_uri: String,
        notes: String,
    ) -> Result<u64, TraceError> {
        let event_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextEventId)
            .ok_or(TraceError::NotInitialized)?;

        let event = TraceEvent {
            event_id,
            batch_id,
            actor,
            stage,
            event_type,
            timestamp: env.ledger().timestamp(),
            metadata_uri,
            notes,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Event(event_id), &event);
        env.storage()
            .persistent()
            .set(&DataKey::NextEventId, &(event_id + 1));

        let mut event_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::BatchEvents(batch_id))
            .unwrap_or(Vec::<u64>::new(env));
        event_ids.push_back(event_id);
        env.storage()
            .persistent()
            .set(&DataKey::BatchEvents(batch_id), &event_ids);

        Ok(event_id)
    }
}

mod test;
