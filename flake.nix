{
    description = "Rust development environment";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {
                    inherit system overlays;
                };
                
                rustToolchain = pkgs.rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
                };
            in
            {
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        rustToolchain
                        pkg-config
                        
                        # Development tools
                        cargo-watch
                        cargo-edit
                        cargo-audit

                        questdb
                        influxdb3
                    ];

                    shellHook = ''
                        echo "Rust development environment"
                        echo "Rust: $(rustc --version)"
                        echo "Cargo: $(cargo --version)"
                    '';
                };
            }
        );
}
