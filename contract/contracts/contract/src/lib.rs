#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Map, Vec, Symbol
};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // 🧩 Tạo poll
    pub fn create_poll(env: Env, admin: Address, options: Vec<Symbol>) {
        admin.require_auth();

        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
        env.storage().instance().set(&symbol_short!("OPTIONS"), &options);

        let mut votes: Map<Symbol, i32> = Map::new(&env);

        for option in options.iter() {
            votes.set(option.clone(), 0);
        }

        env.storage().instance().set(&symbol_short!("VOTES"), &votes);
    }

    // 🗳️ Vote
    pub fn vote(env: Env, voter: Address, option: Symbol) {
        voter.require_auth();

        let mut votes: Map<Symbol, i32> =
            env.storage().instance().get(&symbol_short!("VOTES")).unwrap();

        let current = votes.get(option.clone()).unwrap_or(0);

        votes.set(option, current + 1);

        env.storage().instance().set(&symbol_short!("VOTES"), &votes);
    }

    // 📊 Xem kết quả
    pub fn get_results(env: Env) -> Map<Symbol, i32> {
        env.storage()
            .instance()
            .get(&symbol_short!("VOTES"))
            .unwrap()
    }
}