#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, Address, Address, AttendanceContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let student = Address::generate(&env);
    let id = env.register(AttendanceContract, ());
    let client = AttendanceContractClient::new(&env, &id);
    client.initialize(&admin);
    (env, admin, student, client)
}

#[test]
fn test_mark_and_count() {
    let (env, _, student, client) = setup();
    let sid = client.open_session(
        &String::from_str(&env, "Computacion"),
        &String::from_str(&env, "Consenso SCP"),
    );
    client.mark_present(&sid, &student);
    assert!(client.is_present(&sid, &student));
    assert_eq!(client.attendance_count(&student), 1);
}

#[test]
fn test_double_mark_fails() {
    let (env, _, student, client) = setup();
    let sid = client.open_session(
        &String::from_str(&env, "Redes"),
        &String::from_str(&env, "Ledger"),
    );
    client.mark_present(&sid, &student);
    let err = client.try_mark_present(&sid, &student).unwrap_err();
    assert_eq!(err, Ok(AttendanceError::AlreadyMarked.into()));
}
