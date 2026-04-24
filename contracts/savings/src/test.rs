#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (
    Env,
    Address,
    Address,
    Address,
    SavingsContractClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1_000);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract_v2(token_admin);
    let sac = StellarAssetClient::new(&env, &token_addr.address());
    sac.mint(&user, &10_000);

    let contract_id = env.register(SavingsContract, ());
    let client = SavingsContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_addr.address());
    (env, admin, user, token_addr.address(), client)
}

#[test]
fn test_goal_deposit_and_withdraw_without_penalty() {
    let (env, _admin, user, token_addr, client) = setup();
    let goal_id = client.create_goal(&user, &5_000, &1_100, &500);
    client.deposit(&goal_id, &user, &3_000);

    env.ledger().with_mut(|li| li.timestamp = 1_200);
    let (payout, penalty) = client.withdraw(&goal_id, &user);
    assert_eq!(payout, 3_000);
    assert_eq!(penalty, 0);

    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&user), 10_000);
}

#[test]
fn test_withdraw_early_applies_penalty() {
    let (env, admin, user, token_addr, client) = setup();
    let goal_id = client.create_goal(&user, &5_000, &2_000, &1_000);
    client.deposit(&goal_id, &user, &2_000);

    // timestamp sigue en 1_000, sigue bloqueado
    let (payout, penalty) = client.withdraw(&goal_id, &user);
    assert_eq!(payout, 1_800);
    assert_eq!(penalty, 200);

    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&user), 9_800);
    assert_eq!(token.balance(&admin), 200);
}

#[test]
fn test_only_owner_can_withdraw() {
    let (env, _admin, user, _token_addr, client) = setup();
    let attacker = Address::generate(&env);
    let goal_id = client.create_goal(&user, &1_000, &1_500, &300);
    client.deposit(&goal_id, &user, &500);

    let err = client.try_withdraw(&goal_id, &attacker).unwrap_err();
    assert_eq!(err, Ok(SavingsError::NotOwner.into()));
}
