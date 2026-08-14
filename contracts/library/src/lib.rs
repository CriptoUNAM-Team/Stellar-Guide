#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Title {
    pub title_id: u64,
    pub title: String,
    pub copies: u32,
    pub available: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextId,
    Title(u64),
    Loan(u64, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LibraryError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    TitleNotFound = 3,
    NoCopies = 4,
    AlreadyBorrowed = 5,
    NotBorrowed = 6,
    InvalidCopies = 7,
}

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), LibraryError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(LibraryError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn add_title(env: Env, title: String, copies: u32) -> Result<u64, LibraryError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(LibraryError::NotInitialized)?;
        admin.require_auth();
        if copies == 0 {
            return Err(LibraryError::InvalidCopies);
        }
        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(LibraryError::NotInitialized)?;
        env.storage().persistent().set(
            &DataKey::Title(id),
            &Title {
                title_id: id,
                title,
                copies,
                available: copies,
            },
        );
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        Ok(id)
    }

    pub fn checkout(env: Env, title_id: u64, student: Address) -> Result<(), LibraryError> {
        student.require_auth();
        let mut book: Title = env
            .storage()
            .persistent()
            .get(&DataKey::Title(title_id))
            .ok_or(LibraryError::TitleNotFound)?;
        let loan = DataKey::Loan(title_id, student.clone());
        if env.storage().persistent().has(&loan) {
            return Err(LibraryError::AlreadyBorrowed);
        }
        if book.available == 0 {
            return Err(LibraryError::NoCopies);
        }
        book.available -= 1;
        env.storage().persistent().set(&DataKey::Title(title_id), &book);
        env.storage().persistent().set(&loan, &true);
        Ok(())
    }

    pub fn return_copy(env: Env, title_id: u64, student: Address) -> Result<(), LibraryError> {
        student.require_auth();
        let mut book: Title = env
            .storage()
            .persistent()
            .get(&DataKey::Title(title_id))
            .ok_or(LibraryError::TitleNotFound)?;
        let loan = DataKey::Loan(title_id, student);
        if !env.storage().persistent().has(&loan) {
            return Err(LibraryError::NotBorrowed);
        }
        env.storage().persistent().remove(&loan);
        book.available += 1;
        env.storage().persistent().set(&DataKey::Title(title_id), &book);
        Ok(())
    }

    pub fn get_title(env: Env, title_id: u64) -> Result<Title, LibraryError> {
        env.storage()
            .persistent()
            .get(&DataKey::Title(title_id))
            .ok_or(LibraryError::TitleNotFound)
    }

    pub fn has_loan(env: Env, title_id: u64, student: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Loan(title_id, student))
    }
}

mod test;
