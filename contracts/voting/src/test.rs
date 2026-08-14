#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, Address, Address, VotingContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let voter = Address::generate(&env);
    let id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &id);
    client.initialize(&admin);
    (env, admin, voter, client)
}

#[test]
fn test_yes_wins() {
    let (env, _, voter, client) = setup();
    let other = Address::generate(&env);
    let pid = client.create_proposal(&String::from_str(&env, "Adoptar Testnet en el lab"));
    client.vote(&pid, &voter, &true);
    client.vote(&pid, &other, &true);
    let p = client.get_proposal(&pid);
    assert_eq!(p.yes, 2);
    assert_eq!(p.no, 0);
}

#[test]
fn test_cannot_vote_twice() {
    let (env, _, voter, client) = setup();
    let pid = client.create_proposal(&String::from_str(&env, "Horario vespertino"));
    client.vote(&pid, &voter, &false);
    let err = client.try_vote(&pid, &voter, &true).unwrap_err();
    assert_eq!(err, Ok(VoteError::AlreadyVoted.into()));
}
