# nes_yew

## Build

Add WASM target:

```sh
rustup target add wasm32-unknown-unknown
```

Install Trunk ( https://trunkrs.dev/#install ):

```sh
cargo install --locked trunk
```

Build:

```sh
trunk build --release
```

## Develop

Hot reload:

```sh
trunk serve
```
