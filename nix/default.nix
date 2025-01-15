{
  imports = [
    # Sets additional / modified arguments passed to flake-parts module
    ./args.nix

    # ./checks.nix

    # Sets development shells for the flake output
    ./devshell.nix

    # ./packages.nix

    # Formatter
    ./formatter.nix
  ];
}
