# WASM

## Referred

https://github.com/bevyengine/bevy/tree/latest/examples#examples

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

## Dependencies

No dynamic_linking:

```toml
[dependencies]
# bevy = { version = "0.13.0", features = ["dynamic_linking"] }
# FOR WASM BUILD:
bevy = "0.13.0"
```

and add [profile.wasm-release]

## Build

```sh
cargo build --profile wasm-release --target wasm32-unknown-unknown
# or
# cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-name wasm_bevy_clock \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/bevy_clock.wasm
```

## Copy assets

Copy asset files to wasm directory.

## Run

Then serve `wasm` directory to browser. i.e.

```sh
# cargo install basic-http-server
basic-http-server wasm
```
