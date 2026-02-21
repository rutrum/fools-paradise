{
  description = "Fool's Paradise - a WASM-4 game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    wasm4-nix.url = "path:/home/rutrum/repo/wasm4-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, wasm4-nix, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        # Rust with wasm32 target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain with wasm target
            rustToolchain

            # WASM optimization tools
            pkgs.wabt        # wasm-strip
            pkgs.binaryen    # wasm-opt
            # wasm-snip not in nixpkgs - install via: cargo install wasm-snip

            # WASM-4 CLI
            wasm4-nix.packages.${system}.wasm4

            # Build tools from justfile
            pkgs.just
            pkgs.watchexec
          ];

          shellHook = ''
            echo "Fool's Paradise dev environment"
            echo "  w4 version: $(w4 --version)"
            echo "  rustc version: $(rustc --version)"
            echo ""
            echo "Commands:"
            echo "  just watch    - Watch and rebuild"
            echo "  just bundle   - Create release bundles"
          '';
        };
      }
    );
}
