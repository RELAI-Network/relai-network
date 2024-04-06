# Relai Network Node

The Relai Network substrate codebase.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```


### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/relai-network --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/relai-network purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/relai-network -ldebug --dev
```

### Test

To run unit test for `futur-assets-reg` and `futur-creators-reg` pallets

```
cargo test -p futur-assets-reg 
```
and 
```
cargo test -p futur-creatorss-reg 
```
Or just run `cargo test` for all tests
