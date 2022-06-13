# Hello World Session Code

This session code accepts a message string and stores it in the calling account under the `special_value` NamedKey.

**Usage**: This session code expects a runtime argument named `message` of type string.

**Tests**: There are two tests available to test the Hello World session code. The `should_store_hello_world` test verifies the happy path, where a string *hello world* is saved under the `special_value` NamedKey. The `should_error_on_missing_runtime_arg` test verifies that an error is displayed when the runtime argument is missing.

## Build and Test the Session Code 

### Set up the Rust toolchain
You need the Rust toolchain to develop smart contracts.
```bash
make prepare
```

### Compile smart contracts
Compile Wasm files that will be used to deploy the session code.
```bash
make build-contract
```

### Test
Run the tests suite.
```bash
make test
```

## Deploy the Hello World Session Code

You can deploy the *hello world* session code on a local network using NCTL. For more information on how to run an NCTL network, see [Setting up an NCTL network](https://docs.casperlabs.io/dapp-dev-guide/building-dapps/setup-nctl/).

This command provides a view into the faucet account details. The faucet is the default account created on the NCTL network.

```bash
nctl-view-faucet-account
```

The following command will help you deploy the session code on the NCTL network. In the following command, the KEY PATH path is the path of the faucet account secret key.

```bash
casper-client put-deploy \
    --node-address http://localhost:11101 \
    --chain-name casper-net-1 \
    --secret-key [KEY PATH]/secret_key.pem \
    --payment-amount 5000000000000 \
    --session-path [CONTRACT PATH]/contract.wasm     
```

After the deploy is successful, you can view the changes in the faucet account details.  

