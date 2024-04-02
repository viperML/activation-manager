{
  lib,
  pkgs,
  config,
  ...
}:
let
  inherit (lib) mkOption types;
in
{
  options = {
    manifest = mkOption {
      type = types.package;
      description = "Resulting manifest of all entries.";
      readOnly = true;
    };

    # flavor = mkOption {
    #   type = types.enum [ "home" ];
    #   description = "Specific configuration for the task.";
    # };
  };

  config = {
    manifest = pkgs.writers.writeJSON "activation-manager-manifest.json" {
      version = "0";
      inherit (config) nodes static nodes2;
    };
  };
}
