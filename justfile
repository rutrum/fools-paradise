cart-name := "cart"

default: watch

release-build:
    cargo build --release
    cp target/wasm32-unknown-unknown/release/{{cart-name}}.wasm target/release.wasm
    wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o target/release.wasm target/release.wasm
    wasm-strip target/release.wasm
    wasm-opt -Oz --strip-producers --dce --zero-filled-memory -o target/release.wasm target/release.wasm

bundle: release-build
    w4 bundle target/release.wasm --html target/{{cart-name}}.html

watch:
    w4 watch --no-qr &
    watchexec -cr -d 0 -i src/sprite_consts.rs -- just img

tree:
    tree -I "target|out"

img:
    w4 png2src --template graphics/template.rs --rs graphics/out/*.png > src/sprite_consts.rs
