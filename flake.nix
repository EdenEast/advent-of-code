{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-root.url = "github:srid/flake-root";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      debug = true;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      imports = [
        ./nix
      ];
    };
}

# References:
# - https://sourcegraph.com/github.com/nix-community/vulnix/-/blob/flake.nix
#   - Structure of using flake-parts with treefmt-nix
#
# - https://sourcegraph.com/github.com/informalsystems/cosmos.nix/-/blob/flake.nix
#   - Structure of using rust-overlay with flake-parts and have multiple devshells

