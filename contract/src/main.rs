#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use casper_contract::contract_api::{runtime, storage};
use casper_types::{Key, URef};

const RUNTIME_ARG_KEY_NAME: &str = "key-name";
const RUNTIME_ARG_MESSAGE: &str = "message";

fn store(value: String, key_name: String) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Remove the key if already exists in account entity context
    runtime::remove_key(&key_name);

    // Store this key under the name "key-name" in caller context
    runtime::put_key(&key_name, value_key);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // This contract expectstwo runtime arguments to be provided.
    // The arg is named "message"and will be of type `String`.
    let message: String = runtime::get_named_arg(RUNTIME_ARG_MESSAGE);
    // The arg is named "key-name" and will be of type `String`.
    let key_name: String = runtime::get_named_arg(RUNTIME_ARG_KEY_NAME);
    store(message, key_name);
}
