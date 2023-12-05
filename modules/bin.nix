{
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib) mkOption mdDoc types;
in {
  options.bin = {
    activation-manager = mkOption {
      type = types.package;
    };
  };

  config.bin = {
    activation-manager = pkgs.python3.pkgs.callPackage ../package.nix {};
  };
}
