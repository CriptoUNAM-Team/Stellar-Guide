#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deal {
    pub deal_id: u64,
    pub payer: Address,
    pub payee: Address,
    pub amount: i128,
    pub open: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Arbiter,
    Token,
    NextId,
    Deal(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidAmount = 3,
    DealNotFound = 4,
    DealClosed = 5,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn initialize(env: Env, arbiter: Address, token: Address) -> Result<(), EscrowError> {
        if env.storage().persistent().has(&DataKey::Arbiter) {
            return Err(EscrowError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Arbiter, &arbiter);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn lock(env: Env, payer: Address, payee: Address, amount: i128) -> Result<u64, EscrowError> {
        payer.require_auth();
        if amount <= 0 {
            return Err(EscrowError::InvalidAmount);
        }
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(EscrowError::NotInitialized)?;
        token::Client::new(&env, &token_addr).transfer(
            &payer,
            &env.current_contract_address(),
            &amount,
        );
        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(EscrowError::NotInitialized)?;
        env.storage().persistent().set(
            &DataKey::Deal(id),
            &Deal {
                deal_id: id,
                payer,
                payee,
                amount,
                open: true,
            },
        );
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        Ok(id)
    }

    pub fn release(env: Env, deal_id: u64) -> Result<(), EscrowError> {
        let arbiter: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Arbiter)
            .ok_or(EscrowError::NotInitialized)?;
        arbiter.require_auth();
        let mut deal = Self::open_deal(&env, deal_id)?;
        Self::transfer_out(&env, &deal.payee, deal.amount)?;
        deal.open = false;
        env.storage().persistent().set(&DataKey::Deal(deal_id), &deal);
        Ok(())
    }

    pub fn refund(env: Env, deal_id: u64) -> Result<(), EscrowError> {
        let arbiter: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Arbiter)
            .ok_or(EscrowError::NotInitialized)?;
        arbiter.require_auth();
        let mut deal = Self::open_deal(&env, deal_id)?;
        Self::transfer_out(&env, &deal.payer, deal.amount)?;
        deal.open = false;
        env.storage().persistent().set(&DataKey::Deal(deal_id), &deal);
        Ok(())
    }

    pub fn get_deal(env: Env, deal_id: u64) -> Result<Deal, EscrowError> {
        env.storage()
            .persistent()
            .get(&DataKey::Deal(deal_id))
            .ok_or(EscrowError::DealNotFound)
    }

    fn open_deal(env: &Env, deal_id: u64) -> Result<Deal, EscrowError> {
        let deal: Deal = env
            .storage()
            .persistent()
            .get(&DataKey::Deal(deal_id))
            .ok_or(EscrowError::DealNotFound)?;
        if !deal.open {
            return Err(EscrowError::DealClosed);
        }
        Ok(deal)
    }

    fn transfer_out(env: &Env, to: &Address, amount: i128) -> Result<(), EscrowError> {
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(EscrowError::NotInitialized)?;
        token::Client::new(env, &token_addr).transfer(&env.current_contract_address(), to, &amount);
        Ok(())
    }
}

mod test;
