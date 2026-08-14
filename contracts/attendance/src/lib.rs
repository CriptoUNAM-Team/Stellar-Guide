#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassSession {
    pub session_id: u64,
    pub course: String,
    pub topic: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextSession,
    Session(u64),
    Present(u64, Address),
    Count(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AttendanceError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    SessionNotFound = 3,
    AlreadyMarked = 4,
}

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), AttendanceError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(AttendanceError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextSession, &1u64);
        Ok(())
    }

    pub fn open_session(
        env: Env,
        course: String,
        topic: String,
    ) -> Result<u64, AttendanceError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(AttendanceError::NotInitialized)?;
        admin.require_auth();

        let session_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextSession)
            .ok_or(AttendanceError::NotInitialized)?;

        env.storage().persistent().set(
            &DataKey::Session(session_id),
            &ClassSession {
                session_id,
                course,
                topic,
                timestamp: env.ledger().timestamp(),
            },
        );
        env.storage()
            .persistent()
            .set(&DataKey::NextSession, &(session_id + 1));
        Ok(session_id)
    }

    pub fn mark_present(env: Env, session_id: u64, student: Address) -> Result<(), AttendanceError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(AttendanceError::NotInitialized)?;
        admin.require_auth();

        if !env
            .storage()
            .persistent()
            .has(&DataKey::Session(session_id))
        {
            return Err(AttendanceError::SessionNotFound);
        }
        let key = DataKey::Present(session_id, student.clone());
        if env.storage().persistent().has(&key) {
            return Err(AttendanceError::AlreadyMarked);
        }
        env.storage().persistent().set(&key, &true);
        let count: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::Count(student.clone()))
            .unwrap_or(0u32);
        env.storage()
            .persistent()
            .set(&DataKey::Count(student), &(count + 1));
        Ok(())
    }

    pub fn is_present(env: Env, session_id: u64, student: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Present(session_id, student))
            .unwrap_or(false)
    }

    pub fn get_session(env: Env, session_id: u64) -> Result<ClassSession, AttendanceError> {
        env.storage()
            .persistent()
            .get(&DataKey::Session(session_id))
            .ok_or(AttendanceError::SessionNotFound)
    }

    pub fn attendance_count(env: Env, student: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Count(student))
            .unwrap_or(0u32)
    }
}

mod test;
