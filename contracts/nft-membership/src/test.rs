#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, Address, Address, NftMembershipContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = env.register(NftMembershipContract, ());
    let client = NftMembershipContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    (env, admin, user, client)
}

#[test]
fn test_mint() {
    let (env, _, user, client) = setup();
    let id = client.mint(&user, &String::from_str(&env, "ipfs://membership/1"));
    let token = client.get_token(&id);
    assert_eq!(token.owner, user);
}

#[test]
fn test_transfer() {
    let (env, _, user, client) = setup();
    let other = Address::generate(&env);
    let id = client.mint(&user, &String::from_str(&env, "ipfs://membership/2"));
    client.transfer(&id, &other);
    let token = client.get_token(&id);
    assert_eq!(token.owner, other);
}
