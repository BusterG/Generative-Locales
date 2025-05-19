#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol};

pub struct HelloCounter;

#[contractimpl]
impl HelloCounter {
    pub fn increment(env: Env) -> u32 {
        let key = Symbol::new(&env, "count");
        let count = env.storage().get::<Symbol, u32>(&key).unwrap_or(0);
        let new_count = count + 1;
        env.storage().set(&key, &new_count);
        new_count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().get::<Symbol, u32>(&Symbol::new(&env, "count")).unwrap_or(0)
    }
}
