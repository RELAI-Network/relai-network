# Relai Network Node

The Relai Network substrate codebase.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/relai-network -h
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