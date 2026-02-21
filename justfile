cart-name := "cart"
out-name := "fools_paradise"
sprite_out_file := "src/sprite/sprite_data.rs"

default: watch

release-build: img
    cargo build --release
    cp target/wasm32-unknown-unknown/release/{{cart-name}}.wasm target/release.wasm
    # wasm-snip not in nixpkgs - skipping for now
    # wasm-snip --snip-rust-panicking-code -o target/release.wasm target/release.wasm
    wasm-strip target/release.wasm
    wasm-opt -Oz --strip-producers --dce --zero-filled-memory --enable-bulk-memory --enable-sign-ext --enable-mutable-globals --enable-nontrapping-float-to-int -o target/release.wasm target/release.wasm

bundle: release-build
    w4 bundle target/release.wasm --html target/{{out-name}}.html --linux target/{{out-name}}.linux --title "Fool's Paradise" --icon-file graphics/out/ship1.png

watch:
    w4 watch --no-qr &
    watchexec --clear=reset --debounce 0ms -i {{sprite_out_file}} -- just img

tree:
    tree -I "target|out"

img:
    w4 png2src --template graphics/template.rs --rs graphics/out/*.png > {{sprite_out_file}}

cart-size: release-build
    ls target/release.wasm -lh | cut -d' ' -f 5

doc:
    cargo doc --document-private-items

clean:
    cargo clean

linux: bundle
    target/{{out-name}}.linux
