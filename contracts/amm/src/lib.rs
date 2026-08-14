#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

/// AMM de producto constante (x * y = k). Pensado para explicar swaps en sesión técnica.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reserves {
    pub token_a: Address,
    pub token_b: Address,
    pub reserve_a: i128,
    pub reserve_b: i128,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TokenA,
    TokenB,
    ReserveA,
    ReserveB,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AmmError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InvalidAmount = 3,
    SameToken = 4,
    EmptyPool = 5,
}

#[contract]
pub struct AmmContract;

#[contractimpl]
impl AmmContract {
    pub fn initialize(env: Env, admin: Address, token_a: Address, token_b: Address) -> Result<(), AmmError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(AmmError::AlreadyInitialized);
        }
        if token_a == token_b {
            return Err(AmmError::SameToken);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::TokenA, &token_a);
        env.storage().persistent().set(&DataKey::TokenB, &token_b);
        env.storage().persistent().set(&DataKey::ReserveA, &0i128);
        env.storage().persistent().set(&DataKey::ReserveB, &0i128);
        Ok(())
    }

    pub fn add_liquidity(env: Env, provider: Address, amount_a: i128, amount_b: i128) -> Result<(), AmmError> {
        provider.require_auth();
        if amount_a <= 0 || amount_b <= 0 {
            return Err(AmmError::InvalidAmount);
        }
        let (token_a, token_b) = Self::tokens(&env)?;
        token::Client::new(&env, &token_a).transfer(&provider, &env.current_contract_address(), &amount_a);
        token::Client::new(&env, &token_b).transfer(&provider, &env.current_contract_address(), &amount_b);
        let ra = Self::reserve(&env, &DataKey::ReserveA)?;
        let rb = Self::reserve(&env, &DataKey::ReserveB)?;
        env.storage().persistent().set(&DataKey::ReserveA, &(ra + amount_a));
        env.storage().persistent().set(&DataKey::ReserveB, &(rb + amount_b));
        Ok(())
    }

    /// Intercambia token A por B. `k` se conserva (sin fee, para el lab).
    pub fn swap_a_for_b(env: Env, trader: Address, amount_in: i128) -> Result<i128, AmmError> {
        trader.require_auth();
        if amount_in <= 0 {
            return Err(AmmError::InvalidAmount);
        }
        let ra = Self::reserve(&env, &DataKey::ReserveA)?;
        let rb = Self::reserve(&env, &DataKey::ReserveB)?;
        if ra == 0 || rb == 0 {
            return Err(AmmError::EmptyPool);
        }
        let amount_out = (amount_in * rb) / (ra + amount_in);
        if amount_out <= 0 || amount_out >= rb {
            return Err(AmmError::InvalidAmount);
        }
        let (token_a, token_b) = Self::tokens(&env)?;
        token::Client::new(&env, &token_a).transfer(&trader, &env.current_contract_address(), &amount_in);
        token::Client::new(&env, &token_b).transfer(&env.current_contract_address(), &trader, &amount_out);
        env.storage().persistent().set(&DataKey::ReserveA, &(ra + amount_in));
        env.storage().persistent().set(&DataKey::ReserveB, &(rb - amount_out));
        Ok(amount_out)
    }

    pub fn get_reserves(env: Env) -> Result<Reserves, AmmError> {
        let (token_a, token_b) = Self::tokens(&env)?;
        Ok(Reserves {
            token_a,
            token_b,
            reserve_a: Self::reserve(&env, &DataKey::ReserveA)?,
            reserve_b: Self::reserve(&env, &DataKey::ReserveB)?,
        })
    }

    fn tokens(env: &Env) -> Result<(Address, Address), AmmError> {
        let a = env
            .storage()
            .persistent()
            .get(&DataKey::TokenA)
            .ok_or(AmmError::NotInitialized)?;
        let b = env
            .storage()
            .persistent()
            .get(&DataKey::TokenB)
            .ok_or(AmmError::NotInitialized)?;
        Ok((a, b))
    }

    fn reserve(env: &Env, key: &DataKey) -> Result<i128, AmmError> {
        env.storage()
            .persistent()
            .get(key)
            .ok_or(AmmError::NotInitialized)
    }
}

mod test;
