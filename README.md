# yew-app

## Rust Install
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Versions
```sh
rustc --version
rustup --version
cargo --version
```

## Rust Update
```sh
rustup update
```

## Trunk Install
```sh
cargo install trunk
```

## WASM Install
```sh
rustup target add wasm32-unknown-unknown
```

## Trunk Server Start
```sh
trunk serve --port=80 --open
```

## Rust Uninstall
```sh
rustup self uninstall
```