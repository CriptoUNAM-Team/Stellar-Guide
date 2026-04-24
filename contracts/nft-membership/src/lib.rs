#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NftToken {
    pub token_id: u64,
    pub owner: Address,
    pub metadata_uri: String,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextId,
    Token(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NftError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    TokenNotFound = 3,
    NotOwner = 4,
}

#[contract]
pub struct NftMembershipContract;

#[contractimpl]
impl NftMembershipContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), NftError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(NftError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn mint(env: Env, to: Address, metadata_uri: String) -> Result<u64, NftError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(NftError::NotInitialized)?;
        admin.require_auth();

        let token_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(NftError::NotInitialized)?;
        let token = NftToken {
            token_id,
            owner: to,
            metadata_uri,
        };
        env.storage().persistent().set(&DataKey::Token(token_id), &token);
        env.storage()
            .persistent()
            .set(&DataKey::NextId, &(token_id + 1));
        Ok(token_id)
    }

    pub fn transfer(env: Env, token_id: u64, to: Address) -> Result<(), NftError> {
        let mut token: NftToken = env
            .storage()
            .persistent()
            .get(&DataKey::Token(token_id))
            .ok_or(NftError::TokenNotFound)?;
        token.owner.require_auth();
        token.owner = to;
        env.storage().persistent().set(&DataKey::Token(token_id), &token);
        Ok(())
    }

    pub fn get_token(env: Env, token_id: u64) -> Result<NftToken, NftError> {
        env.storage()
            .persistent()
            .get(&DataKey::Token(token_id))
            .ok_or(NftError::TokenNotFound)
    }
}

mod test;
