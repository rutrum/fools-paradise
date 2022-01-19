cart-name := "cart"
out-name := "fools_paradise"
sprite_out_file := "src/sprite/sprite_data.rs"

default: watch

release-build: img
    cargo build --release
    cp target/wasm32-unknown-unknown/release/{{cart-name}}.wasm target/release.wasm
    # wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o target/release.wasm target/release.wasm
    wasm-snip --snip-rust-panicking-code -o target/release.wasm target/release.wasm
    wasm-strip target/release.wasm
    wasm-opt -Oz --strip-producers --dce --zero-filled-memory -o target/release.wasm target/release.wasm

bundle: release-build
    w4 bundle target/release.wasm --html target/{{out-name}}.html --windows target/{{out-name}}.exe --linux target/{{out-name}}.linux --mac target/{{out-name}}.mac --title "Fool's Paradise" --icon-file graphics/out/ship1.png

watch:
    w4 watch --no-qr &
    watchexec -cr -d 0 -i {{sprite_out_file}} -- just img

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
