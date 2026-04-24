#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub id: u64,
    pub borrower: Address,
    pub collateral: i128,
    pub debt: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    NextId,
    MinCollateralBps,
    Position(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LoanError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidAmount = 3,
    PositionNotFound = 4,
    Unauthorized = 5,
    PositionClosed = 6,
    InsufficientCollateral = 7,
    NoDebt = 8,
}

#[contract]
pub struct LoanContract;

#[contractimpl]
impl LoanContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        min_collateral_bps: u32,
    ) -> Result<(), LoanError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(LoanError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage()
            .persistent()
            .set(&DataKey::MinCollateralBps, &min_collateral_bps);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn create_position(
        env: Env,
        borrower: Address,
        collateral_amount: i128,
    ) -> Result<u64, LoanError> {
        borrower.require_auth();
        if collateral_amount <= 0 {
            return Err(LoanError::InvalidAmount);
        }
        Self::get_admin(&env)?;
        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(LoanError::NotInitialized)?;
        let pos = Position {
            id,
            borrower,
            collateral: collateral_amount,
            debt: 0,
            active: true,
        };
        env.storage().persistent().set(&DataKey::Position(id), &pos);
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        Ok(id)
    }

    pub fn borrow(env: Env, position_id: u64, debt_amount: i128) -> Result<(), LoanError> {
        if debt_amount <= 0 {
            return Err(LoanError::InvalidAmount);
        }
        let mut pos = Self::get_position_internal(&env, position_id)?;
        if !pos.active {
            return Err(LoanError::PositionClosed);
        }
        pos.borrower.require_auth();

        let min_bps: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::MinCollateralBps)
            .ok_or(LoanError::NotInitialized)?;
        let max_debt = (pos.collateral * 10_000i128) / (min_bps as i128);
        if pos.debt + debt_amount > max_debt {
            return Err(LoanError::InsufficientCollateral);
        }

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(LoanError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(
            &env.current_contract_address(),
            &pos.borrower,
            &debt_amount,
        );

        pos.debt += debt_amount;
        env.storage().persistent().set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    pub fn repay(env: Env, position_id: u64, amount: i128) -> Result<(), LoanError> {
        if amount <= 0 {
            return Err(LoanError::InvalidAmount);
        }
        let mut pos = Self::get_position_internal(&env, position_id)?;
        if !pos.active {
            return Err(LoanError::PositionClosed);
        }
        if pos.debt <= 0 {
            return Err(LoanError::NoDebt);
        }
        pos.borrower.require_auth();

        let pay = if amount > pos.debt { pos.debt } else { amount };
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(LoanError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&pos.borrower, &env.current_contract_address(), &pay);

        pos.debt -= pay;
        env.storage().persistent().set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    pub fn close_position(env: Env, position_id: u64) -> Result<(), LoanError> {
        let mut pos = Self::get_position_internal(&env, position_id)?;
        pos.borrower.require_auth();
        if pos.debt != 0 {
            return Err(LoanError::NoDebt);
        }
        pos.active = false;
        env.storage().persistent().set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    pub fn get_position(env: Env, position_id: u64) -> Result<Position, LoanError> {
        Self::get_position_internal(&env, position_id)
    }

    fn get_position_internal(env: &Env, position_id: u64) -> Result<Position, LoanError> {
        env.storage()
            .persistent()
            .get(&DataKey::Position(position_id))
            .ok_or(LoanError::PositionNotFound)
    }

    fn get_admin(env: &Env) -> Result<Address, LoanError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(LoanError::NotInitialized)
    }
}

mod test;
