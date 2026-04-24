#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, PayrollContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract_v2(token_admin);
    let sac = StellarAssetClient::new(&env, &token_addr.address());
    sac.mint(&admin, &10_000_000);

    let contract_id = env.register(PayrollContract, ());
    let client = PayrollContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_addr.address());
    (env, admin, token_addr.address(), client)
}

#[test]
fn test_add_and_disperse_period() {
    let (env, admin, token_addr, client) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.add_recipient(&alice, &1_000);
    client.add_recipient(&bob, &2_000);

    let total = client.disperse_period(&202604);
    assert_eq!(total, 3_000);

    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&alice), 1_000);
    assert_eq!(token.balance(&bob), 2_000);
    assert_eq!(token.balance(&admin), 10_000_000 - 3_000);
}

#[test]
fn test_disperse_same_period_fails() {
    let (env, _, _, client) = setup();
    let alice = Address::generate(&env);
    client.add_recipient(&alice, &1_000);
    client.disperse_period(&202604);

    let err = client.try_disperse_period(&202604).unwrap_err();
    assert_eq!(err, Ok(PayrollError::PeriodAlreadyExecuted.into()));
}

#[test]
fn test_remove_recipient() {
    let (env, _, _, client) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    client.add_recipient(&alice, &500);
    client.add_recipient(&bob, &500);
    client.remove_recipient(&alice);

    let all = client.get_all_recipients();
    assert_eq!(all.len(), 1);
}
