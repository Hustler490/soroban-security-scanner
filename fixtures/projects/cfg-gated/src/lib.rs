// Fixture: production code and `#[cfg(test)]` code in one contract.
use soroban_sdk::{contractimpl, Env};

#[contractimpl]
impl Vault {
    pub fn shipped(env: Env) {
        env.storage().persistent().set(&0u32, &1u32);
    }

    #[cfg(test)]
    pub fn test_only(env: Env) {
        env.storage().persistent().set(&0u32, &2u32);
    }
}
