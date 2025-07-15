#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        utils::create_run_genesis_request, ExecuteRequestBuilder, LmdbWasmTestBuilder,
        DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_PUBLIC_KEY,
    };
    use casper_execution_engine::{engine_state::Error as CoreError, execution::ExecError};

    use casper_types::{runtime_args, ApiError, GenesisAccount, Key, Motes, RuntimeArgs, U512};

    const RUNTIME_ARG_KEY_NAME: &str = "key-name";
    const KEY_NAME_VALUE: &str = "my-custom_key-name";
    const RUNTIME_ARG_MESSAGE: &str = "message";
    const MESSAGE_VALUE: &str = "hello world";
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

        let session_args = runtime_args! {
            RUNTIME_ARG_MESSAGE => MESSAGE_VALUE,
            RUNTIME_ARG_KEY_NAME => KEY_NAME_VALUE
        };

        let execute_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CONTRACT_WASM, session_args)
                .build();

        // prepare assertions.
        let result_of_query = builder.query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[KEY_NAME_VALUE.to_string()],
        );
        assert!(result_of_query.is_err());

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();

        // make assertions
        let result_of_query = builder
            .query(
                None,
                Key::Account(*DEFAULT_ACCOUNT_ADDR),
                &[KEY_NAME_VALUE.to_string()],
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        assert_eq!(result_of_query, MESSAGE_VALUE);
    }

    #[test]
    fn should_error_on_missing_runtime_arg() {
        let mut builder = LmdbWasmTestBuilder::default();
        builder
            .run_genesis(create_run_genesis_request(vec![GenesisAccount::Account {
                public_key: DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
                balance: Motes::new(U512::from(5_000_000_000_000_u64)),
                validator: None,
            }]))
            .commit();

        let session_args = RuntimeArgs::new();

        let execute_request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, CONTRACT_WASM, session_args)
                .build();

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
