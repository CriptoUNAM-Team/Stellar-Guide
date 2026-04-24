#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Goal {
    pub id: u64,
    pub owner: Address,
    pub target_amount: i128,
    pub balance: i128,
    pub unlock_time: u64,
    pub penalty_bps: u32,
    pub closed: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    NextGoalId,
    Goal(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SavingsError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    GoalNotFound = 3,
    GoalClosed = 4,
    InvalidAmount = 5,
    InvalidPenalty = 6,
    NotOwner = 7,
    NothingToWithdraw = 8,
}

#[contract]
pub struct SavingsContract;

#[contractimpl]
impl SavingsContract {
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), SavingsError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(SavingsError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::NextGoalId, &1u64);
        Ok(())
    }

    pub fn create_goal(
        env: Env,
        owner: Address,
        target_amount: i128,
        unlock_time: u64,
        penalty_bps: u32,
    ) -> Result<u64, SavingsError> {
        owner.require_auth();
        if target_amount <= 0 {
            return Err(SavingsError::InvalidAmount);
        }
        if penalty_bps > 10_000 {
            return Err(SavingsError::InvalidPenalty);
        }
        Self::get_admin(&env)?;

        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextGoalId)
            .ok_or(SavingsError::NotInitialized)?;

        let goal = Goal {
            id,
            owner,
            target_amount,
            balance: 0,
            unlock_time,
            penalty_bps,
            closed: false,
        };
        env.storage().persistent().set(&DataKey::Goal(id), &goal);
        env.storage().persistent().set(&DataKey::NextGoalId, &(id + 1));
        Ok(id)
    }

    pub fn deposit(env: Env, goal_id: u64, from: Address, amount: i128) -> Result<(), SavingsError> {
        from.require_auth();
        if amount <= 0 {
            return Err(SavingsError::InvalidAmount);
        }
        let mut goal = Self::get_goal_internal(&env, goal_id)?;
        if goal.closed {
            return Err(SavingsError::GoalClosed);
        }

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(SavingsError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        goal.balance += amount;
        env.storage().persistent().set(&DataKey::Goal(goal_id), &goal);
        Ok(())
    }

    pub fn withdraw(env: Env, goal_id: u64, to: Address) -> Result<(i128, i128), SavingsError> {
        to.require_auth();
        let mut goal = Self::get_goal_internal(&env, goal_id)?;
        if goal.closed {
            return Err(SavingsError::GoalClosed);
        }
        if goal.owner != to {
            return Err(SavingsError::NotOwner);
        }
        if goal.balance <= 0 {
            return Err(SavingsError::NothingToWithdraw);
        }

        let now = env.ledger().timestamp();
        let penalty = if now < goal.unlock_time {
            (goal.balance * (goal.penalty_bps as i128)) / 10_000
        } else {
            0
        };
        let payout = goal.balance - penalty;

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(SavingsError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        let contract_addr = env.current_contract_address();
        let admin = Self::get_admin(&env)?;

        token_client.transfer(&contract_addr, &to, &payout);
        if penalty > 0 {
            token_client.transfer(&contract_addr, &admin, &penalty);
        }

        goal.balance = 0;
        goal.closed = true;
        env.storage().persistent().set(&DataKey::Goal(goal_id), &goal);
        Ok((payout, penalty))
    }

    pub fn get_goal(env: Env, goal_id: u64) -> Result<Goal, SavingsError> {
        Self::get_goal_internal(&env, goal_id)
    }

    fn get_goal_internal(env: &Env, goal_id: u64) -> Result<Goal, SavingsError> {
        env.storage()
            .persistent()
            .get(&DataKey::Goal(goal_id))
            .ok_or(SavingsError::GoalNotFound)
    }

    fn get_admin(env: &Env) -> Result<Address, SavingsError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(SavingsError::NotInitialized)
    }
}

mod test;
