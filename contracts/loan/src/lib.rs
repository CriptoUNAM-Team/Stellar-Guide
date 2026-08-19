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
    PositionHealthy = 9,
}

#[contract]
pub struct LoanContract;

#[contractimpl]
impl LoanContract {
    /// `min_collateral_bps` 15000 = 150% de colateral vs deuda (LTV máx. ~66%).
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        min_collateral_bps: u32,
    ) -> Result<(), LoanError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(LoanError::AlreadyInitialized);
        }
        if min_collateral_bps < 10_000 {
            return Err(LoanError::InvalidAmount);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage()
            .persistent()
            .set(&DataKey::MinCollateralBps, &min_collateral_bps);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    /// Deposita colateral en el contrato y abre una posición.
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
        Self::transfer_in(&env, &borrower, collateral_amount)?;

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

    pub fn deposit_collateral(
        env: Env,
        position_id: u64,
        amount: i128,
    ) -> Result<(), LoanError> {
        if amount <= 0 {
            return Err(LoanError::InvalidAmount);
        }
        let mut pos = Self::get_position_internal(&env, position_id)?;
        if !pos.active {
            return Err(LoanError::PositionClosed);
        }
        pos.borrower.require_auth();
        Self::transfer_in(&env, &pos.borrower, amount)?;
        pos.collateral += amount;
        env.storage()
            .persistent()
            .set(&DataKey::Position(position_id), &pos);
        Ok(())
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

        let min_bps = Self::min_bps(&env)?;
        let new_debt = pos.debt + debt_amount;
        if !Self::is_healthy(pos.collateral, new_debt, min_bps) {
            return Err(LoanError::InsufficientCollateral);
        }

        Self::transfer_out(&env, &pos.borrower, debt_amount)?;
        pos.debt = new_debt;
        env.storage()
            .persistent()
            .set(&DataKey::Position(position_id), &pos);
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
        Self::transfer_in(&env, &pos.borrower, pay)?;
        pos.debt -= pay;
        env.storage()
            .persistent()
            .set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    /// El liquidator paga la deuda y recibe el colateral (demo DeFi).
    pub fn liquidate(
        env: Env,
        liquidator: Address,
        position_id: u64,
    ) -> Result<(), LoanError> {
        liquidator.require_auth();
        let mut pos = Self::get_position_internal(&env, position_id)?;
        if !pos.active {
            return Err(LoanError::PositionClosed);
        }
        if pos.debt <= 0 {
            return Err(LoanError::NoDebt);
        }
        let min_bps = Self::min_bps(&env)?;
        if Self::is_healthy(pos.collateral, pos.debt, min_bps) {
            return Err(LoanError::PositionHealthy);
        }

        Self::transfer_in(&env, &liquidator, pos.debt)?;
        Self::transfer_out(&env, &liquidator, pos.collateral)?;
        pos.debt = 0;
        pos.collateral = 0;
        pos.active = false;
        env.storage()
            .persistent()
            .set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    /// Solo admin: subir el umbral deja posiciones bajo-colateralizadas (demo de liquidación).
    pub fn set_min_collateral_bps(env: Env, min_collateral_bps: u32) -> Result<(), LoanError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if min_collateral_bps < 10_000 {
            return Err(LoanError::InvalidAmount);
        }
        env.storage()
            .persistent()
            .set(&DataKey::MinCollateralBps, &min_collateral_bps);
        Ok(())
    }

    pub fn close_position(env: Env, position_id: u64) -> Result<(), LoanError> {
        let mut pos = Self::get_position_internal(&env, position_id)?;
        pos.borrower.require_auth();
        if pos.debt != 0 {
            return Err(LoanError::NoDebt);
        }
        if pos.collateral > 0 {
            Self::transfer_out(&env, &pos.borrower, pos.collateral)?;
            pos.collateral = 0;
        }
        pos.active = false;
        env.storage()
            .persistent()
            .set(&DataKey::Position(position_id), &pos);
        Ok(())
    }

    pub fn get_position(env: Env, position_id: u64) -> Result<Position, LoanError> {
        Self::get_position_internal(&env, position_id)
    }

    pub fn is_liquidatable(env: Env, position_id: u64) -> Result<bool, LoanError> {
        let pos = Self::get_position_internal(&env, position_id)?;
        if !pos.active || pos.debt <= 0 {
            return Ok(false);
        }
        let min_bps = Self::min_bps(&env)?;
        Ok(!Self::is_healthy(pos.collateral, pos.debt, min_bps))
    }

    fn is_healthy(collateral: i128, debt: i128, min_bps: u32) -> bool {
        if debt <= 0 {
            return true;
        }
        collateral * 10_000i128 >= debt * (min_bps as i128)
    }

    fn min_bps(env: &Env) -> Result<u32, LoanError> {
        env.storage()
            .persistent()
            .get(&DataKey::MinCollateralBps)
            .ok_or(LoanError::NotInitialized)
    }

    fn token(env: &Env) -> Result<Address, LoanError> {
        env.storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(LoanError::NotInitialized)
    }

    fn transfer_in(env: &Env, from: &Address, amount: i128) -> Result<(), LoanError> {
        let token_addr = Self::token(env)?;
        token::Client::new(env, &token_addr).transfer(
            from,
            env.current_contract_address(),
            &amount,
        );
        Ok(())
    }

    fn transfer_out(env: &Env, to: &Address, amount: i128) -> Result<(), LoanError> {
        let token_addr = Self::token(env)?;
        token::Client::new(env, &token_addr).transfer(
            &env.current_contract_address(),
            to,
            &amount,
        );
        Ok(())
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
