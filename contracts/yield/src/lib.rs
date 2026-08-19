#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    TotalAssets,
    TotalShares,
    Share(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum YieldError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidAmount = 3,
    NoShares = 4,
}

#[contract]
pub struct YieldVaultContract;

#[contractimpl]
impl YieldVaultContract {
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), YieldError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(YieldError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::TotalAssets, &0i128);
        env.storage().persistent().set(&DataKey::TotalShares, &0i128);
        Ok(())
    }

    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<i128, YieldError> {
        user.require_auth();
        if amount <= 0 {
            return Err(YieldError::InvalidAmount);
        }
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(YieldError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&user, env.current_contract_address(), &amount);

        let total_assets = Self::total_assets(&env)?;
        let total_shares = Self::total_shares(&env)?;
        let minted_shares = if total_shares == 0 || total_assets == 0 {
            amount
        } else {
            (amount * total_shares) / total_assets
        };

        let user_shares = Self::user_shares(&env, user.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Share(user), &(user_shares + minted_shares));
        env.storage()
            .persistent()
            .set(&DataKey::TotalAssets, &(total_assets + amount));
        env.storage()
            .persistent()
            .set(&DataKey::TotalShares, &(total_shares + minted_shares));
        Ok(minted_shares)
    }

    pub fn harvest(env: Env, yield_amount: i128) -> Result<(), YieldError> {
        let admin = Self::admin(&env)?;
        admin.require_auth();
        if yield_amount <= 0 {
            return Err(YieldError::InvalidAmount);
        }
        let total_assets = Self::total_assets(&env)?;
        env.storage()
            .persistent()
            .set(&DataKey::TotalAssets, &(total_assets + yield_amount));
        Ok(())
    }

    pub fn withdraw(env: Env, user: Address, shares: i128) -> Result<i128, YieldError> {
        user.require_auth();
        if shares <= 0 {
            return Err(YieldError::InvalidAmount);
        }
        let total_assets = Self::total_assets(&env)?;
        let total_shares = Self::total_shares(&env)?;
        if total_shares <= 0 {
            return Err(YieldError::NoShares);
        }
        let user_shares = Self::user_shares(&env, user.clone());
        if shares > user_shares {
            return Err(YieldError::NoShares);
        }
        let amount_out = (shares * total_assets) / total_shares;

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(YieldError::NotInitialized)?;
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &user, &amount_out);

        env.storage()
            .persistent()
            .set(&DataKey::Share(user), &(user_shares - shares));
        env.storage()
            .persistent()
            .set(&DataKey::TotalAssets, &(total_assets - amount_out));
        env.storage()
            .persistent()
            .set(&DataKey::TotalShares, &(total_shares - shares));
        Ok(amount_out)
    }

    pub fn get_share_balance(env: Env, user: Address) -> i128 {
        Self::user_shares(&env, user)
    }

    pub fn get_totals(env: Env) -> Result<(i128, i128), YieldError> {
        Ok((Self::total_assets(&env)?, Self::total_shares(&env)?))
    }

    fn admin(env: &Env) -> Result<Address, YieldError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(YieldError::NotInitialized)
    }
    fn total_assets(env: &Env) -> Result<i128, YieldError> {
        env.storage()
            .persistent()
            .get(&DataKey::TotalAssets)
            .ok_or(YieldError::NotInitialized)
    }
    fn total_shares(env: &Env) -> Result<i128, YieldError> {
        env.storage()
            .persistent()
            .get(&DataKey::TotalShares)
            .ok_or(YieldError::NotInitialized)
    }
    fn user_shares(env: &Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Share(user))
            .unwrap_or(0i128)
    }
}

mod test;
