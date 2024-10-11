# UCS01 Cosmwasm Transfer Demo

This example demonstrates how to create a CosmWasm contract that calls Union’s UCS01 Relay to execute a cross-chain asset transfer.

Follow the guide at [docs.union.build/integrations/ucs01/cosmwasm](https://docs.union.build/integrations/ucs01/cosmwasm)

### Initial setup, ensure `wasm32-unknown-unknown` is installed

```sh
rustup target add wasm32-unknown-unknown
```

### Build the contract

```sh
cargo build --target wasm32-unknown-unknown --release
```