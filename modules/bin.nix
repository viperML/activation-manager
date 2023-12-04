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
    activation-manager = pkgs.perlPackages.buildPerlPackage {
      pname = "activation-manager";
      version = "0";

      src = lib.cleanSource ../.;
      outputs = ["out"];
    };
  };
}
