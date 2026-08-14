#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, EnrollmentContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(EnrollmentContract, ());
    let client = EnrollmentContractClient::new(&env, &id);
    client.initialize(&admin);
    (env, client)
}

#[test]
fn test_enroll_and_drop() {
    let (env, client) = setup();
    let student = Address::generate(&env);
    let cid = client.create_course(&String::from_str(&env, "Soroban 101"), &2);
    client.enroll(&cid, &student);
    assert!(client.is_enrolled(&cid, &student));
    assert_eq!(client.get_course(&cid).enrolled, 1);
    client.drop_course(&cid, &student);
    assert_eq!(client.get_course(&cid).enrolled, 0);
}

#[test]
fn test_course_full() {
    let (env, client) = setup();
    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let cid = client.create_course(&String::from_str(&env, "Lab"), &1);
    client.enroll(&cid, &a);
    let err = client.try_enroll(&cid, &b).unwrap_err();
    assert_eq!(err, Ok(EnrollmentError::CourseFull.into()));
}
