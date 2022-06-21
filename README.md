## Build for WASM

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Run locally

This will build the project and copy the `.wasm` file to `/docs`.

```
./build.sh; basic-http-server docs
```
