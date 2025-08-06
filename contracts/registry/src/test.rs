use crate::{error::Error, name::is_valid, SorobanContract__, SorobanContract__Client};
use assert_matches::assert_matches;
use loam_sdk::soroban_sdk::{
    self, env, set_env,
    testutils::{Address as _, BytesN as _},
    Address, Bytes, BytesN, Env, IntoVal,
};
use soroban_sdk::String as SorobanString;

extern crate std;

// Use `include_bytes!` to get the WASM without name collisions.
const REGISTRY_WASM: &[u8] = include_bytes!("../../../target/stellar/registry.wasm");

fn to_string(env: &Env, s: &str) -> SorobanString {
    SorobanString::from_str(env, s)
}

fn default_version(env: &Env) -> soroban_sdk::String {
    to_string(env, "0.0.0")
}

// Initialize the LOCAL contract, not the imported one.
fn init() -> (SorobanContract__Client<'static>, Address) {
    set_env(Env::default());
    let env = env();
    // Register the local contract type, not the wasm.
    let address = Address::generate(env);
    let contract_id = env.register(SorobanContract__, (address.clone(),));
    let client = SorobanContract__Client::new(env, &contract_id);
    // Initialize the admin, which is required by the contract.
    // client.admin_set(&address);  <-- Remove this line, since admin is initialized during contract registration.
    (client, address)
}

#[test]
fn handle_error_cases() {
    let (client, address) = &init();
    let env = env();

    let name = &to_string(env, "publisher");
    assert_matches!(
        client.try_fetch_hash(name, &None).unwrap_err(),
        Ok(Error::NoSuchContractPublished)
    );

    // Use the new REGISTRY_WASM constant
    let wasm_hash = env.deployer().upload_contract_wasm(REGISTRY_WASM);

    assert_matches!(
        client.try_fetch_hash(name, &None).unwrap_err(),
        Ok(Error::NoSuchContractPublished)
    );

    let bytes = Bytes::from_slice(env, REGISTRY_WASM);
    env.mock_all_auths();
    let version = default_version(env);
    client.publish(name, address, &bytes, &version);
    assert_eq!(client.fetch_hash(name, &None), wasm_hash);

    assert_matches!(
        client
            .try_fetch_hash(name, &Some(to_string(env, "0.0.1")))
            .unwrap_err(),
        Ok(Error::NoSuchVersion)
    );
}

#[test]
fn returns_most_recent_version() {
    let (client, address) = &init();
    let env = env();
    let name = &to_string(env, "publisher");
    let bytes = Bytes::from_slice(env, REGISTRY_WASM);
    env.mock_all_auths();
    let version = default_version(env);
    client.publish(name, address, &bytes, &version);
    let fetched_hash = client.fetch_hash(name, &None);
    let wasm_hash = env.deployer().upload_contract_wasm(REGISTRY_WASM);
    assert_eq!(fetched_hash, wasm_hash);

    let second_hash: BytesN<32> = BytesN::random(env);
    client.publish_hash(
        name,
        address,
        &second_hash.into_val(env),
        &to_string(env, "0.0.1"),
    );
    let res = client.fetch_hash(name, &None);
    assert_eq!(res, second_hash);
}

fn test_string(s: &str, result: bool) {
    let env = env();
    assert!(
        is_valid(&to_string(env, s)) == result,
        "{s} should be {}valid",
        if result { "" } else { "in" }
    );
}

#[test]
fn validate_names() {
    set_env(Env::default());
    test_string("publish", true);
    test_string("a_a_b", true);
    test_string("abcdefghabcdefgh", true);
    test_string("abcdefghabcdefghabcdefghabcdefgh", true);
    test_string(
        "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgh",
        true,
    );
    test_string(
        "abcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefghabcdefgha",
        false,
    );
    test_string("a-a_b", true);
    test_string("a-a]]]_b", false);
    test_string("_ab", false);
    test_string("-ab", false);
    test_string("1ab", false);
}

#[test]
fn validate_version() {
    let (client, address) = &init();
    let env = env();
    let name = &to_string(env, "registry");
    let bytes = &Bytes::from_slice(env, REGISTRY_WASM);
    env.mock_all_auths();
    let version = &to_string(env, "0.0.0");
    let new_version = &to_string(env, "0.0.1");
    client.publish(name, address, bytes, version);
    assert_eq!(
        client.try_publish(name, address, bytes, version),
        Err(Ok(Error::VersionMustBeGreaterThanCurrent))
    );
    assert_eq!(
        client.try_publish(name, address, bytes, &to_string(env, "0.  0.0"),),
        Err(Ok(Error::InvalidVersion))
    );
    client.publish(name, address, bytes, new_version);
    assert_eq!(
        client.try_publish(name, address, bytes, version),
        Err(Ok(Error::VersionMustBeGreaterThanCurrent))
    );
}
