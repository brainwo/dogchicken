## Build for WASM

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Serve

```
cargo install basic-http-server
basic-http-server .
```
