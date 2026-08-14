#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_record_and_get() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let student = Address::generate(&env);
    let id = env.register(GradesContract, ());
    let client = GradesContractClient::new(&env, &id);
    client.initialize(&admin);
    client.record_grade(
        &student,
        &String::from_str(&env, "Lab-SCP"),
        &85,
        &100,
    );
    let g = client.get_grade(&student, &String::from_str(&env, "Lab-SCP"));
    assert_eq!(g.score, 85);
}

#[test]
fn test_invalid_score() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let student = Address::generate(&env);
    let id = env.register(GradesContract, ());
    let client = GradesContractClient::new(&env, &id);
    client.initialize(&admin);
    let err = client
        .try_record_grade(&student, &String::from_str(&env, "Quiz"), &11, &10)
        .unwrap_err();
    assert_eq!(err, Ok(GradeError::InvalidScore.into()));
}
