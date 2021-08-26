# Komodo Snapshot

A library with tools for creating snapshots on Komodo blockchains.

rust nightly is required: `rustup default nightly`. Visit [rustup.rs](https://rustup.rs) to install Rust.

## How to use

Add `komodo_snapshot` as a dependency in your Cargo.toml:

```toml
komodo_snapshot = "0.1.0"
```

Then use snapshot as follows:

```rust
extern crate komodo_snapshot;

use snapshot::Snapshot;

let snapshot = Snapshot::builder()
    .on_chain("KMD")
    .using_threshold(1.0)
    .store_in_csv("./output.csv")
    .take();
```
