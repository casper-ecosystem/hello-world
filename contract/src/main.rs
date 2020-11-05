#![no_main]

extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::convert::TryInto;

use casperlabs_contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef,
};
/// The name of the key under which the string will be stored
const KEY: &str = "special_value";
/// The value of the string to be stored
const ARG_MESSAGE: &str = "hello world";

#[casperlabs_contract]
mod hello_world {

    #[casperlabs_constructor]
    fn init() {
        let value = String::from(ARG_MESSAGE);
        set_key(KEY, value);
    }

    #[casperlabs_method]
    fn update(value: String) {
        set_key(KEY, value)
    }

    fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                storage::write(key_ref, value);
            }
            None => {
                let key = storage::new_uref(value).into();
                runtime::put_key(name, key)
            }
        }
    }
}
