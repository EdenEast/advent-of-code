# Defines all shells for working in this repository. The `default` shell contains the base tools used by all languages.
#
# Each language can be used by its own .envrc file `use flake ./#lang` or by using the `nix develop` command
{
  perSystem = {pkgs, ...}: let
    default-packages = with pkgs; [
      aoc-cli
      just
      watchexec
      hyperfine
    ];
    rustToolchain = pkgs.rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "rust-analyzer"
      ];
    };
  in {
    devShells = {
      default = pkgs.mkShell {
        name = "aoc-default";
        packages = default-packages;
      };
      lua = pkgs.mkShell {
        name = "aoc-lua";
        packages = with pkgs;
          [
            luajit
            luau
            stylua
            lua-language-server
          ]
          ++ default-packages;
      };
      odin = pkgs.mkShell {
        name = "aoc-odin";
        packages = with pkgs;
          [
            odin
            ols
            odinfmt
          ]
          ++ default-packages;
      };
      rust = pkgs.mkShell {
        name = "aoc-rust";
        nativeBuildInputs = [rustToolchain];
        packages = default-packages;
      };
      zig = pkgs.mkShell {
        name = "aoc-zig";
        packages = with pkgs;
          [
            zig
            zls
          ]
          ++ default-packages;
      };
    };
  };
}
