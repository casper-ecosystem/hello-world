#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use casper_contract::contract_api::{runtime, storage};
use casper_types::{Key, URef};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

fn store(value: String) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "my-key-name" in caller context
    runtime::put_key(KEY_NAME, value_key);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // This contract expects a single runtime argument to be provided.  The arg is named "message"
    // and will be of type `String`.
    let value: String = runtime::get_named_arg(RUNTIME_ARG_NAME);
    store(value);
}
