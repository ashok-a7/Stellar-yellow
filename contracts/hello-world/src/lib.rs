#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Vec};

#[derive(Clone)]
#[contract]
pub struct JobBoard;

// Structure representing a job listing
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
    // Post a new job to the board
    pub fn post_job(env: Env, id: u64, title: String, description: String, employer: String) {
        let mut jobs: Vec<Job> = env
            .storage()
            .instance()
            .get(&symbol_short!("JOBS"))
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

    // Return all available job listings
    pub fn get_jobs(env: Env) -> Vec<Job> {
        env.storage()
            .instance()
            .get(&symbol_short!("JOBS"))
            .unwrap_or(Vec::new(&env))
    }

    // Find a job using its unique ID
    pub fn get_job(env: Env, id: u64) -> Option<Job> {
        let jobs: Vec<Job> = env
            .storage()
            .instance()
            .get(&symbol_short!("JOBS"))
            .unwrap_or(Vec::new(&env));

        for job in jobs.iter() {
            if job.id == id {
                return Some(job);
            }
        }

        None
    }
}

// Unit tests
mod test;
