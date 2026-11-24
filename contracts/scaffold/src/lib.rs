//! Placeholder contract that keeps CI green until the tranche 1 contracts land.
//! It is removed when `identity_registry`, `rights_receipt` and `round_manager` are added.
#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Scaffold;

#[contractimpl]
impl Scaffold {
    /// Returns the repository specification version.
    pub fn spec_version(_env: Env) -> Symbol {
        symbol_short!("v0_1")
    }
}

#[cfg(test)]
mod test;
