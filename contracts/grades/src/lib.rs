#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grade {
    pub student: Address,
    pub assignment: String,
    pub score: u32,
    pub max_score: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Grade(Address, String),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GradeError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidScore = 3,
    GradeNotFound = 4,
}

#[contract]
pub struct GradesContract;

#[contractimpl]
impl GradesContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), GradeError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(GradeError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn record_grade(
        env: Env,
        student: Address,
        assignment: String,
        score: u32,
        max_score: u32,
    ) -> Result<(), GradeError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(GradeError::NotInitialized)?;
        admin.require_auth();
        if max_score == 0 || score > max_score {
            return Err(GradeError::InvalidScore);
        }
        env.storage().persistent().set(
            &DataKey::Grade(student.clone(), assignment.clone()),
            &Grade {
                student,
                assignment,
                score,
                max_score,
            },
        );
        Ok(())
    }

    pub fn get_grade(
        env: Env,
        student: Address,
        assignment: String,
    ) -> Result<Grade, GradeError> {
        env.storage()
            .persistent()
            .get(&DataKey::Grade(student, assignment))
            .ok_or(GradeError::GradeNotFound)
    }
}

mod test;
