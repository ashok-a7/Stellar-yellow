#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Vec, String};

#[derive(Clone)]
#[contract]
pub struct JobBoard;

#[derive(Clone)]
#[contracttype]
pub struct Job {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub employer: String,
}

#[contractimpl]
impl JobBoard {

    // Add a new job
    pub fn post_job(env: Env, id: u64, title: String, description: String, employer: String) {
        let mut jobs: Vec<Job> = env.storage().instance().get(&symbol_short!("JOBS"))
            .unwrap_or(Vec::new(&env));

        let job = Job {
            id,
            title,
            description,
            employer,
        };

        jobs.push_back(job);
        env.storage().instance().set(&symbol_short!("JOBS"), &jobs);
    }

    // Get all jobs
    pub fn get_jobs(env: Env) -> Vec<Job> {
        env.storage().instance().get(&symbol_short!("JOBS"))
            .unwrap_or(Vec::new(&env))
    }

    // Get job by ID
    pub fn get_job(env: Env, id: u64) -> Option<Job> {
        let jobs: Vec<Job> = env.storage().instance().get(&symbol_short!("JOBS"))
            .unwrap_or(Vec::new(&env));

        for job in jobs.iter() {
            if job.id == id {
                return Some(job);
            }
        }
        None
    }
}

mod test;