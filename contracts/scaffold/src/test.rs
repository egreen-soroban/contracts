use super::{Scaffold, ScaffoldClient};
use soroban_sdk::{symbol_short, Env};

#[test]
fn returns_spec_version() {
    let env = Env::default();
    let id = env.register(Scaffold, ());
    let client = ScaffoldClient::new(&env, &id);
    assert_eq!(client.spec_version(), symbol_short!("v0_1"));
}
