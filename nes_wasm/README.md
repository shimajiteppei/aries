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

Develop:

```sh
wasm-pack build --target web --no-pack --no-typescript
cd .. && python3 -m http.server 8080 --directory ./
```

Build:

Do commit the build products for distribution with jsdelivr.

```sh
wasm-pack build --release --target web --no-pack --no-typescript
```
