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

wasm-bindgen --out-name feel-the-time \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/feel-the-time.wasm
```

## Copy assets

Copy asset files to wasm directory.

## Use wasm-opt

### Installing

```sh
brew install binaryen
```

### Optimize

```sh
wasm-opt -Oz --output wasm/target/optimized.wasm wasm/target/feel-the-time_bg.wasm
mv wasm/target/optimized.wasm wasm/target/feel-the-time_bg.wasm
```

## Run

Then serve `wasm` directory to browser. i.e.

```sh
# cargo install basic-http-server
basic-http-server wasm
```
