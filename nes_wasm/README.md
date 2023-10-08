# nes_wasm

## Build

Add WASM target:

```sh
rustup target add wasm32-unknown-unknown
```

Install wasm-pack:

```sh
cargo install wasm-pack
```

Build:

```sh
wasm-pack build --release --target web
cd .. && python3 -m http.server 8080 --directory ./
```
