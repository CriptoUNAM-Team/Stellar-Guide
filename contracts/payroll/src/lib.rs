#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recipient {
    pub address: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Recipient(Address),
    RecipientList,
    PaidPeriod(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PayrollError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    RecipientNotFound = 3,
    RecipientAlreadyExists = 4,
    InvalidAmount = 5,
    NoRecipients = 6,
    PeriodAlreadyExecuted = 7,
}

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), PayrollError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(PayrollError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage()
            .persistent()
            .set(&DataKey::RecipientList, &Vec::<Address>::new(&env));
        Ok(())
    }

    pub fn add_recipient(
        env: Env,
        recipient: Address,
        amount: i128,
    ) -> Result<(), PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(PayrollError::InvalidAmount);
        }
        if env
            .storage()
            .persistent()
            .has(&DataKey::Recipient(recipient.clone()))
        {
            return Err(PayrollError::RecipientAlreadyExists);
        }

        env.storage().persistent().set(
            &DataKey::Recipient(recipient.clone()),
            &Recipient {
                address: recipient.clone(),
                amount,
            },
        );

        let mut list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::RecipientList)
            .unwrap_or(Vec::new(&env));
        list.push_back(recipient);
        env.storage().persistent().set(&DataKey::RecipientList, &list);
        Ok(())
    }

    pub fn remove_recipient(env: Env, recipient: Address) -> Result<(), PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Recipient(recipient.clone()))
        {
            return Err(PayrollError::RecipientNotFound);
        }

        env.storage()
            .persistent()
            .remove(&DataKey::Recipient(recipient.clone()));
        let list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::RecipientList)
            .unwrap_or(Vec::new(&env));
        let mut new_list = Vec::new(&env);
        for addr in list.iter() {
            if addr != recipient {
                new_list.push_back(addr);
            }
        }
        env.storage()
            .persistent()
            .set(&DataKey::RecipientList, &new_list);
        Ok(())
    }

    pub fn get_recipient(env: Env, recipient: Address) -> Result<Recipient, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::Recipient(recipient))
            .ok_or(PayrollError::RecipientNotFound)
    }

    pub fn get_all_recipients(env: Env) -> Result<Vec<Address>, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::RecipientList)
            .ok_or(PayrollError::NotInitialized)
    }

    pub fn disperse_period(env: Env, period_id: u64) -> Result<i128, PayrollError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if env.storage().persistent().has(&DataKey::PaidPeriod(period_id)) {
            return Err(PayrollError::PeriodAlreadyExecuted);
        }

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .ok_or(PayrollError::NotInitialized)?;
        let list: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::RecipientList)
            .ok_or(PayrollError::NotInitialized)?;
        if list.is_empty() {
            return Err(PayrollError::NoRecipients);
        }

        let token_client = token::Client::new(&env, &token_addr);
        let mut total = 0i128;
        for recipient in list.iter() {
            let data: Recipient = env
                .storage()
                .persistent()
                .get(&DataKey::Recipient(recipient.clone()))
                .ok_or(PayrollError::RecipientNotFound)?;
            token_client.transfer(&admin, &data.address, &data.amount);
            total += data.amount;
        }

        env.storage()
            .persistent()
            .set(&DataKey::PaidPeriod(period_id), &true);
        Ok(total)
    }

    fn get_admin(env: &Env) -> Result<Address, PayrollError> {
        env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(PayrollError::NotInitialized)
    }
}

mod test;
