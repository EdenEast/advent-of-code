# Provides the default formatter for 'nix fmt'
# https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-fmt
{inputs, ...}: {
  imports = with inputs; [
    treefmt-nix.flakeModule
    flake-root.flakeModule
  ];
  perSystem = {
    config,
    pkgs,
    ...
  }: {
    treefmt = {
      config = {
        package = pkgs.treefmt;

        inherit (config.flake-root) projectRootFile;
        programs = {
          alejandra.enable = true; # nix formatter https://github.com/kamadorueda/alejandra/blob/main/STYLE.md
          deadnix.enable = true; # removes dead nix code https://github.com/astro/deadnix
          just.enable = true; # wrapper around `just --fmt`
          odinfmt.enable = true; # oden formatter using the lsp https://github.com/DanielGavin/ols
          rustfmt.enable = true; # rust formatter https://github.com/rust-lang/rustfmt
          statix.enable = true; # prevents use of nix anti-patterns https://github.com/nerdypepper/statix
          stylua.enable = true; # lua formatter https://github.com/JohnnyMorganz/StyLua
          zig.enable = true; # wrapper around `zig fmt`
        };
      };
    };
    formatter = config.treefmt.build.wrapper;
  };
}
