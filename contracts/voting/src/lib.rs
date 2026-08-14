#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub proposal_id: u64,
    pub title: String,
    pub yes: u32,
    pub no: u32,
    pub open: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    NextId,
    Proposal(u64),
    Voted(u64, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VoteError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    ProposalNotFound = 3,
    Closed = 4,
    AlreadyVoted = 5,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), VoteError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(VoteError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::NextId, &1u64);
        Ok(())
    }

    pub fn create_proposal(env: Env, title: String) -> Result<u64, VoteError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(VoteError::NotInitialized)?;
        admin.require_auth();
        let id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::NextId)
            .ok_or(VoteError::NotInitialized)?;
        env.storage().persistent().set(
            &DataKey::Proposal(id),
            &Proposal {
                proposal_id: id,
                title,
                yes: 0,
                no: 0,
                open: true,
            },
        );
        env.storage().persistent().set(&DataKey::NextId, &(id + 1));
        Ok(id)
    }

    pub fn vote(env: Env, proposal_id: u64, voter: Address, support: bool) -> Result<(), VoteError> {
        voter.require_auth();
        let mut p: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .ok_or(VoteError::ProposalNotFound)?;
        if !p.open {
            return Err(VoteError::Closed);
        }
        let vkey = DataKey::Voted(proposal_id, voter.clone());
        if env.storage().persistent().has(&vkey) {
            return Err(VoteError::AlreadyVoted);
        }
        env.storage().persistent().set(&vkey, &true);
        if support {
            p.yes += 1;
        } else {
            p.no += 1;
        }
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &p);
        Ok(())
    }

    pub fn close_proposal(env: Env, proposal_id: u64) -> Result<(), VoteError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(VoteError::NotInitialized)?;
        admin.require_auth();
        let mut p: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .ok_or(VoteError::ProposalNotFound)?;
        p.open = false;
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &p);
        Ok(())
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Result<Proposal, VoteError> {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .ok_or(VoteError::ProposalNotFound)
    }
}

mod test;
