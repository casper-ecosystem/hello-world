# Hello World Smart Contract

The steps below are a quick start if you have already set up the [nctl](https://github.com/CasperLabs/casper-node/tree/master/utils/nctl) testing tool. 

## Install

### Set up the Rust toolchain
You need the Rust toolchain to develop smart contracts.
```bash
$ make prepare
```

### Compile smart contracts
Compile WASM files that will be used later.
```bash
$ make build-contracts
```

### Test
Execute tests.
```bash
$ make test
```
