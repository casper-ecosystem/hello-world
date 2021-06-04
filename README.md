# Hello World Smart Contract

Simple hello world smart contract for Casper using rust.

## Prerequisites

- [GNU Make](https://www.gnu.org/software/make/)
- [Rustup](https://rustup.rs/)
- [Specific rust toolchain](#rust-toolchain)

For example, on ubuntu 20.04

- You may want to install another rust version as your primary toolchain

```bash
sudo apt update -y -qq
sudo apt install -y wget build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
  -y \
  --default-toolchain nightly \
  --profile minimal
```

- Switch to the required toolchain for this project

```bash
rustup toolchain install "$(cat rust-toolchain)" --component clippy rustfmt
rustup override set "$(cat rust-toolchain)"
rustup show
```

## Install

### Set up the Rust toolchain

You need to target the `wasm32-unknown-unknown` platform to develop smart contracts.
Rust provides support for various platform targets. `wasm32-unknown-unknown`
is a [tier 2 platform target](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2).

More information about compiling Rust to WebAssembly and the rationale behind it
can be read in [this book](https://rustwasm.github.io/docs/book/introduction.html).

```bash
make prepare
```

### Compile smart contracts

Compile WASM files that will be used later.

```bash
make build-contract
```

### Test

Execute tests.

```bash
make test
```
