{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    utils.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, fenix, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system} // { inherit (fenix.packages.${system}.stable) cargo rustc rust-src clippy rustfmt; }; in
      rec {
        devShell = pkgs.mkShell {
          name = "aoc";
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
          ] ++ (lib.optionals pkgs.stdenv.isDarwin [ libiconv ]);

          CARGO_BUILD_RUSTFLAGS = if pkgs.stdenv.isDarwin then "-C rpath" else null;
          RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
        };
      });
}
