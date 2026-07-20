#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_post_and_get_jobs() {
    let env = Env::default();
    let contract_id = env.register(JobBoard, ());
    let client = JobBoardClient::new(&env, &contract_id);

    let id = 1u64;
    let title = String::from_str(&env, "Rust Developer");
    let description = String::from_str(&env, "Build awesome Soroban contracts");
    let employer = String::from_str(&env, "Stellar Foundation");

    client.post_job(&id, &title, &description, &employer);

    let jobs = client.get_jobs();
    assert_eq!(jobs.len(), 1);

    let job = client.get_job(&id).unwrap();
    assert_eq!(job.id, id);
    assert_eq!(job.title, title);
}
