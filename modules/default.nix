{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkOption
    mdDoc
    types
    ;
in {
  imports = [
    # Core
    ./dag.nix
    ./static.nix
    ./root.nix

    # Functionality
    ./path.nix
    ./bin.nix
  ];

  options = {
    manifest = mkOption {
      type = types.package;
      description = mdDoc "Resulting manifest of all entries";
    };
  };

  config = {
    manifest = pkgs.writers.writeJSON "activation-manager-manifest.json" {
      version = "0";
      inherit (config) dag root static;
    };
  };
}
