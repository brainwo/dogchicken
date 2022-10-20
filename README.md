A game made for Rusty Jam #2

## Developing

### Build for WASM

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

### Run locally

This will build the project and copy the `.wasm` file to `/docs`.

```
./build.sh; basic-http-server docs
```

---

#### Questions my future self might ask:

> **Why the released web root on `/docs`?**

Because the GitHub Pages only supports either root or `/docs` folder, or I just don't know how.
