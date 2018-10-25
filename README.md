# eosio-rust
Rust smart contract for EOSIO based blockchains.
The source of eosio-rust.rs is based on Andrea Passaglia's wonderful article at:
https://www.sales4k.com/blockchain/eos-contracts-rust-no-cpp/.

The contract is deployed on EOS mainnet at: https://eosflare.io/account/rustcontract.

Use daily Rust with rustup (https://rustup.rs/).

Build with:
cargo  +nightly build --target wasm32-unknown-unknown --release --verbose

Then deploy the .wasm and .abi file. The abi of "rustcontract" account is in the root of this repo.
