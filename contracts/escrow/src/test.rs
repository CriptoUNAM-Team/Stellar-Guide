#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address, Address, EscrowContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let arbiter = Address::generate(&env);
    let payer = Address::generate(&env);
    let payee = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin);
    StellarAssetClient::new(&env, &token.address()).mint(&payer, &5_000);

    let id = env.register(EscrowContract, ());
    let client = EscrowContractClient::new(&env, &id);
    client.initialize(&arbiter, &token.address());
    (env, payer, payee, token.address(), id, client)
}

#[test]
fn test_release_to_payee() {
    let (env, payer, payee, token_addr, _, client) = setup();
    let deal = client.lock(&payer, &payee, &1_000);
    client.release(&deal);
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&payer), 4_000);
    assert_eq!(token.balance(&payee), 1_000);
    assert_eq!(client.get_deal(&deal).open, false);
}

#[test]
fn test_refund_to_payer() {
    let (env, payer, payee, token_addr, _, client) = setup();
    let deal = client.lock(&payer, &payee, &1_000);
    client.refund(&deal);
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&payer), 5_000);
    assert_eq!(token.balance(&payee), 0);
}

#[test]
fn test_cannot_release_twice() {
    let (_, payer, payee, _, _, client) = setup();
    let deal = client.lock(&payer, &payee, &500);
    client.release(&deal);
    let err = client.try_release(&deal).unwrap_err();
    assert_eq!(err, Ok(EscrowError::DealClosed.into()));
}
