use casperlabs_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casperlabs_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, RuntimeArgs, U512,
};

pub const TEST_ACCOUNT: AccountHash = AccountHash::new([7u8; 32]);
pub const HELLO_WORLD: &str = "hello_world";
pub const HELLO_WORLD_HASH: &str = "hello_world_hash";

pub struct HelloWorldContract {
    pub context: TestContext,
    pub hello_world_hash: Hash,
}

impl HelloWorldContract {
    pub fn deploy() -> Self {
        let mut context = TestContextBuilder::new()
            .with_account(TEST_ACCOUNT, U512::from(128_000_000))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(TEST_ACCOUNT)
            .with_authorization_keys(&[TEST_ACCOUNT])
            .build();
        context.run(session);
        let hello_world_hash = Self::contract_hash(&context, HELLO_WORLD_HASH);
        Self {
            context,
            hello_world_hash,
        }
    }

    pub fn contract_hash(context: &TestContext, name: &str) -> Hash {
        context
            .query(TEST_ACCOUNT, &[name])
            .unwrap_or_else(|_| panic!("{} contract not found", name))
            .into_t()
            .unwrap_or_else(|_| panic!("{} is not type Contract", name))
    }

    pub fn call_update(&mut self, value: String) {
        let code = Code::Hash(self.hello_world_hash, "update".to_string());
        let args = runtime_args! {
            "value" => value
        };
        let session = SessionBuilder::new(code, args)
            .with_address(TEST_ACCOUNT)
            .with_authorization_keys(&[TEST_ACCOUNT])
            .build();
        self.context.run(session);
    }
    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(TEST_ACCOUNT, &[HELLO_WORLD, &name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }
}
