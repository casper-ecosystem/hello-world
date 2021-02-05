use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, RuntimeArgs, U512,
    PublicKey
};

pub const HELLO_WORLD: &str = "hello_world";
pub const HELLO_WORLD_HASH: &str = "hello_world_hash";

pub struct HelloWorldContract {
    pub context: TestContext,
    pub hello_world_hash: Hash,
    pub test_account: PublicKey
}

impl HelloWorldContract {
    pub fn deploy() -> Self {
        let test_account: PublicKey = PublicKey::ed25519([3u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(test_account, test_account.to_account_hash(), U512::from(500_000_000_000_000_000u64))
            .build();
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(test_account.to_account_hash())
            .with_authorization_keys(&[test_account.to_account_hash()])
            .build();
        context.run(session);
        let hello_world_hash = Self::contract_hash(&context, test_account, HELLO_WORLD_HASH);
        Self {
            context,
            hello_world_hash,
            test_account
        }
    }

    pub fn contract_hash(context: &TestContext, account: PublicKey, name: &str) -> Hash {
        context
            .query(account.to_account_hash(), &[name.to_string()])
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
            .with_address(self.test_account.to_account_hash())
            .with_authorization_keys(&[self.test_account.to_account_hash()])
            .build();
        self.context.run(session);
    }
    pub fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.test_account.to_account_hash(), &[HELLO_WORLD.to_string(), name.to_string()])
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
