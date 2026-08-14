#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address, Address, AmmContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let lp = Address::generate(&env);
    let trader = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let a = env.register_stellar_asset_contract_v2(token_admin.clone());
    let b = env.register_stellar_asset_contract_v2(token_admin);
    let sac_a = StellarAssetClient::new(&env, &a.address());
    let sac_b = StellarAssetClient::new(&env, &b.address());
    sac_a.mint(&lp, &10_000);
    sac_b.mint(&lp, &20_000);
    sac_a.mint(&trader, &1_000);

    let id = env.register(AmmContract, ());
    let client = AmmContractClient::new(&env, &id);
    client.initialize(&admin, &a.address(), &b.address());
    (env, lp, trader, a.address(), b.address(), client)
}

#[test]
fn test_swap_conserves_k() {
    let (env, lp, trader, token_a, token_b, client) = setup();
    client.add_liquidity(&lp, &10_000, &20_000);
    let out = client.swap_a_for_b(&trader, &1_000);
    // 1000 * 20000 / (10000 + 1000) = 1818
    assert_eq!(out, 1818);

    let r = client.get_reserves();
    assert_eq!(r.reserve_a, 11_000);
    assert_eq!(r.reserve_b, 20_000 - 1818);

    let ta = TokenClient::new(&env, &token_a);
    let tb = TokenClient::new(&env, &token_b);
    assert_eq!(ta.balance(&trader), 0);
    assert_eq!(tb.balance(&trader), 1818);
}

#[test]
fn test_empty_pool_cannot_swap() {
    let (_, _, trader, _, _, client) = setup();
    let err = client.try_swap_a_for_b(&trader, &100).unwrap_err();
    assert_eq!(err, Ok(AmmError::EmptyPool.into()));
}
