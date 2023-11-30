{
  config,
  lib,
  pkgs,
  ...
}: let
  inherit
    (lib)
    mkOption
    types
    mdDoc
    ;
in {
  options = {
    static = {
      derivations = mkOption {
        type = types.attrsOf types.package;
        default = {};
      };

      result = mkOption {
        type = types.package;
        readOnly = true;
      };
    };
  };

  config.static = {
    result = pkgs.buildEnv {
      name = "activation-manager-static";
      paths = builtins.attrValues config.static.derivations;
    };
  };
}
