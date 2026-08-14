#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, Address, LibraryContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(LibraryContract, ());
    let client = LibraryContractClient::new(&env, &id);
    client.initialize(&admin);
    (env, admin, client)
}

#[test]
fn test_checkout_and_return() {
    let (env, _, client) = setup();
    let student = Address::generate(&env);
    let tid = client.add_title(&String::from_str(&env, "SCP"), &1);
    client.checkout(&tid, &student);
    assert!(client.has_loan(&tid, &student));
    assert_eq!(client.get_title(&tid).available, 0);
    client.return_copy(&tid, &student);
    assert_eq!(client.get_title(&tid).available, 1);
}

#[test]
fn test_no_copies() {
    let (env, _, client) = setup();
    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let tid = client.add_title(&String::from_str(&env, "Ledger"), &1);
    client.checkout(&tid, &a);
    let err = client.try_checkout(&tid, &b).unwrap_err();
    assert_eq!(err, Ok(LibraryError::NoCopies.into()));
}
