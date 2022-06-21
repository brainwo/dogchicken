cargo build --target wasm32-unknown-unknown --release
cp  -f ./target/wasm32-unknown-unknown/release/dogchicken.wasm ./www/dogchicken.wasm
