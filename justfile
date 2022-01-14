default: serve

release-build:
    cargo build --release
    cd target/wasm32-unknown-unknown/release
    wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o target/release.wasm target/release.wasm
    wasm-strip target/release.wasm
    wasm-opt -Oz --strip-producers --dce --zero-filled-memory -o target/release.wasm target/release.wasm

serve:
    w4 watch
