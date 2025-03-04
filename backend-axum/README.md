## Quick Start

### dev
```sh
cargo run
```

### produce
```sh
cargo build --release
cp ./target/release/backend-axum ./main
RUST_LOG=info ./main
```
