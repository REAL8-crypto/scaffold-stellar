use crate::{error::Error, name::is_valid, SorobanContract__Client as SorobanContractClient};
use assert_matches::assert_matches;
use loam_sdk::soroban_sdk::{
    self, env, set_env,
    testutils::{Address as _, BytesN as _},
    Address, Bytes, BytesN, Env, IntoVal,
};
use soroban_sdk::String as SorobanString;

extern crate std;

fn to_string(env: &Env, s: &str) -> SorobanString {
    SorobanString::from_str(env, s)
}

fn default_version(env: &Env) -> soroban_sdk::String {
    to_string(env, "0.0.0")
}

// stellar_registry::import_contract_client!(registry);
// Equivalent to:

mod registry {
    use super::soroban_sdk;
    soroban_sdk::contractimport!(file = "../../../target/stellar/registry.wasm");
}

fn init() -> (SorobanContractClient<'static>, Address) {
    set_env(Env::default());
    let env = env();
    let contract_id = Address::generate(env);
    let address = Address::generate(env);
    let client = SorobanContractClient::new(
        env,
        &env.register_at(&contract_id, registry::WASM, (address.clone(),)),
    );
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

    let wasm_hash = env.deployer().upload_contract_wasm(registry::WASM);

    assert_matches!(
        client.try_fetch_hash(name, &None).unwrap_err(),
        Ok(Error::NoSuchContractPublished)
    );

    let bytes = Bytes::from_slice(env, registry::WASM);
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
    // let other_address = Address::generate(env);
    // let res = client
    //     .try_publish(name, &other_address, &bytes, &None, &None)
    //     .unwrap_err();

    // assert!(matches!(res, Ok(Error::AlreadyPublished)));
}

#[test]
fn returns_most_recent_version() {
    let (client, address) = &init();
    let env = env();
    let name = &to_string(env, "publisher");
    // client.register_name(address, name);
    let bytes = Bytes::from_slice(env, registry::WASM);
    env.mock_all_auths();
    let version = default_version(env);
    client.publish(name, address, &bytes, &version);
    let fetched_hash = client.fetch_hash(name, &None);
    let wasm_hash = env.deployer().upload_contract_wasm(registry::WASM);
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
    let bytes = &Bytes::from_slice(env, registry::WASM);
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