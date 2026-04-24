#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address, YieldVaultContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract_v2(token_admin);
    let sac = StellarAssetClient::new(&env, &token_addr.address());
    sac.mint(&user, &20_000);

    let contract_id = env.register(YieldVaultContract, ());
    let client = YieldVaultContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_addr.address());
    (env, admin, user, token_addr.address(), client)
}

#[test]
fn test_deposit_and_withdraw() {
    let (env, admin, user, token_addr, client) = setup();
    let shares = client.deposit(&user, &10_000);
    assert_eq!(shares, 10_000);
    client.harvest(&2_000);

    let out = client.withdraw(&user, &5_000);
    assert_eq!(out, 6_000);

    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&user), 16_000);
    assert_eq!(token.balance(&admin), 0);
}

#[test]
fn test_share_balance() {
    let (_, _, user, _, client) = setup();
    client.deposit(&user, &5_000);
    assert_eq!(client.get_share_balance(&user), 5_000);
}

#[test]
fn test_totals() {
    let (_, _, user, _, client) = setup();
    client.deposit(&user, &8_000);
    let (assets, shares) = client.get_totals();
    assert_eq!(assets, 8_000);
    assert_eq!(shares, 8_000);
}
