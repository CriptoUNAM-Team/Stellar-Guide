#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address, Address, LoanContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let borrower = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract_v2(token_admin);
    let sac = StellarAssetClient::new(&env, &token_addr.address());
    let contract_id = env.register(LoanContract, ());
    sac.mint(&contract_id, &20_000);
    sac.mint(&borrower, &10_000);
    let client = LoanContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_addr.address(), &15_000);
    (env, admin, borrower, token_addr.address(), contract_id, client)
}

#[test]
fn test_borrow_and_repay() {
    let (env, _admin, borrower, token_addr, contract_id, client) = setup();
    let pos = client.create_position(&borrower, &3_000);
    client.borrow(&pos, &2_000);
    let p = client.get_position(&pos);
    assert_eq!(p.debt, 2_000);

    client.repay(&pos, &500);
    let p2 = client.get_position(&pos);
    assert_eq!(p2.debt, 1_500);

    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&borrower), 10_000 - 3_000 + 2_000 - 500);
    assert_eq!(token.balance(&contract_id), 20_000 + 3_000 - 2_000 + 500);
}

#[test]
fn test_cannot_overborrow() {
    let (_, _, borrower, _, _, client) = setup();
    let pos = client.create_position(&borrower, &1_000);
    let err = client.try_borrow(&pos, &800).unwrap_err();
    assert_eq!(err, Ok(LoanError::InsufficientCollateral.into()));
}

#[test]
fn test_close_position_when_debt_zero() {
    let (env, _, borrower, token_addr, _, client) = setup();
    let pos = client.create_position(&borrower, &2_000);
    client.borrow(&pos, &1_000);
    client.repay(&pos, &1_000);
    client.close_position(&pos);
    let p = client.get_position(&pos);
    assert_eq!(p.active, false);
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&borrower), 10_000);
}

#[test]
fn test_liquidate_after_threshold_hike() {
    let (env, _admin, borrower, token_addr, _, client) = setup();
    let liquidator = Address::generate(&env);
    StellarAssetClient::new(&env, &token_addr).mint(&liquidator, &5_000);

    let pos = client.create_position(&borrower, &3_000);
    client.borrow(&pos, &2_000);
    assert_eq!(client.is_liquidatable(&pos), false);

    client.set_min_collateral_bps(&20_000);
    assert_eq!(client.is_liquidatable(&pos), true);

    client.liquidate(&liquidator, &pos);
    let p = client.get_position(&pos);
    assert_eq!(p.active, false);
    assert_eq!(p.debt, 0);
    let token = TokenClient::new(&env, &token_addr);
    assert_eq!(token.balance(&liquidator), 5_000 - 2_000 + 3_000);
}
