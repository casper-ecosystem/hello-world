fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use casper_engine_test_support::{
        utils::create_run_genesis_request, DeployItemBuilder, ExecuteRequestBuilder,
        LmdbWasmTestBuilder, ARG_AMOUNT, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_PUBLIC_KEY,
        DEFAULT_PAYMENT,
    };
    use casper_execution_engine::{engine_state::Error as CoreError, execution::ExecError};

    use casper_types::{runtime_args, ApiError, GenesisAccount, Key, Motes, RuntimeArgs, U512};

    // Define `KEY_NAME` constant to match that in the contract.
    const KEY_NAME: &str = "my-key-name";
    const VALUE: &str = "hello world";
    const RUNTIME_ARG_NAME: &str = "message";
    const CONTRACT_WASM: &str = "contract.wasm";

    #[test]
    fn should_store_hello_world() {
        let mut builder = LmdbWasmTestBuilder::default();
        builder
            .run_genesis(create_run_genesis_request(vec![GenesisAccount::Account {
                public_key: DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
                balance: Motes::new(U512::from(5_000_000_000_000_u64)),
                validator: None,
            }]))
            .commit();

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = runtime_args! {
            RUNTIME_ARG_NAME => VALUE,
        };

        let deploy_item = DeployItemBuilder::new()
            .with_standard_payment(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(&deploy_item).build();

        // prepare assertions.
        let result_of_query = builder.query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[KEY_NAME.to_string()],
        );
        assert!(result_of_query.is_err());

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();

        // make assertions
        let result_of_query = builder
            .query(
                None,
                Key::Account(*DEFAULT_ACCOUNT_ADDR),
                &[KEY_NAME.to_string()],
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        assert_eq!(result_of_query, VALUE);
    }

    #[test]
    fn should_error_on_missing_runtime_arg() {
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = RuntimeArgs::new();

        let deploy_item = DeployItemBuilder::new()
            .with_standard_payment(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_session_code(session_code, session_args)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(&deploy_item).build();

        let mut builder = LmdbWasmTestBuilder::default();
        builder
            .run_genesis(create_run_genesis_request(vec![GenesisAccount::Account {
                public_key: DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
                balance: Motes::new(U512::from(5_000_000_000_000_u64)),
                validator: None,
            }]))
            .commit();
        builder.exec(execute_request).commit().expect_failure();

        let actual_error = builder.get_error().expect("must have error");

        assert!(
            matches!(
                actual_error,
                CoreError::Exec(ExecError::Revert(ApiError::MissingArgument))
            ),
            "Expected {:?}, received {:?}",
            CoreError::Exec(ExecError::Revert(ApiError::MissingArgument)),
            actual_error
        );
    }
}
