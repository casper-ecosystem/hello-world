PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rust-src --toolchain ${PINNED_TOOLCHAIN}

build-contract:
	cd contract && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	wasm-strip ./target/wasm32-unknown-unknown/release/contract.wasm

test: build-contract
	mkdir -p tests/wasm
	cp ./target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
